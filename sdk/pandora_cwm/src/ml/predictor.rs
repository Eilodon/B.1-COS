#[cfg(feature = "ml")]
use pandora_error::PandoraError;
#[cfg(feature = "ml")]
use smartcore::linear::logistic_regression::LogisticRegression;
#[cfg(feature = "ml")]
use smartcore::linalg::basic::matrix::DenseMatrix;

/// A machine learning predictor for world model predictions using logistic regression.
///
/// This predictor uses `smartcore`'s logistic regression implementation to learn
/// patterns from world model data and make predictions about future states.
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "ml")]
/// # {
/// use pandora_cwm::ml::predictor::WorldModelPredictor;
///
/// let mut predictor = WorldModelPredictor::new(2);
/// let x = vec![0.0, 0.0, 1.0, 1.0, 2.0, 2.0];
/// let y = vec![0, 1, 1];
/// predictor.train(x, 3, 2, y).unwrap();
///
/// let test_x = vec![1.5, 1.5];
/// let predictions = predictor.predict(test_x, 1, 2).unwrap();
/// assert_eq!(predictions.len(), 1);
/// # }
/// ```
#[cfg(feature = "ml")]
pub struct WorldModelPredictor {
    model: Option<LogisticRegression<f64, i32, DenseMatrix<f64>, Vec<i32>>>,
    feature_dim: usize,
}

#[cfg(feature = "ml")]
impl WorldModelPredictor {
    /// Creates a new predictor with the specified feature dimension.
    ///
    /// # Arguments
    ///
    /// * `feature_dim` - The number of input features expected by the model
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "ml")]
    /// # {
    /// use pandora_cwm::ml::predictor::WorldModelPredictor;
    /// let predictor = WorldModelPredictor::new(10);
    /// # }
    /// ```
    pub fn new(feature_dim: usize) -> Self {
        Self { model: None, feature_dim }
    }

    /// Trains the predictor on the provided data.
    ///
    /// # Arguments
    ///
    /// * `x` - Training features as a flattened vector
    /// * `nrows` - Number of training samples
    /// * `ncols` - Number of features per sample
    /// * `y` - Training labels
    ///
    /// # Errors
    ///
    /// Returns `PandoraError::config` if feature dimensions don't match or training fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "ml")]
    /// # {
    /// use pandora_cwm::ml::predictor::WorldModelPredictor;
    /// let mut predictor = WorldModelPredictor::new(2);
    /// let x = vec![0.0, 0.0, 1.0, 1.0];
    /// let y = vec![0, 1];
    /// predictor.train(x, 2, 2, y).unwrap();
    /// # }
    /// ```
    pub fn train(&mut self, x: Vec<f64>, nrows: usize, ncols: usize, y: Vec<i32>) -> Result<(), PandoraError> {
        if ncols != self.feature_dim {
            return Err(PandoraError::config(format!(
                "Expected {} features, got {}",
                self.feature_dim,
                ncols
            )));
        }
        let x_mat = DenseMatrix::new(nrows, ncols, x.clone(), false);
        let model = LogisticRegression::fit(&x_mat, &y, Default::default())
            .map_err(|e| PandoraError::config_with_source("Training failed", e))?;
        self.model = Some(model);
        Ok(())
    }

    /// Makes predictions on new data using the trained model.
    ///
    /// # Arguments
    ///
    /// * `x` - Test features as a flattened vector
    /// * `nrows` - Number of test samples
    /// * `ncols` - Number of features per sample
    ///
    /// # Errors
    ///
    /// Returns `PandoraError::config` if the model hasn't been trained or prediction fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "ml")]
    /// # {
    /// use pandora_cwm::ml::predictor::WorldModelPredictor;
    /// let mut predictor = WorldModelPredictor::new(2);
    /// let x = vec![0.0, 0.0, 1.0, 1.0];
    /// let y = vec![0, 1];
    /// predictor.train(x, 2, 2, y).unwrap();
    /// 
    /// let test_x = vec![0.5, 0.5];
    /// let predictions = predictor.predict(test_x, 1, 2).unwrap();
    /// assert_eq!(predictions.len(), 1);
    /// # }
    /// ```
    pub fn predict(&self, x: Vec<f64>, nrows: usize, ncols: usize) -> Result<Vec<i32>, PandoraError> {
        let model = self
            .model
            .as_ref()
            .ok_or_else(|| PandoraError::config("Model not trained"))?;
        let x_mat = DenseMatrix::new(nrows, ncols, x, false);
        model
            .predict(&x_mat)
            .map_err(|e| PandoraError::config_with_source("Prediction failed", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "ml")]
    fn test_train_predict() {
        let x = vec![0.0, 0.0,
                     1.0, 1.0,
                     2.0, 2.0];
        let y = vec![0, 1, 1];

        let mut predictor = WorldModelPredictor::new(2);
        predictor.train(x.clone(), 3, 2, y).unwrap();

        let predictions = predictor.predict(x, 3, 2).unwrap();
        assert_eq!(predictions.len(), 3usize);
    }
}


