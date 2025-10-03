use async_trait::async_trait;
use fnv::FnvHashMap;
use pandora_core::interfaces::skills::SkillModule;
use pandora_error::PandoraError;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
pub mod circuit_breaker;
pub use circuit_breaker::{CircuitBreakerConfig, CircuitBreakerManager, CircuitStats};
pub mod simple_api;
pub mod static_skills;
pub mod automatic_scientist_orchestrator;
#[cfg(feature = "ml")]
pub use automatic_scientist_orchestrator::{AutomaticScientistOrchestrator, ExperimentState, ExperimentResult, ScientistState};
#[cfg(feature = "ml")]
#[cfg(test)]
mod automatic_scientist_test;
#[cfg(feature = "prometheus_export")]
use once_cell::sync::Lazy;
#[cfg(feature = "prometheus_export")]
use prometheus::{register_counter_vec, register_histogram_vec, CounterVec, HistogramVec};
pub use static_skills::{HybridSkillRegistry, StaticSkill};
use tokio::time::sleep;
use tokio::time::timeout as tokio_timeout;
use tracing::{info, instrument, warn};

#[cfg(feature = "schema_validation")]
use jsonschema::{Draft, JSONSchema};

/// Trait định nghĩa interface cho Orchestrator để dễ dàng testing và mocking
#[async_trait]
pub trait OrchestratorTrait: Send + Sync {
    /// Xử lý một request với skill name và input
    async fn process_request(
        &self,
        skill_name: &str,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, PandoraError>;

    /// Xử lý một request với RoutingPolicy (primary + fallbacks)
    /// API công khai để bật định tuyến và dự phòng theo chính sách.
    async fn process_with_policy(
        &self,
        routing: RoutingPolicy,
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
        Self {
            skills: FnvHashMap::default(),
        }
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

impl Default for SkillRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// `Orchestrator` là bộ não trung tâm, điều phối luồng nhận thức và thực thi các skill.
pub struct Orchestrator {
    registry: Arc<SkillRegistry>,
    retry_policy: RetryPolicy,
    timeout_policy: TimeoutPolicy,
    circuit_breaker: Arc<CircuitBreakerManager>,
    // Optional hybrid registry for static-dispatch skills
    hybrid_registry: Option<Arc<HybridSkillRegistry>>,
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

    pub fn with_config(registry: Arc<SkillRegistry>, circuit_config: CircuitBreakerConfig) -> Self {
        info!(
            "Orchestrator initialized with {} skills",
            registry.skills.len()
        );
        Self {
            registry,
            retry_policy: RetryPolicy::default(),
            timeout_policy: TimeoutPolicy::default(),
            circuit_breaker: Arc::new(CircuitBreakerManager::new(circuit_config)),
            hybrid_registry: None,
        }
    }

    /// Create orchestrator with static dispatch for built-in skills.
    /// Faster for built-in skills; dynamic plugins still supported.
    pub fn new_with_static_dispatch() -> Self {
        let mut hybrid = HybridSkillRegistry::new();
        // Register built-in skills
        let _ = hybrid.register_static("arithmetic");
        let _ = hybrid.register_static("logical_reasoning");
        let _ = hybrid.register_static("pattern_matching");
        let _ = hybrid.register_static("analogy_reasoning");

        let registry = Arc::new(SkillRegistry::new());
        let mut this = Self::with_config(registry, CircuitBreakerConfig::default());
        this.hybrid_registry = Some(Arc::new(hybrid));
        this
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
    #[instrument(name = "orchestrator.process_with_routing", skip_all, fields(primary = %routing.primary, fallbacks = routing.fallbacks.len()))]
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
            #[cfg(feature = "prometheus_export")]
            let _timer = METRICS
                .request_duration
                .with_label_values(&[&skill_name])
                .start_timer();
            match self.try_execute_with_policies(&skill_name, &input).await {
                Ok(val) => return Ok(val),
                Err(e) => {
                    warn!("Policy attempt failed for skill '{}': {}", skill_name, e);
                    #[cfg(feature = "prometheus_export")]
                    {
                        METRICS
                            .request_errors
                            .with_label_values(&[&skill_name])
                            .inc();
                    }
                    last_err = Some(e);
                    // Nếu lỗi do circuit open hoặc timeout, thử fallback tiếp theo
                    continue;
                }
            }
            // timer dropped here
        }
        if let Some(e) = last_err {
            Err(e)
        } else {
            Err(PandoraError::Unknown("Routing exhausted".into()))
        }
    }

    #[instrument(name = "orchestrator.try_execute_with_policies", skip_all, fields(skill = skill_name))]
    async fn try_execute_with_policies(
        &self,
        skill_name: &str,
        input: &serde_json::Value,
    ) -> Result<serde_json::Value, PandoraError> {
        // Circuit breaker gate
        if self.is_circuit_open(skill_name) {
            warn!("Circuit open for skill '{}'", skill_name);
            return Err(PandoraError::CircuitOpen {
                resource: skill_name.to_string(),
            });
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
                    #[cfg(feature = "prometheus_export")]
                    {
                        METRICS
                            .request_errors
                            .with_label_values(&[skill_name])
                            .inc();
                    }
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

        if let Some(e) = last_err {
            Err(e)
        } else {
            Err(PandoraError::Unknown(
                "Retry loop exited without error".into(),
            ))
        }
    }

    #[instrument(name = "orchestrator.execute_with_timeout", skip_all, fields(skill = skill_name))]
    async fn execute_with_timeout(
        &self,
        skill_name: &str,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, PandoraError> {
        // Prefer hybrid registry when available
        if let Some(hybrid) = &self.hybrid_registry {
            if let Some(skill_ref) = hybrid.get(skill_name) {
                let timeout_ms = self.timeout_policy.timeout_ms;
                let exec_fut = skill_ref.execute(input);
                return match tokio_timeout(Duration::from_millis(timeout_ms), exec_fut).await {
                    Ok(res) => {
                        #[cfg(feature = "prometheus_export")]
                        {
                            METRICS.request_total.with_label_values(&[skill_name]).inc();
                        }
                        res
                    }
                    Err(_) => Err(PandoraError::Timeout {
                        operation: format!("skill:{}", skill_name),
                        timeout_ms,
                    }),
                };
            }
        }

        let skill =
            self.registry
                .get_skill(skill_name)
                .ok_or_else(|| PandoraError::SkillNotFound {
                    skill_name: skill_name.to_string(),
                })?;

        // Optional: validate input against skill's declared JSON schema
        #[cfg(feature = "schema_validation")]
        {
            let descriptor = skill.descriptor();
            if !descriptor.input_schema.is_empty() {
                match serde_json::from_str::<serde_json::Value>(&descriptor.input_schema) {
                    Ok(schema_json) => {
                        if let Err(err) = compile_and_validate(&schema_json, &input) {
                            return Err(PandoraError::InvalidSkillInput {
                                skill_name: skill_name.to_string(),
                                message: format!("Schema validation failed: {}", err),
                            });
                        }
                    }
                    Err(e) => {
                        warn!(
                            "Invalid input_schema JSON for skill '{}': {}",
                            skill_name, e
                        );
                    }
                }
            }
        }

        let timeout_ms = self.timeout_policy.timeout_ms;
        let exec_fut = skill.execute(input);
        match tokio_timeout(Duration::from_millis(timeout_ms), exec_fut).await {
            Ok(res) => {
                #[cfg(feature = "schema_validation")]
                {
                    if let Ok(ref output_val) = res {
                        let descriptor = skill.descriptor();
                        if !descriptor.output_schema.is_empty() {
                            if let Ok(schema_json) =
                                serde_json::from_str::<serde_json::Value>(&descriptor.output_schema)
                            {
                                if let Err(err) = compile_and_validate(&schema_json, output_val) {
                                    return Err(PandoraError::InvalidSkillInput {
                                        skill_name: skill_name.to_string(),
                                        message: format!(
                                            "Output schema validation failed: {}",
                                            err
                                        ),
                                    });
                                }
                            }
                        }
                    }
                }
                #[cfg(feature = "prometheus_export")]
                {
                    METRICS.request_total.with_label_values(&[skill_name]).inc();
                }
                res
            }
            Err(_) => Err(PandoraError::Timeout {
                operation: format!("skill:{}", skill_name),
                timeout_ms,
            }),
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

    async fn process_with_policy(
        &self,
        routing: RoutingPolicy,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, PandoraError> {
        info!(
            "Orchestrator: Xử lý với routing policy (primary='{}', fallbacks={})",
            routing.primary,
            routing.fallbacks.len()
        );
        self.process_with_routing(routing, input).await
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
#[allow(dead_code)]
enum CircuitState {
    Closed { failures: u32 },
    Open { opened_at: Instant },
    HalfOpen { trial_permits: u32 },
}

// =========================
// Config loading
// =========================

#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct OrchestratorConfig {
    #[serde(default)]
    pub retry: RetryConfig,
    #[serde(default)]
    pub timeout: TimeoutConfig,
    #[serde(default)]
    pub circuit: CircuitConfig,
}

// Default derived above

#[derive(Clone, Debug, serde::Deserialize)]
pub struct RetryConfig {
    #[serde(default = "RetryConfig::default_max_retries")]
    pub max_retries: u32,
    #[serde(default = "RetryConfig::default_initial_backoff_ms")]
    pub initial_backoff_ms: u64,
    #[serde(default = "RetryConfig::default_backoff_multiplier")]
    pub backoff_multiplier: f64,
    #[serde(default = "RetryConfig::default_max_backoff_ms")]
    pub max_backoff_ms: u64,
    #[serde(default = "RetryConfig::default_jitter_ms")]
    pub jitter_ms: u64,
}

impl RetryConfig {
    fn default_max_retries() -> u32 {
        2
    }
    fn default_initial_backoff_ms() -> u64 {
        50
    }
    fn default_backoff_multiplier() -> f64 {
        2.0
    }
    fn default_max_backoff_ms() -> u64 {
        1_000
    }
    fn default_jitter_ms() -> u64 {
        25
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: Self::default_max_retries(),
            initial_backoff_ms: Self::default_initial_backoff_ms(),
            backoff_multiplier: Self::default_backoff_multiplier(),
            max_backoff_ms: Self::default_max_backoff_ms(),
            jitter_ms: Self::default_jitter_ms(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct TimeoutConfig {
    #[serde(default = "TimeoutConfig::default_timeout_ms")]
    pub timeout_ms: u64,
}

impl TimeoutConfig {
    fn default_timeout_ms() -> u64 {
        2_000
    }
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            timeout_ms: Self::default_timeout_ms(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct CircuitConfig {
    #[serde(default = "CircuitConfig::default_failure_threshold")]
    pub failure_threshold: u32,
    #[serde(default = "CircuitConfig::default_open_cooldown_ms")]
    pub open_cooldown_ms: u64,
    #[serde(default = "CircuitConfig::default_half_open_trial")]
    pub half_open_trial: u32,
}

impl CircuitConfig {
    fn default_failure_threshold() -> u32 {
        3
    }
    fn default_open_cooldown_ms() -> u64 {
        5_000
    }
    fn default_half_open_trial() -> u32 {
        1
    }
}

impl Default for CircuitConfig {
    fn default() -> Self {
        Self {
            failure_threshold: Self::default_failure_threshold(),
            open_cooldown_ms: Self::default_open_cooldown_ms(),
            half_open_trial: Self::default_half_open_trial(),
        }
    }
}

impl OrchestratorConfig {
    pub fn from_sources() -> Result<Self, PandoraError> {
        let mut builder = config::Config::builder();
        builder = builder
            .set_default("retry.max_retries", RetryConfig::default_max_retries())
            .map_err(|e| PandoraError::Unknown(format!("config default error: {}", e)))?;
        builder = builder
            .set_default(
                "retry.initial_backoff_ms",
                RetryConfig::default_initial_backoff_ms(),
            )
            .map_err(|e| PandoraError::Unknown(format!("config default error: {}", e)))?;
        builder = builder
            .set_default(
                "retry.backoff_multiplier",
                RetryConfig::default_backoff_multiplier(),
            )
            .map_err(|e| PandoraError::Unknown(format!("config default error: {}", e)))?;
        builder = builder
            .set_default(
                "retry.max_backoff_ms",
                RetryConfig::default_max_backoff_ms(),
            )
            .map_err(|e| PandoraError::Unknown(format!("config default error: {}", e)))?;
        builder = builder
            .set_default("retry.jitter_ms", RetryConfig::default_jitter_ms())
            .map_err(|e| PandoraError::Unknown(format!("config default error: {}", e)))?;
        builder = builder
            .set_default("timeout.timeout_ms", TimeoutConfig::default_timeout_ms())
            .map_err(|e| PandoraError::Unknown(format!("config default error: {}", e)))?;
        builder = builder
            .set_default(
                "circuit.failure_threshold",
                CircuitConfig::default_failure_threshold(),
            )
            .map_err(|e| PandoraError::Unknown(format!("config default error: {}", e)))?;
        builder = builder
            .set_default(
                "circuit.open_cooldown_ms",
                CircuitConfig::default_open_cooldown_ms(),
            )
            .map_err(|e| PandoraError::Unknown(format!("config default error: {}", e)))?;
        builder = builder
            .set_default(
                "circuit.half_open_trial",
                CircuitConfig::default_half_open_trial(),
            )
            .map_err(|e| PandoraError::Unknown(format!("config default error: {}", e)))?;

        // Optional: load from file orchestrator.toml if present
        builder = builder.add_source(config::File::with_name("orchestrator").required(false));
        // Env overrides with prefix ORCH_, flatten with __ separator, e.g., ORCH_RETRY__MAX_RETRIES
        builder = builder.add_source(config::Environment::with_prefix("ORCH").separator("__"));

        let cfg = builder
            .build()
            .map_err(|e| PandoraError::Unknown(format!("config load error: {}", e)))?;
        cfg.try_deserialize::<Self>()
            .map_err(|e| PandoraError::Unknown(format!("config deserialize error: {}", e)))
    }

    pub fn apply(self, orchestrator: Orchestrator) -> Orchestrator {
        let retry = RetryPolicy {
            max_retries: self.retry.max_retries,
            initial_backoff_ms: self.retry.initial_backoff_ms,
            backoff_multiplier: self.retry.backoff_multiplier,
            max_backoff_ms: self.retry.max_backoff_ms,
            jitter_ms: self.retry.jitter_ms,
        };
        let timeout = TimeoutPolicy {
            timeout_ms: self.timeout.timeout_ms,
        };
        let circuit_cfg = CircuitBreakerConfig {
            failure_threshold: self.circuit.failure_threshold,
            open_cooldown_ms: self.circuit.open_cooldown_ms,
            half_open_trial: self.circuit.half_open_trial,
            max_circuits: CircuitBreakerConfig::default().max_circuits,
            state_ttl_secs: CircuitBreakerConfig::default().state_ttl_secs,
        };

        Orchestrator {
            registry: orchestrator.registry,
            retry_policy: retry,
            timeout_policy: timeout,
            circuit_breaker: Arc::new(CircuitBreakerManager::new(circuit_cfg)),
            hybrid_registry: orchestrator.hybrid_registry,
        }
    }
}

#[cfg(feature = "schema_validation")]
fn compile_and_validate(
    schema_json: &serde_json::Value,
    instance: &serde_json::Value,
) -> Result<(), String> {
    let compiled = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(schema_json)
        .map_err(|e| format!("schema compile error: {}", e))?;
    if let Err(errors) = compiled.validate(instance) {
        let mut msgs: Vec<String> = Vec::new();
        for err in errors {
            msgs.push(format!("{} at {}", err, err.instance_path));
            if msgs.len() >= 5 {
                break;
            }
        }
        return Err(msgs.join(" | "));
    }
    Ok(())
}

#[cfg(feature = "prometheus_export")]
struct Metrics {
    request_total: CounterVec,
    request_errors: CounterVec,
    request_duration: HistogramVec,
}

#[cfg(feature = "prometheus_export")]
static METRICS: Lazy<Metrics> = Lazy::new(|| {
    let request_total = register_counter_vec!(
        "cognitive_requests_total",
        "Total number of skill execution requests",
        &["skill"]
    )
    .unwrap_or_else(|e| panic!("register counter: {}", e));

    let request_errors = register_counter_vec!(
        "cognitive_requests_failed_total",
        "Total number of failed skill execution requests",
        &["skill"]
    )
    .unwrap_or_else(|e| panic!("register counter: {}", e));

    let request_duration = register_histogram_vec!(
        "cognitive_request_duration_seconds",
        "Skill execution duration in seconds",
        &["skill"],
        vec![0.005, 0.01, 0.02, 0.05, 0.1, 0.2, 0.5, 1.0]
    )
    .unwrap_or_else(|e| panic!("register histogram: {}", e));

    Metrics {
        request_total,
        request_errors,
        request_duration,
    }
});
