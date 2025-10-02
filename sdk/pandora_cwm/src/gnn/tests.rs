#[cfg(test)]
#[cfg(feature = "ml")]
mod tests {
    use ndarray::{arr2};
    use crate::gnn::{message_passing::aggregate_mean, layers::GraphConvLayer};

    #[test]
    fn test_aggregate_mean_self_loop_fallback() {
        let adj = arr2(&[[0.0, 1.0], [0.0, 0.0]]);
        let x = arr2(&[[1.0, 0.0], [0.0, 1.0]]);
        let out = aggregate_mean(&adj, &x);
        assert_eq!(out.shape(), &[2, 2]);
        // node 0 aggregates node 1 -> [0,1]
        assert_eq!(out[[0,1]], 1.0);
        // node 1 falls back to its own features -> [0,1]
        assert_eq!(out[[1,1]], 1.0);
    }

    #[test]
    fn test_graph_conv_linear() {
        let adj = arr2(&[[0.0, 1.0], [1.0, 0.0]]);
        let x = arr2(&[[1.0, 2.0], [3.0, 4.0]]);
        let w = arr2(&[[1.0, 0.0],[0.0, 1.0]]);
        let layer = GraphConvLayer::new(w);
        let y = layer.forward(&adj, &x);
        assert_eq!(y.shape(), &[2, 2]);
    }
}


