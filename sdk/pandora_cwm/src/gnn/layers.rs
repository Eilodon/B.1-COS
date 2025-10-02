#[cfg(feature = "ml")]
use ndarray::{Array2};

/// A basic Graph Convolution Layer for Graph Neural Networks.
///
/// This layer performs message passing followed by a linear transformation,
/// implementing a simple form of graph convolution.
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "ml")]
/// # {
/// use pandora_cwm::gnn::layers::GraphConvLayer;
/// use ndarray::arr2;
///
/// let weight = arr2(&[[1.0, 0.0], [0.0, 1.0]]);
/// let layer = GraphConvLayer::new(weight);
/// let adj = arr2(&[[0.0, 1.0], [1.0, 0.0]]);
/// let features = arr2(&[[1.0, 0.0], [0.0, 1.0]]);
/// let output = layer.forward(&adj, &features);
/// assert_eq!(output.shape(), &[2, 2]);
/// # }
/// ```
#[cfg(feature = "ml")]
pub struct GraphConvLayer {
    pub weight: Array2<f32>, // (in_dim, out_dim)
}

#[cfg(feature = "ml")]
impl GraphConvLayer {
    /// Creates a new GraphConvLayer with the given weight matrix.
    ///
    /// # Arguments
    ///
    /// * `weight` - Weight matrix of shape (in_dim, out_dim)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "ml")]
    /// # {
    /// use pandora_cwm::gnn::layers::GraphConvLayer;
    /// use ndarray::arr2;
    /// let weight = arr2(&[[1.0, 0.0], [0.0, 1.0]]);
    /// let layer = GraphConvLayer::new(weight);
    /// # }
    /// ```
    pub fn new(weight: Array2<f32>) -> Self { Self { weight } }

    /// Performs forward pass through the graph convolution layer.
    ///
    /// # Arguments
    ///
    /// * `adj` - Adjacency matrix (n x n)
    /// * `x` - Input node features (n x in_dim)
    ///
    /// # Returns
    ///
    /// * `Array2<f32>` - Output features (n x out_dim)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "ml")]
    /// # {
    /// use pandora_cwm::gnn::layers::GraphConvLayer;
    /// use ndarray::arr2;
    /// let weight = arr2(&[[1.0, 0.0], [0.0, 1.0]]);
    /// let layer = GraphConvLayer::new(weight);
    /// let adj = arr2(&[[0.0, 1.0], [1.0, 0.0]]);
    /// let x = arr2(&[[1.0, 0.0], [0.0, 1.0]]);
    /// let output = layer.forward(&adj, &x);
    /// # }
    /// ```
    pub fn forward(&self, adj: &Array2<f32>, x: &Array2<f32>) -> Array2<f32> {
        // simple: mean aggregate then linear
        let agg = crate::gnn::message_passing::aggregate_mean(adj, x);
        agg.dot(&self.weight)
    }
}


