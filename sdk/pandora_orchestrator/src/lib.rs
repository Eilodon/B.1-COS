use async_trait::async_trait;
use pandora_core::interfaces::skills::SkillModule;
use pandora_error::{PandoraError, ErrorContext};
use std::collections::HashMap;
use fnv::FnvHashMap;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
mod circuit_breaker;
use circuit_breaker::{CircuitBreakerManager, CircuitBreakerConfig, CircuitStats};
use tokio::time::sleep;
use tokio::time::timeout as tokio_timeout;
use tracing::{error, info, warn};

/// Trait định nghĩa interface cho Orchestrator để dễ dàng testing và mocking
#[async_trait]
pub trait OrchestratorTrait: Send + Sync {
    /// Xử lý một request với skill name và input
    async fn process_request(
        &self,
        skill_name: &str,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, PandoraError>;

    /// Lấy danh sách tất cả skills có sẵn
    fn get_available_skills(&self) -> Vec<String>;

    /// Kiểm tra xem skill có tồn tại không
    fn has_skill(&self, skill_name: &str) -> bool;
}

/// `SkillRegistry` chịu trách nhiệm lưu trữ và quản lý tất cả các skill có sẵn.
/// Trong tương lai, nó sẽ có khả năng tự động khám phá các skill.
pub struct SkillRegistry {
    skills: FnvHashMap<String, Arc<dyn SkillModule>>,
}

impl SkillRegistry {
    pub fn new() -> Self {
        Self { skills: FnvHashMap::default() }
    }

    /// Đăng ký một skill mới vào registry.
    pub fn register(&mut self, skill: Arc<dyn SkillModule>) {
        let name = skill.descriptor().name;
        self.skills.insert(name, skill);
    }

    /// Tìm một skill dựa trên tên của nó.
    pub fn get_skill(&self, name: &str) -> Option<&Arc<dyn SkillModule>> {
        self.skills.get(name)
    }

    /// Danh sách tên tất cả các skill.
    pub fn list_skill_names(&self) -> Vec<String> {
        self.skills.keys().cloned().collect()
    }
}

/// `Orchestrator` là bộ não trung tâm, điều phối luồng nhận thức và thực thi các skill.
pub struct Orchestrator {
    registry: Arc<SkillRegistry>,
    retry_policy: RetryPolicy,
    timeout_policy: TimeoutPolicy,
    circuit_breaker: Arc<CircuitBreakerManager>,
}

impl Orchestrator {
    pub fn new(registry: Arc<SkillRegistry>) -> Self {
        Self::with_config(registry, CircuitBreakerConfig::default())
    }

    /// Start background cleanup task
    pub fn start_cleanup_task(self: Arc<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(300)); // Every 5 min
            loop {
                interval.tick().await;
                self.circuit_breaker.cleanup_expired();
                let stats = self.circuit_breaker.stats();
                info!(
                    "Circuit breaker stats: {} total ({} closed, {} open, {} half-open)",
                    stats.total_circuits, stats.closed, stats.open, stats.half_open
                );
            }
        })
    }

    /// Get circuit breaker statistics
    pub fn circuit_stats(&self) -> CircuitStats {
        self.circuit_breaker.stats()
    }

    pub fn with_config(
        registry: Arc<SkillRegistry>,
        circuit_config: CircuitBreakerConfig,
    ) -> Self {
        info!("Orchestrator initialized with {} skills", registry.skills.len());
        Self {
            registry,
            retry_policy: RetryPolicy::default(),
            timeout_policy: TimeoutPolicy::default(),
            circuit_breaker: Arc::new(CircuitBreakerManager::new(circuit_config)),
        }
    }

    /// Cấu hình các policy v2 theo builder-style.
    pub fn with_retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = policy;
        self
    }
    pub fn with_timeout_policy(mut self, policy: TimeoutPolicy) -> Self {
        self.timeout_policy = policy;
        self
    }
    // removed with_circuit_policy; now use with_config to set circuit config

    /// Xử lý với routing policy (v2). Primary + fallbacks, theo retry/backoff/circuit/timeout.
    pub async fn process_with_routing(
        &self,
        routing: RoutingPolicy,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, PandoraError> {
        let candidates = std::iter::once(routing.primary)
            .chain(routing.fallbacks.into_iter())
            .collect::<Vec<_>>();

        let mut last_err: Option<PandoraError> = None;
        for skill_name in candidates {
            match self.try_execute_with_policies(&skill_name, &input).await {
                Ok(val) => return Ok(val),
                Err(e) => {
                    warn!("Policy attempt failed for skill '{}': {}", skill_name, e);
                    last_err = Some(e);
                    // Nếu lỗi do circuit open hoặc timeout, thử fallback tiếp theo
                    continue;
                }
            }
        }
        Err(last_err.unwrap_or(PandoraError::Unknown("Routing exhausted".into())))
    }

    async fn try_execute_with_policies(
        &self,
        skill_name: &str,
        input: &serde_json::Value,
    ) -> Result<serde_json::Value, PandoraError> {
        // Circuit breaker gate
        if self.is_circuit_open(skill_name) {
            warn!("Circuit open for skill '{}'", skill_name);
            return Err(PandoraError::CircuitOpen { resource: skill_name.to_string() });
        }

        // Retry with exponential backoff and timeout per attempt
        let mut attempt: u32 = 0;
        let max_attempts = self.retry_policy.max_retries.saturating_add(1); // first try + retries
        let mut last_err: Option<PandoraError> = None;

        while attempt < max_attempts {
            if attempt > 0 {
                let backoff = self.retry_policy.backoff_duration(attempt);
                sleep(backoff).await;
            }

            let result = self.execute_with_timeout(skill_name, input.clone()).await;
            match result {
                Ok(val) => {
                    self.record_success(skill_name);
                    return Ok(val);
                }
                Err(err) => {
                    self.record_failure(skill_name);
                    // Không retry nếu lỗi không retryable
                    if !err.is_retryable() {
                        return Err(err);
                    }
                    last_err = Some(err);
                    attempt += 1;
                    continue;
                }
            }
        }

        Err(last_err.unwrap_or(PandoraError::Unknown("Retry loop exited without error".into())))
    }

    async fn execute_with_timeout(
        &self,
        skill_name: &str,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, PandoraError> {
        let skill = self
            .registry
            .get_skill(skill_name)
            .ok_or_else(|| PandoraError::SkillNotFound { skill_name: skill_name.to_string() })?;

        let timeout_ms = self.timeout_policy.timeout_ms;
        let exec_fut = skill.execute(input);
        match tokio_timeout(Duration::from_millis(timeout_ms), exec_fut).await {
            Ok(res) => res,
            Err(_) => Err(PandoraError::Timeout { operation: format!("skill:{}", skill_name), timeout_ms }),
        }
    }

    fn is_circuit_open(&self, skill_name: &str) -> bool {
        self.circuit_breaker.is_open(skill_name)
    }

    fn record_success(&self, skill_name: &str) {
        self.circuit_breaker.record_success(skill_name);
    }

    fn record_failure(&self, skill_name: &str) {
        self.circuit_breaker.record_failure(skill_name);
    }
}

#[async_trait]
impl OrchestratorTrait for Orchestrator {
    async fn process_request(
        &self,
        skill_name: &str,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, PandoraError> {
        info!("Orchestrator: Nhận yêu cầu cho skill '{}'", skill_name);
        // v2: sử dụng routing mặc định (không có fallback)
        let routing = RoutingPolicy {
            primary: skill_name.to_string(),
            fallbacks: vec![],
        };
        let output = self.process_with_routing(routing, input).await?;
        info!(
            "Orchestrator: Skill '{}' đã thực thi thành công.",
            skill_name
        );
        Ok(output)
    }

    fn get_available_skills(&self) -> Vec<String> {
        self.registry.skills.keys().cloned().collect()
    }

    fn has_skill(&self, skill_name: &str) -> bool {
        self.registry.skills.contains_key(skill_name)
    }
}

// =========================
// Policy definitions (v2)
// =========================

#[derive(Clone, Debug)]
pub struct RoutingPolicy {
    pub primary: String,
    pub fallbacks: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub initial_backoff_ms: u64,
    pub backoff_multiplier: f64,
    pub max_backoff_ms: u64,
    pub jitter_ms: u64,
}

impl RetryPolicy {
    fn backoff_duration(&self, attempt: u32) -> Duration {
        // attempt >= 1 when called here
        let exp = self.backoff_multiplier.powi(attempt as i32 - 1);
        let mut ms = (self.initial_backoff_ms as f64 * exp) as u64;
        if ms > self.max_backoff_ms {
            ms = self.max_backoff_ms;
        }
        if self.jitter_ms > 0 {
            let jitter = (fastrand::u64(..=self.jitter_ms)) as i64 - (self.jitter_ms as i64 / 2);
            let adj = (ms as i64 + jitter).max(0) as u64;
            Duration::from_millis(adj)
        } else {
            Duration::from_millis(ms)
        }
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 2,
            initial_backoff_ms: 50,
            backoff_multiplier: 2.0,
            max_backoff_ms: 1_000,
            jitter_ms: 25,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TimeoutPolicy {
    pub timeout_ms: u64,
}

impl Default for TimeoutPolicy {
    fn default() -> Self {
        Self { timeout_ms: 2_000 }
    }
}

#[derive(Clone, Debug)]
pub struct CircuitBreakerPolicy {
    pub failure_threshold: u32,
    pub open_cooldown_ms: u64,
    pub half_open_trial: u32,
}

impl Default for CircuitBreakerPolicy {
    fn default() -> Self {
        Self {
            failure_threshold: 3,
            open_cooldown_ms: 5_000,
            half_open_trial: 1,
        }
    }
}

#[derive(Debug)]
enum CircuitState {
    Closed { failures: u32 },
    Open { opened_at: Instant },
    HalfOpen { trial_permits: u32 },
}
