#[cfg(feature = "ml")]
use candle_core::{Tensor};
#[cfg(feature = "ml")]
use candle_nn::{Module, VarBuilder, linear, Linear};

#[cfg(feature = "ml")]
pub struct ProbabilisticOutput {
    pub mean: Tensor,
    pub variance: Tensor,
}

#[cfg(feature = "ml")]
pub struct ProbabilisticFeedForward {
    layer1: Linear,
    layer2_mean: Linear,
    layer2_variance: Linear,
    span: tracing::Span,
}

#[cfg(feature = "ml")]
impl ProbabilisticFeedForward {
    pub fn new(in_dim: usize, hidden_dim: usize, out_dim: usize, vb: VarBuilder) -> candle_core::Result<Self> {
        let layer1 = linear(in_dim, hidden_dim, vb.pp("layer1"))?;
        let layer2_mean = linear(hidden_dim, out_dim, vb.pp("layer2_mean"))?;
        let layer2_variance = linear(hidden_dim, out_dim, vb.pp("layer2_variance"))?;
        Ok(Self { layer1, layer2_mean, layer2_variance, span: tracing::span!(tracing::Level::TRACE, "prob-ff") })
    }
}

#[cfg(feature = "ml")]
impl Module for ProbabilisticFeedForward {
    fn forward(&self, xs: &Tensor) -> candle_core::Result<Tensor> {
        let xs = self.layer1.forward(xs)?.relu()?;
        self.layer2_mean.forward(&xs)
    }
}

#[cfg(feature = "ml")]
impl ProbabilisticFeedForward {
    pub fn forward_probabilistic(&self, xs: &Tensor) -> candle_core::Result<ProbabilisticOutput> {
        let _enter = self.span.enter();
        let hidden = self.layer1.forward(xs)?.relu()?;
        let mean = self.layer2_mean.forward(&hidden)?;
        let variance = self.layer2_variance.forward(&hidden)?.softplus()?;
        Ok(ProbabilisticOutput { mean, variance })
    }
}
