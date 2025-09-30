use async_trait::async_trait;

#[async_trait]
pub trait FepCell {
    type Belief;
    type Observation;
    type Action;

    fn get_internal_model(&self) -> &Self::Belief;
    async fn perceive(&mut self, observation: Self::Observation) -> f64;
    async fn act(&mut self) -> Option<Self::Action>;
}
