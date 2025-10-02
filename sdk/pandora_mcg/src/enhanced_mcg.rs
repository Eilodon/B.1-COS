use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use tracing::{info, warn};

/// System metrics collected by the Meta-Cognitive Governor for monitoring.
///
/// These metrics represent the current state of the cognitive system and are used
/// to make decisions about self-improvement actions.
///
/// # Examples
///
/// ```rust
/// use pandora_mcg::enhanced_mcg::{SystemMetrics, ResourceMetrics};
///
/// let metrics = SystemMetrics {
///     uncertainty: 0.3,
///     compression_reward: 0.8,
///     novelty_score: 0.5,
///     performance: 0.9,
///     resource_usage: ResourceMetrics {
///         cpu_usage: 0.6,
///         memory_usage: 0.7,
///         latency_ms: 10.0,
///     },
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub uncertainty: f32,
    pub compression_reward: f64,
    pub novelty_score: f32,
    pub performance: f32,
    pub resource_usage: ResourceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub latency_ms: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    pub timestamp: u64,
    pub metrics: SystemMetrics,
    pub decision: Option<ActionTrigger>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionTrigger {
    TriggerSelfImprovementLevel1 {
        reason: String,
        target_component: String,
        confidence: f32,
    },
    TriggerSelfImprovementLevel2 {
        reason: String,
        target_component: String,
        confidence: f32,
    },
    TriggerSelfImprovementLevel3 {
        reason: String,
        target_component: String,
        confidence: f32,
    },
    RequestMoreInformation {
        reason: String,
        priority: Priority,
    },
    OptimizeResources {
        reason: String,
        target: String,
    },
    NoAction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct AdaptiveThreshold {
    base_value: f32,
    current_value: f32,
    learning_rate: f32,
    min_value: f32,
    max_value: f32,
}

impl AdaptiveThreshold {
    pub fn new(base: f32, min: f32, max: f32, learning_rate: f32) -> Self {
        Self { base_value: base, current_value: base, learning_rate, min_value: min, max_value: max }
    }

    pub fn get(&self) -> f32 { self.current_value }

    pub fn adapt(&mut self, performance_delta: f32) {
        let adjustment = -performance_delta * self.learning_rate;
        self.current_value = (self.current_value + adjustment).clamp(self.min_value, self.max_value);
        info!("MCG: Adapted threshold from {:.4} to {:.4} (delta: {:.4})", self.base_value, self.current_value, adjustment);
    }

    pub fn reset(&mut self) { self.current_value = self.base_value; }
}

#[derive(Debug, Clone)]
pub struct AnomalyDetector {
    window_size: usize,
    history: VecDeque<f32>,
    threshold_std: f32,
}

impl AnomalyDetector {
    pub fn new(window_size: usize, threshold_std: f32) -> Self {
        Self { window_size, history: VecDeque::with_capacity(window_size), threshold_std }
    }

    pub fn score(&mut self, value: f32) -> f32 {
        if self.history.len() >= self.window_size { self.history.pop_front(); }
        self.history.push_back(value);
        if self.history.len() < 3 { return 0.0; }
        let mean: f32 = self.history.iter().sum::<f32>() / self.history.len() as f32;
        let variance: f32 = self.history.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / self.history.len() as f32;
        let std = variance.sqrt();
        if std < 1e-6 { return 0.0; }
        let z_score = ((value - mean) / std).abs();
        if z_score > self.threshold_std {
            warn!("MCG: Anomaly detected! Value: {:.4}, Mean: {:.4}, Std: {:.4}, Z-score: {:.4}", value, mean, std, z_score);
            z_score / self.threshold_std
        } else { 0.0 }
    }
}

#[derive(Debug, Clone)]
pub struct ConfidenceTracker {
    success_count: usize,
    total_count: usize,
    recent_successes: VecDeque<bool>,
    window_size: usize,
}

impl ConfidenceTracker {
    pub fn new(window_size: usize) -> Self {
        Self { success_count: 0, total_count: 0, recent_successes: VecDeque::with_capacity(window_size), window_size }
    }

    pub fn compute(&self, anomaly_score: f32) -> f32 {
        let base_confidence = if self.total_count > 0 { self.success_count as f32 / self.total_count as f32 } else { 0.5 };
        let anomaly_penalty = anomaly_score * 0.3;
        (base_confidence - anomaly_penalty).clamp(0.0, 1.0)
    }

    pub fn update(&mut self, success: bool) {
        if self.recent_successes.len() >= self.window_size {
            if let Some(old) = self.recent_successes.pop_front() {
                if old { self.success_count = self.success_count.saturating_sub(1); }
                self.total_count = self.total_count.saturating_sub(1);
            }
        }
        self.recent_successes.push_back(success);
        if success { self.success_count += 1; }
        self.total_count += 1;
    }

    pub fn success_rate(&self) -> f32 {
        if self.total_count > 0 { self.success_count as f32 / self.total_count as f32 } else { 0.5 }
    }
}

/// Enhanced Meta-Cognitive Governor with adaptive thresholds and anomaly detection.
///
/// This governor monitors system metrics and makes intelligent decisions about
/// when to trigger self-improvement actions based on adaptive thresholds and
/// historical performance.
///
/// # Examples
///
/// ```rust
/// use pandora_mcg::enhanced_mcg::{EnhancedMetaCognitiveGovernor, SystemMetrics, ResourceMetrics};
///
/// let mut mcg = EnhancedMetaCognitiveGovernor::new();
/// let metrics = SystemMetrics {
///     uncertainty: 0.8,
///     compression_reward: 0.9,
///     novelty_score: 0.7,
///     performance: 0.5,
///     resource_usage: ResourceMetrics {
///         cpu_usage: 0.6,
///         memory_usage: 0.7,
///         latency_ms: 15.0,
///     },
/// };
///
/// let decision = mcg.monitor_comprehensive(&metrics);
/// // The governor will analyze metrics and potentially trigger improvements
/// ```
pub struct EnhancedMetaCognitiveGovernor {
    uncertainty_threshold: AdaptiveThreshold,
    compression_threshold: AdaptiveThreshold,
    novelty_threshold: AdaptiveThreshold,
    uncertainty_detector: AnomalyDetector,
    compression_detector: AnomalyDetector,
    novelty_detector: AnomalyDetector,
    confidence_tracker: ConfidenceTracker,
    state_history: VecDeque<SystemState>,
    max_history: usize,
    min_samples_for_adaptation: usize,
}

impl EnhancedMetaCognitiveGovernor {
    pub fn new() -> Self {
        Self {
            uncertainty_threshold: AdaptiveThreshold::new(0.5, 0.2, 0.8, 0.05),
            compression_threshold: AdaptiveThreshold::new(0.7, 0.3, 0.9, 0.05),
            novelty_threshold: AdaptiveThreshold::new(0.6, 0.3, 0.9, 0.05),
            uncertainty_detector: AnomalyDetector::new(50, 2.5),
            compression_detector: AnomalyDetector::new(50, 2.5),
            novelty_detector: AnomalyDetector::new(50, 2.5),
            confidence_tracker: ConfidenceTracker::new(100),
            state_history: VecDeque::with_capacity(1000),
            max_history: 1000,
            min_samples_for_adaptation: 20,
        }
    }

    pub fn monitor_comprehensive(&mut self, metrics: &SystemMetrics) -> DecisionWithConfidence {
        info!("\n=== Meta-Cognitive Governor - Comprehensive Monitoring ===");
        let uncertainty_anomaly = self.uncertainty_detector.score(metrics.uncertainty);
        let compression_anomaly = self.compression_detector.score(metrics.compression_reward as f32);
        let novelty_anomaly = self.novelty_detector.score(metrics.novelty_score);
        let max_anomaly = uncertainty_anomaly.max(compression_anomaly).max(novelty_anomaly);
        if max_anomaly > 0.0 { warn!("MCG: Anomaly detected! Score: {:.4}", max_anomaly); }

        let mut triggers = Vec::new();
        if metrics.uncertainty > self.uncertainty_threshold.get() {
            info!("MCG: High uncertainty detected: {:.4} > {:.4}", metrics.uncertainty, self.uncertainty_threshold.get());
            triggers.push((
                "uncertainty",
                ActionTrigger::TriggerSelfImprovementLevel1 {
                    reason: format!("Uncertainty {:.4} exceeds threshold {:.4}", metrics.uncertainty, self.uncertainty_threshold.get()),
                    target_component: "world_model".to_string(),
                    confidence: 0.0,
                },
            ));
        }
        if metrics.compression_reward > self.compression_threshold.get() as f64 {
            info!("MCG: High compression reward: {:.4} > {:.4}", metrics.compression_reward, self.compression_threshold.get());
            triggers.push((
                "compression",
                ActionTrigger::TriggerSelfImprovementLevel2 {
                    reason: format!("Compression reward {:.4} indicates learning opportunity", metrics.compression_reward),
                    target_component: "learning_engine".to_string(),
                    confidence: 0.0,
                },
            ));
        }
        if metrics.novelty_score > self.novelty_threshold.get() {
            info!("MCG: High novelty detected: {:.4} > {:.4}", metrics.novelty_score, self.novelty_threshold.get());
            triggers.push((
                "novelty",
                ActionTrigger::TriggerSelfImprovementLevel3 {
                    reason: format!("Novelty score {:.4} suggests new pattern", metrics.novelty_score),
                    target_component: "pattern_recognizer".to_string(),
                    confidence: 0.0,
                },
            ));
        }
        if metrics.resource_usage.cpu_usage > 0.9 || metrics.resource_usage.memory_usage > 0.9 {
            warn!("MCG: High resource usage - CPU: {:.2}%, Memory: {:.2}%", metrics.resource_usage.cpu_usage * 100.0, metrics.resource_usage.memory_usage * 100.0);
            triggers.push((
                "resources",
                ActionTrigger::OptimizeResources { reason: "High resource utilization".to_string(), target: "system".to_string() },
            ));
        }

        let (decision, confidence) = if triggers.is_empty() {
            info!("MCG: System stable. No action needed.");
            (ActionTrigger::NoAction, 1.0)
        } else {
            let (_, mut action) = triggers[0].clone();
            let confidence = self.confidence_tracker.compute(max_anomaly);
            action = match action {
                ActionTrigger::TriggerSelfImprovementLevel1 { reason, target_component, .. } => ActionTrigger::TriggerSelfImprovementLevel1 { reason, target_component, confidence },
                ActionTrigger::TriggerSelfImprovementLevel2 { reason, target_component, .. } => ActionTrigger::TriggerSelfImprovementLevel2 { reason, target_component, confidence },
                ActionTrigger::TriggerSelfImprovementLevel3 { reason, target_component, .. } => ActionTrigger::TriggerSelfImprovementLevel3 { reason, target_component, confidence },
                other => other,
            };
            (action, confidence)
        };

        self.add_state(SystemState {
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
            metrics: metrics.clone(),
            decision: if matches!(decision, ActionTrigger::NoAction) { None } else { Some(decision.clone()) },
        });

        if self.state_history.len() >= self.min_samples_for_adaptation { self.adapt_thresholds(); }
        info!("=== Meta-Cognitive Governor - Decision Complete ===\n");
        DecisionWithConfidence { decision, confidence }
    }

    fn add_state(&mut self, state: SystemState) {
        if self.state_history.len() >= self.max_history { self.state_history.pop_front(); }
        self.state_history.push_back(state);
    }

    fn adapt_thresholds(&mut self) {
        if self.state_history.len() < 2 { return; }
        let recent = &self.state_history[self.state_history.len() - 1];
        let previous = &self.state_history[self.state_history.len() - 2];
        let performance_delta = recent.metrics.performance - previous.metrics.performance;
        self.uncertainty_threshold.adapt(performance_delta);
        self.compression_threshold.adapt(performance_delta);
        self.novelty_threshold.adapt(performance_delta);
    }

    pub fn report_outcome(&mut self, success: bool) {
        self.confidence_tracker.update(success);
        info!("MCG: Outcome reported. Success rate: {:.2}%", self.confidence_tracker.success_rate() * 100.0);
    }

    pub fn get_health_summary(&self) -> HealthSummary {
        HealthSummary {
            confidence: self.confidence_tracker.success_rate(),
            state_history_size: self.state_history.len(),
            current_thresholds: ThresholdSummary { uncertainty: self.uncertainty_threshold.get(), compression: self.compression_threshold.get(), novelty: self.novelty_threshold.get() },
        }
    }
}

impl Default for EnhancedMetaCognitiveGovernor { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionWithConfidence { pub decision: ActionTrigger, pub confidence: f32 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthSummary { pub confidence: f32, pub state_history_size: usize, pub current_thresholds: ThresholdSummary }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdSummary { pub uncertainty: f32, pub compression: f32, pub novelty: f32 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_threshold() {
        let mut threshold = AdaptiveThreshold::new(0.5, 0.2, 0.8, 0.1);
        threshold.adapt(0.1);
        assert!(threshold.get() < 0.5);
        threshold.adapt(-0.1);
        assert!(threshold.get() >= 0.2 && threshold.get() <= 0.8);
    }

    #[test]
    fn test_anomaly_detector() {
        let mut detector = AnomalyDetector::new(10, 2.0);
        for _ in 0..10 { assert_eq!(detector.score(5.0), 0.0); }
        let score = detector.score(50.0);
        assert!(score > 0.0);
    }

    #[test]
    fn test_confidence_tracker() {
        let mut tracker = ConfidenceTracker::new(10);
        for _ in 0..7 { tracker.update(true); }
        for _ in 0..3 { tracker.update(false); }
        assert!(tracker.success_rate() > 0.6);
    }
}


