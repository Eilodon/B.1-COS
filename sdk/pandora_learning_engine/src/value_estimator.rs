use pandora_core::ontology::EpistemologicalFlow;

pub trait ValueEstimator {
    fn estimate(&self, flow: &EpistemologicalFlow) -> f64;
}

#[derive(Default)]
pub struct MeanRewardEstimator;

impl ValueEstimator for MeanRewardEstimator {
    fn estimate(&self, _flow: &EpistemologicalFlow) -> f64 {
        0.0
    }
}

#[derive(Debug, Clone)]
pub struct ExponentialMovingAverageEstimator {
    alpha: f64,
    state: std::collections::HashMap<u64, f64>, // keyed by hash of flow
}

impl ExponentialMovingAverageEstimator {
    pub fn new(alpha: f64) -> Self { Self { alpha, state: std::collections::HashMap::new() } }

    fn key(flow: &EpistemologicalFlow) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        // Rely on EpistemologicalFlow implementing Debug/Hash-like via bytes
        format!("{:?}", flow).hash(&mut hasher);
        hasher.finish()
    }

    pub fn update(&mut self, flow: &EpistemologicalFlow, reward: f64) {
        let k = Self::key(flow);
        let prev = *self.state.get(&k).unwrap_or(&reward);
        let ema = self.alpha * reward + (1.0 - self.alpha) * prev;
        self.state.insert(k, ema);
    }
}

impl ValueEstimator for ExponentialMovingAverageEstimator {
    fn estimate(&self, flow: &EpistemologicalFlow) -> f64 {
        let k = Self::key(flow);
        *self.state.get(&k).unwrap_or(&0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    #[test]
    fn default_estimator_returns_zero() {
        let est = MeanRewardEstimator::default();
        let flow = EpistemologicalFlow::from_bytes(Bytes::from_static(b"x"));
        assert_eq!(est.estimate(&flow), 0.0);
    }

    #[test]
    fn ema_estimator_updates_state() {
        let mut est = ExponentialMovingAverageEstimator::new(0.5);
        let flow = EpistemologicalFlow::from_bytes(Bytes::from_static(b"z"));
        assert_eq!(est.estimate(&flow), 0.0);
        est.update(&flow, 1.0);
        let v1 = est.estimate(&flow);
        assert!(v1 > 0.0);
        est.update(&flow, 0.0);
        let v2 = est.estimate(&flow);
        assert!(v2 >= 0.0);
    }
}


