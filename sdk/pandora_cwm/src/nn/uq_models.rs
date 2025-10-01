// Tạm thời cung cấp stub ML khi bật feature `ml` mà không cần phụ thuộc candle
// để các crate khác có thể compile với `--all-features`.

#[cfg(feature = "ml")]
pub struct ProbabilisticOutput {
    pub variance: Variance,
}

#[cfg(feature = "ml")]
impl ProbabilisticOutput {
    pub fn new(variance: f32) -> Self { Self { variance: Variance(variance) } }
}

#[cfg(feature = "ml")]
pub struct Variance(pub f32);

#[cfg(feature = "ml")]
impl Variance {
    pub fn mean_all(&self) -> Result<Variance, ()> { Ok(Variance(self.0)) }

    pub fn to_scalar<T>(&self) -> Result<T, ()>
    where
        T: From<f32>,
    {
        Ok(T::from(self.0))
    }
}
