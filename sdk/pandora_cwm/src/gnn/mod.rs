//! Graph Neural Network implementation for causal world modeling

pub mod types;
pub mod message_passing;
pub mod layers;

use types::{CausalGraph, GnnConfig, NodeData, CausalEdge};
use pandora_error::PandoraError;
use petgraph::graph::NodeIndex;

/// Graph Neural Network for causal reasoning and world modeling.
pub struct GraphNeuralNetwork {
    /// The underlying causal graph
    pub graph: CausalGraph,
    /// Configuration parameters
    pub config: GnnConfig,
}

impl GraphNeuralNetwork {
    /// Creates a new GraphNeuralNetwork with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the GNN
    ///
    /// # Returns
    ///
    /// * `Result<Self, PandoraError>` - The initialized GNN or an error
    ///
    /// # Examples
    ///
/// ```rust
/// use pandora_cwm::gnn::GraphNeuralNetwork;
/// use pandora_cwm::gnn::types::GnnConfig;
///
/// let config = GnnConfig::new(64, 128, 3);
/// let gnn = GraphNeuralNetwork::new(config)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
    pub fn new(config: GnnConfig) -> Result<Self, PandoraError> {
        config.validate()?;
        
        let graph = CausalGraph::new();
        
        Ok(Self {
            graph,
            config,
        })
    }

    /// Adds a new node to the graph with the given features.
    ///
    /// # Arguments
    ///
    /// * `features` - Node features/embedding
    ///
    /// # Returns
    ///
    /// * `NodeIndex` - Index of the added node
    pub fn add_node(&mut self, features: NodeData) -> NodeIndex {
        self.graph.add_node(features)
    }

    /// Adds a causal edge between two nodes.
    ///
    /// # Arguments
    ///
    /// * `from` - Source node index
    /// * `to` - Target node index
    /// * `edge` - Causal edge information
    ///
    /// # Returns
    ///
    /// * `Result<petgraph::graph::EdgeIndex, PandoraError>` - Edge index or error
    pub fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, edge: CausalEdge) -> Result<petgraph::graph::EdgeIndex, PandoraError> {
        if from.index() >= self.graph.node_count() || to.index() >= self.graph.node_count() {
            return Err(PandoraError::config("Node indices must exist in the graph"));
        }
        
        Ok(self.graph.add_edge(from, to, edge))
    }

    /// Returns the number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Returns the number of edges in the graph.
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// Returns a reference to the underlying graph.
    pub fn graph(&self) -> &CausalGraph {
        &self.graph
    }

    /// Returns a mutable reference to the underlying graph.
    pub fn graph_mut(&mut self) -> &mut CausalGraph {
        &mut self.graph
    }

    /// Gets contextual embedding for an EpistemologicalFlow.
    ///
    /// This method extracts features from the flow and creates an embedding
    /// that represents the current state and context for prediction.
    ///
    /// # Arguments
    ///
    /// * `flow` - The epistemological flow to embed
    ///
    /// # Returns
    ///
    /// * `Result<Vec<f32>, PandoraError>` - The contextual embedding
    pub fn get_contextual_embedding(&self, flow: &pandora_core::ontology::EpistemologicalFlow) -> Result<Vec<f32>, PandoraError> {
        // Simplified implementation: create a basic embedding from flow features
        // In a full implementation, this would use the GNN to process the flow
        
        let mut embedding = vec![0.0; self.config.hidden_dims];
        
        // Extract basic features from the flow
        // This is a placeholder - in reality, we'd use the GNN layers
        // to process the flow's state and create a meaningful embedding
        
        // Simple feature extraction based on flow content
        if let Some(ref sankhara) = flow.sankhara {
            let intent_str = sankhara.as_ref();
            // Hash the intent string to create a simple embedding
            let mut hash = 0u32;
            for byte in intent_str.bytes() {
                hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
            }
            // Normalize to [-1, 1] range
            let normalized = (hash as f32) / (u32::MAX as f32) * 2.0 - 1.0;
            embedding[0] = normalized;
        }
        
        // Add some random noise to simulate more complex features
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for val in embedding.iter_mut().skip(1) {
            *val = rng.gen_range(-0.5..0.5);
        }
        
        Ok(embedding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::CausalEdgeKind;

    #[test]
    fn test_gnn_creation() {
        let config = GnnConfig::new(64, 128, 3);
        let gnn = GraphNeuralNetwork::new(config);
        assert!(gnn.is_ok());
        
        let gnn = gnn.unwrap();
        assert_eq!(gnn.node_count(), 0);
        assert_eq!(gnn.edge_count(), 0);
    }

    #[test]
    fn test_add_node() {
        let config = GnnConfig::new(64, 128, 3);
        let mut gnn = GraphNeuralNetwork::new(config).unwrap();
        
        let features = vec![1.0, 2.0, 3.0];
        let node_idx = gnn.add_node(features);
        
        assert_eq!(gnn.node_count(), 1);
        assert_eq!(gnn.graph[node_idx], vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_add_edge() {
        let config = GnnConfig::new(64, 128, 3);
        let mut gnn = GraphNeuralNetwork::new(config).unwrap();
        
        let node1 = gnn.add_node(vec![1.0, 2.0]);
        let node2 = gnn.add_node(vec![3.0, 4.0]);
        
        let edge = CausalEdge::new(CausalEdgeKind::Cause, 0.9);
        let edge_idx = gnn.add_edge(node1, node2, edge);
        
        assert!(edge_idx.is_ok());
        assert_eq!(gnn.edge_count(), 1);
    }

    #[test]
    fn test_add_edge_invalid_nodes() {
        let config = GnnConfig::new(64, 128, 3);
        let mut gnn = GraphNeuralNetwork::new(config).unwrap();
        
        let node1 = gnn.add_node(vec![1.0, 2.0]);
        let invalid_node = NodeIndex::new(999); // Non-existent node
        
        let edge = CausalEdge::new(CausalEdgeKind::Cause, 0.9);
        let result = gnn.add_edge(node1, invalid_node, edge);
        
        assert!(result.is_err());
    }
}


