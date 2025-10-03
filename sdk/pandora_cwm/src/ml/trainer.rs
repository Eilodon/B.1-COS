#[cfg(feature = "ml")]
use crate::ml::predictor::WorldModelPredictor;
#[cfg(feature = "ml")]
use pandora_error::PandoraError;

#[cfg(feature = "ml")]
pub struct WorldModelTrainer;

#[cfg(feature = "ml")]
impl WorldModelTrainer {
    pub fn train_basic(
        predictor: &mut WorldModelPredictor,
        x: Vec<f64>,
        nrows: usize,
        ncols: usize,
        y: Vec<i32>,
    ) -> Result<(), PandoraError> {
        predictor.train(x, nrows, ncols, y)
    }
}
