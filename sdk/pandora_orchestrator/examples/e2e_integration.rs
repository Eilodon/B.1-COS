use pandora_core::fep_cell::SkandhaProcessor;
use pandora_core::skandha_implementations::basic_skandhas::*;
use pandora_learning_engine::{LearningEngine, ExponentialMovingAverageEstimator, ExperienceBuffer, Policy, EpsilonGreedyPolicy};
use pandora_learning_engine::world_models::SimpleWorldModel;
use pandora_mcg::enhanced_mcg::{EnhancedMetaCognitiveGovernor, SystemMetrics, ResourceMetrics, ActionTrigger as EnhancedTrigger};
use pandora_mcg::ActionTrigger as RootTrigger;
use pandora_sie::SelfImprovementEngine;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[cfg(feature = "ml")]
use pandora_cwm::ml::predictor::WorldModelPredictor;
#[cfg(feature = "ml")]
use pandora_cwm::nn::uq_models::ProbabilisticOutput;
#[cfg(feature = "ml")]
use ndarray::Array2;
#[cfg(any(feature = "ml", feature = "prometheus_export"))]
use std::time::Instant;

#[cfg(feature = "prometheus_export")]
use metrics::{counter, histogram};

// Note: Using direct metrics macros instead of lazy_static for simplicity

#[tokio::main(flavor = "multi_thread")] 
async fn main() {
    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();
    let _ = tracing::subscriber::set_global_default(subscriber);

    info!("E2E: Wiring Skandha → LearningEngine → MCG → SIE");

    // 1) Core pipeline
    let processor = SkandhaProcessor::new(
        Box::new(BasicRupaSkandha),
        Box::new(BasicVedanaSkandha),
        Box::new(BasicSannaSkandha),
        Box::new(BasicSankharaSkandha),
        Box::new(BasicVinnanaSkandha),
    );

    // 2) Learning engine
    let le = LearningEngine::new(0.7, 0.3);
    let current_model = SimpleWorldModel::new("baseline".into(), 5.0, 0.8);
    let new_model = current_model.evolve(3.5, 0.85);
    let mut ema = ExponentialMovingAverageEstimator::new(0.5);
    let mut buffer = ExperienceBuffer::with_capacity(32);
    let mut policy = EpsilonGreedyPolicy::default();

    // 3) Meta-cognitive governor + SIE
    let mut mcg = EnhancedMetaCognitiveGovernor::new();
    let sie = SelfImprovementEngine::new();

    // Simulate one end-to-end cycle
    #[cfg(feature = "prometheus_export")]
    let cycle_start = Instant::now();
    
    let flow_event = b"error: service latency spike".to_vec();
    let flow = processor.run_epistemological_cycle(flow_event).expect("reborn event");
    let flow = pandora_core::ontology::EpistemologicalFlow::from_bytes(bytes::Bytes::from(flow));

    // Learning step
    let (reward, advantage) = le.learn_single_step(&current_model, &new_model, &flow, &mut ema, &mut buffer);
    info!(?reward, %advantage, "LE: reward + advantage");
    
    #[cfg(feature = "prometheus_export")]
    {
        let total_reward = le.get_total_weighted_reward(&reward);
        histogram!("learning_rewards").record(total_reward);
    }

    // MCG monitoring (mocked system metrics)
    let metrics = SystemMetrics {
        uncertainty: 0.8,
        compression_reward: reward.compression_reward,
        novelty_score: 0.65,
        performance: 0.55,
        resource_usage: ResourceMetrics { cpu_usage: 0.45, memory_usage: 0.50, latency_ms: 12.0 },
    };
    let decision = mcg.monitor_comprehensive(&metrics);
    info!(?decision, "MCG: decision");
    
    #[cfg(feature = "prometheus_export")]
    {
        let decision_type = match &decision.decision {
            EnhancedTrigger::TriggerSelfImprovementLevel1 { .. } => "level1",
            EnhancedTrigger::TriggerSelfImprovementLevel2 { .. } => "level2", 
            EnhancedTrigger::TriggerSelfImprovementLevel3 { .. } => "level3",
            EnhancedTrigger::RequestMoreInformation { .. } => "request_info",
            EnhancedTrigger::OptimizeResources { .. } => "optimize",
            EnhancedTrigger::NoAction => "no_action",
        };
        counter!("mcg_decisions_total", "decision_type" => decision_type).increment(1);
    }

    // Route decision to SIE
    let root_trigger: Option<RootTrigger> = match &decision.decision {
        EnhancedTrigger::TriggerSelfImprovementLevel1 { reason, target_component, .. } =>
            Some(RootTrigger::TriggerSelfImprovementLevel1 { reason: reason.clone(), target_component: target_component.clone() }),
        EnhancedTrigger::TriggerSelfImprovementLevel2 { reason, target_component, .. } =>
            Some(RootTrigger::TriggerSelfImprovementLevel2 { reason: reason.clone(), target_component: target_component.clone() }),
        EnhancedTrigger::TriggerSelfImprovementLevel3 { reason, target_component, .. } =>
            Some(RootTrigger::TriggerSelfImprovementLevel3 { reason: reason.clone(), target_component: target_component.clone() }),
        EnhancedTrigger::RequestMoreInformation { .. }
        | EnhancedTrigger::OptimizeResources { .. }
        | EnhancedTrigger::ProposeCausalHypothesis { .. }
        | EnhancedTrigger::NoAction => None,
    };

    if let Some(rt) = root_trigger {
        let action = sie.execute(&rt).await.expect("sie action");
        info!(%action.description, "SIE: executed");
        
        #[cfg(feature = "prometheus_export")]
        {
            let strategy = match rt {
                RootTrigger::TriggerSelfImprovementLevel1 { .. } => "refinement",
                RootTrigger::TriggerSelfImprovementLevel2 { .. } => "architecture_search",
                RootTrigger::TriggerSelfImprovementLevel3 { .. } => "code_generation",
                RootTrigger::TriggerSelfImprovementLevel4 { .. } => "meta_learning",
                _ => "unknown",
            };
            counter!("sie_actions_total", "strategy" => strategy).increment(1);
        }
    }

    // Policy update (no-op)
    policy.update(&flow, advantage);

    // GNN ITR-NN Integration Demo
    #[cfg(feature = "ml")]
    {
        info!("--- GNN ITR-NN Integration Demo ---");
        let gnn_start = Instant::now();
        
        // Mock graph data: 3 nodes, 2 features each
        let node_features = Array2::from_shape_vec((3, 2), vec![1.0, 0.5, 0.8, 0.3, 0.2, 0.9]).unwrap();
        let edge_indices = vec![(0, 1), (1, 2), (2, 0)]; // Triangle graph
        
        info!("GNN: Processing graph with {} nodes, {} edges", node_features.nrows(), edge_indices.len());
        
        // Simulate GNN processing (message passing)
        let mut aggregated_features = vec![0.0; node_features.ncols()];
        for (i, j) in edge_indices {
            for k in 0..node_features.ncols() {
                aggregated_features[k] += (node_features[[i, k]] + node_features[[j, k]]) / 2.0;
            }
        }
        
        let _gnn_duration = gnn_start.elapsed().as_secs_f64();
        info!("GNN: Aggregated features: {:?}", aggregated_features);
        
        #[cfg(feature = "prometheus_export")]
        histogram!("gnn_processing_duration_seconds").record(_gnn_duration);
    }

    // UQ Predictor Demo
    #[cfg(feature = "ml")]
    {
        info!("--- UQ Predictor Demo ---");
        
        // Create mock training data
        let x_train = Array2::from_shape_vec((10, 2), (0..20).map(|i| (i as f64) * 0.1).collect()).unwrap();
        let y_train: Vec<i32> = (0..10).map(|i| if i % 2 == 0 { 1 } else { 0 }).collect();
        
        // Convert to flat vectors for predictor API
        let x_flat: Vec<f64> = x_train.iter().cloned().collect();
        let mut predictor = WorldModelPredictor::new(2); // 2 features
        predictor.train(x_flat, 10, 2, y_train).expect("Training failed");
        
        // Make prediction with uncertainty
        let x_test = Array2::from_shape_vec((1, 2), vec![0.5, 0.7]).unwrap();
        let x_test_flat: Vec<f64> = x_test.iter().cloned().collect();
        let prediction = predictor.predict(x_test_flat, 1, 2).expect("Prediction failed");
        
        info!("UQ Predictor: Prediction = {:?}", prediction);
        
        // Create probabilistic output with uncertainty
        let uq_output = ProbabilisticOutput::new(0.15); // High epistemic uncertainty
        
        let variance_scalar: f32 = uq_output.variance.to_scalar().unwrap_or(0.0);
        let confidence_level = if variance_scalar < 0.1 { "high" } 
                              else if variance_scalar < 0.3 { "medium" } 
                              else { "low" };
        
        info!("UQ Output: Variance = {:.3}, Confidence = {}", 
              variance_scalar, confidence_level);
        
        #[cfg(feature = "prometheus_export")]
        counter!("uq_predictions_total", "confidence_level" => confidence_level).increment(1);
    }

    #[cfg(feature = "prometheus_export")]
    {
        let cycle_duration = cycle_start.elapsed().as_secs_f64();
        histogram!("e2e_cycle_duration_seconds").record(cycle_duration);
        counter!("e2e_cycles_total").increment(1);
    }

    info!("E2E: Completed one integrated cycle with GNN + UQ.");
}


