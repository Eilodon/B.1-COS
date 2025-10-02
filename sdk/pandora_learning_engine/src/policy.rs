use pandora_core::ontology::EpistemologicalFlow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Noop,
    Explore,
    Exploit,
}

pub trait Policy {
    fn select_action(&self, flow: &EpistemologicalFlow, explore_rate: f64) -> Action;
    fn update(&mut self, _flow: &EpistemologicalFlow, _advantage: f64) {}
}

#[derive(Default)]
pub struct EpsilonGreedyPolicy;

impl Policy for EpsilonGreedyPolicy {
    fn select_action(&self, _flow: &EpistemologicalFlow, explore_rate: f64) -> Action {
        if explore_rate > 0.0 { Action::Explore } else { Action::Exploit }
    }
    fn update(&mut self, _flow: &EpistemologicalFlow, _advantage: f64) {
        // no-op for sample stub
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn epsilon_greedy_switches_mode() {
        let pol = EpsilonGreedyPolicy::default();
        let flow = EpistemologicalFlow::from_bytes(Bytes::from_static(b"state"));
        assert_eq!(pol.select_action(&flow, 0.1), Action::Explore);
        assert_eq!(pol.select_action(&flow, 0.0), Action::Exploit);
    }
}


