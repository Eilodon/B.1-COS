use lru::LruCache;
use parking_lot::Mutex;
use std::num::NonZeroUsize;
use std::time::{Duration, Instant};
use tracing::{debug, warn};

/// Circuit breaker state with timestamp for TTL
#[derive(Debug, Clone)]
pub enum CircuitState {
    Closed {
        failures: u32,
        last_updated: Instant,
    },
    Open {
        opened_at: Instant,
    },
    HalfOpen {
        trial_permits: u32,
        last_updated: Instant,
    },
}

impl CircuitState {
    /// Check if state has expired based on TTL
    pub fn is_expired(&self, ttl: Duration) -> bool {
        let last_touch = match self {
            CircuitState::Closed { last_updated, .. } => *last_updated,
            CircuitState::Open { opened_at } => *opened_at,
            CircuitState::HalfOpen { last_updated, .. } => *last_updated,
        };
        last_touch.elapsed() > ttl
    }
}

/// Configuration for circuit breaker behavior
#[derive(Clone, Debug)]
pub struct CircuitBreakerConfig {
    /// Number of failures before opening circuit
    pub failure_threshold: u32,
    /// Duration to wait before trying half-open
    pub open_cooldown_ms: u64,
    /// Number of trial requests in half-open state
    pub half_open_trial: u32,
    /// Maximum number of circuits to track
    pub max_circuits: usize,
    /// TTL for idle circuit states
    pub state_ttl_secs: u64,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 3,
            open_cooldown_ms: 5_000,
            half_open_trial: 1,
            max_circuits: 1000,
            state_ttl_secs: 3600, // 1 hour
        }
    }
}

/// Circuit breaker manager with LRU eviction and TTL
pub struct CircuitBreakerManager {
    states: Mutex<LruCache<String, CircuitState>>,
    config: CircuitBreakerConfig,
}

impl CircuitBreakerManager {
    pub fn new(config: CircuitBreakerConfig) -> Self {
        let capacity = NonZeroUsize::new(config.max_circuits)
            .unwrap_or_else(|| NonZeroUsize::new(1).expect("1 is non-zero"));

        Self {
            states: Mutex::new(LruCache::new(capacity)),
            config,
        }
    }

    /// Check if circuit is open for a skill
    pub fn is_open(&self, skill_name: &str) -> bool {
        let mut states = self.states.lock();
        // Ensure exists
        if states.get(skill_name).is_none() {
            states.put(
                skill_name.to_string(),
                CircuitState::Closed {
                    failures: 0,
                    last_updated: Instant::now(),
                },
            );
        }
        // Get mutable reference for in-place transitions
        let state = match states.get_mut(skill_name) {
            Some(s) => s,
            None => {
                // Shouldn't happen due to insert above; reset to Closed
                states.put(
                    skill_name.to_string(),
                    CircuitState::Closed {
                        failures: 0,
                        last_updated: Instant::now(),
                    },
                );
                states
                    .get_mut(skill_name)
                    .expect("state exists after insert")
            }
        };

        // Check if expired and reset if needed
        let ttl = Duration::from_secs(self.config.state_ttl_secs);
        if state.is_expired(ttl) {
            debug!("Circuit state expired for '{}'", skill_name);
            // Remove expired entry entirely; caller sees circuit as closed
            let _ = state;
            states.pop(skill_name);
            return false;
        }

        // Check state
        match state {
            CircuitState::Closed { .. } => false,
            CircuitState::Open { opened_at } => {
                let elapsed = opened_at.elapsed();
                let cooldown = Duration::from_millis(self.config.open_cooldown_ms);

                if elapsed >= cooldown {
                    // Transition to half-open
                    debug!(
                        "Circuit cooldown elapsed for '{}', entering half-open",
                        skill_name
                    );
                    let consumed = self.config.half_open_trial.saturating_sub(1);
                    *state = CircuitState::HalfOpen {
                        trial_permits: consumed,
                        last_updated: Instant::now(),
                    };
                    false
                } else {
                    true
                }
            }
            CircuitState::HalfOpen { trial_permits, .. } => {
                if *trial_permits > 0 {
                    let remaining = trial_permits.saturating_sub(1);
                    *state = CircuitState::HalfOpen {
                        trial_permits: remaining,
                        last_updated: Instant::now(),
                    };
                    false
                } else {
                    true
                }
            }
        }
    }

    /// Record successful execution
    pub fn record_success(&self, skill_name: &str) {
        let mut states = self.states.lock();
        states.put(
            skill_name.to_string(),
            CircuitState::Closed {
                failures: 0,
                last_updated: Instant::now(),
            },
        );
        debug!("Circuit reset to closed for '{}'", skill_name);
    }

    /// Record failed execution
    pub fn record_failure(&self, skill_name: &str) {
        let mut states = self.states.lock();

        let new_state = match states.get(skill_name) {
            Some(CircuitState::Closed { failures, .. }) => {
                let new_failures = failures + 1;
                if new_failures >= self.config.failure_threshold {
                    warn!(
                        "Circuit opened for '{}' after {} failures",
                        skill_name, new_failures
                    );
                    CircuitState::Open {
                        opened_at: Instant::now(),
                    }
                } else {
                    debug!(
                        "Circuit failure {}/{} for '{}'",
                        new_failures, self.config.failure_threshold, skill_name
                    );
                    CircuitState::Closed {
                        failures: new_failures,
                        last_updated: Instant::now(),
                    }
                }
            }
            Some(CircuitState::HalfOpen { .. }) => {
                warn!(
                    "Circuit re-opened for '{}' after half-open failure",
                    skill_name
                );
                CircuitState::Open {
                    opened_at: Instant::now(),
                }
            }
            _ => {
                if self.config.failure_threshold <= 1 {
                    CircuitState::Open {
                        opened_at: Instant::now(),
                    }
                } else {
                    CircuitState::Closed {
                        failures: 1,
                        last_updated: Instant::now(),
                    }
                }
            }
        };

        states.put(skill_name.to_string(), new_state);
    }

    /// Get current statistics
    pub fn stats(&self) -> CircuitStats {
        let states = self.states.lock();
        let mut closed = 0;
        let mut open = 0;
        let mut half_open = 0;

        for (_, state) in states.iter() {
            match state {
                CircuitState::Closed { .. } => closed += 1,
                CircuitState::Open { .. } => open += 1,
                CircuitState::HalfOpen { .. } => half_open += 1,
            }
        }

        CircuitStats {
            total_circuits: states.len(),
            closed,
            open,
            half_open,
            capacity: states.cap().get(),
        }
    }

    /// Manually cleanup expired states (called periodically)
    pub fn cleanup_expired(&self) {
        let mut states = self.states.lock();
        let ttl = Duration::from_secs(self.config.state_ttl_secs);

        let to_remove: Vec<String> = states
            .iter()
            .filter_map(|(name, state)| {
                if state.is_expired(ttl) {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect();

        for name in to_remove {
            states.pop(&name);
            debug!("Cleaned up expired circuit state for '{}'", name);
        }
    }
}

#[derive(Debug, Clone)]
pub struct CircuitStats {
    pub total_circuits: usize,
    pub closed: usize,
    pub open: usize,
    pub half_open: usize,
    pub capacity: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_circuit_opens_after_threshold() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            ..Default::default()
        };
        let manager = CircuitBreakerManager::new(config);

        // Record failures
        assert!(!manager.is_open("test_skill"));
        manager.record_failure("test_skill");
        assert!(!manager.is_open("test_skill"));
        manager.record_failure("test_skill");
        assert!(!manager.is_open("test_skill"));
        manager.record_failure("test_skill");

        // Should be open now
        assert!(manager.is_open("test_skill"));
    }

    #[test]
    fn test_circuit_cooldown() {
        let config = CircuitBreakerConfig {
            failure_threshold: 1,
            open_cooldown_ms: 100,
            ..Default::default()
        };
        let manager = CircuitBreakerManager::new(config);

        // Open circuit
        manager.record_failure("test_skill");
        assert!(manager.is_open("test_skill"));

        // Wait for cooldown
        sleep(Duration::from_millis(150));

        // Should transition to half-open
        assert!(!manager.is_open("test_skill")); // First trial
        assert!(manager.is_open("test_skill")); // No more trials
    }

    #[test]
    fn test_circuit_success_resets() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            ..Default::default()
        };
        let manager = CircuitBreakerManager::new(config);

        // Build up failures
        manager.record_failure("test_skill");
        manager.record_failure("test_skill");

        // Success resets
        manager.record_success("test_skill");

        // Need 3 more failures to open
        manager.record_failure("test_skill");
        manager.record_failure("test_skill");
        assert!(!manager.is_open("test_skill"));
        manager.record_failure("test_skill");
        assert!(manager.is_open("test_skill"));
    }

    #[test]
    fn test_lru_eviction() {
        let config = CircuitBreakerConfig {
            max_circuits: 2,
            ..Default::default()
        };
        let manager = CircuitBreakerManager::new(config);

        // Add 3 skills (should evict oldest)
        manager.record_failure("skill_1");
        manager.record_failure("skill_2");
        manager.record_failure("skill_3");

        let stats = manager.stats();
        assert_eq!(stats.total_circuits, 2); // LRU evicted skill_1
    }

    #[test]
    fn test_ttl_expiration() {
        let config = CircuitBreakerConfig {
            state_ttl_secs: 1,
            ..Default::default()
        };
        let manager = CircuitBreakerManager::new(config);

        // Create state
        manager.record_failure("test_skill");
        assert_eq!(manager.stats().total_circuits, 1);

        // Wait for TTL
        sleep(Duration::from_secs(2));

        // Accessing should reset expired state
        assert!(!manager.is_open("test_skill"));

        // Cleanup removes it
        manager.cleanup_expired();
        assert_eq!(manager.stats().total_circuits, 0);
    }
}
