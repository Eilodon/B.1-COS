//! Non-Attachment Learning Tests
//!
//! This module tests the agent's ability to adapt and change its policy
//! when the environment changes, demonstrating non-attachment to old strategies.

#[cfg(test)]
use crate::{
    Action, ActiveInferenceSankharaSkandha, LearningEngine, NeuralQValueEstimator,
    ValueDrivenPolicy,
};
#[cfg(test)]
use bytes::Bytes;
#[cfg(test)]
use pandora_core::interfaces::skandhas::SankharaSkandha;
#[cfg(test)]
use pandora_core::ontology::EpistemologicalFlow;
#[cfg(test)]
use pandora_cwm::gnn::types::GnnConfig;
#[cfg(test)]
use pandora_cwm::model::InterdependentCausalModel;
#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use std::sync::{Arc, Mutex};

/// Test non-attachment learning with environment change
/// Initially action A is optimal, after 100 cycles action B becomes optimal
#[cfg(test)]
#[test]
fn test_non_attachment_learning_environment_change() {
    println!("🔄 Testing non-attachment learning with environment change");

    let config = GnnConfig::new(96, 192, 4);
    let cwm = Arc::new(Mutex::new(InterdependentCausalModel::new(config).unwrap()));
    let learning_engine = Arc::new(LearningEngine::new(0.8, 0.2)); // Higher learning rate for faster adaptation

    let available_actions = vec!["action_a", "action_b", "action_c", "action_d"];

    // Create Q-value estimator with tracking
    let _q_estimator = NeuralQValueEstimator::new(0.15, 0.9);
    let mut policy = ValueDrivenPolicy::new(0.15, 0.9, 0.3); // Higher exploration initially

    let sankhara = ActiveInferenceSankharaSkandha::new(
        cwm.clone(),
        learning_engine.clone(),
        4,
        available_actions,
        0.9,
        0.2,
    );

    // Track action selection and performance
    let mut action_counts = HashMap::new();
    let mut performance_history = Vec::new();
    let mut adaptation_metrics = Vec::new();

    println!("🎯 Scenario: Environment changes at cycle 100");
    println!("   Phase 1 (cycles 1-100): Action A is optimal (reward = 1.0)");
    println!("   Phase 2 (cycles 101-200): Action B becomes optimal (reward = 1.0)");
    println!("   Other actions: Always give reward = 0.1");

    let mut flow = EpistemologicalFlow::from_bytes(Bytes::from_static(b"environment_change_test"));

    for cycle in 1..=200 {
        // Determine current environment phase
        let is_phase_1 = cycle <= 100;
        let environment_phase = if is_phase_1 { "Phase 1" } else { "Phase 2" };

        if cycle % 25 == 0 {
            println!(
                "\n🔄 Cycle {} ({}): Testing adaptation",
                cycle, environment_phase
            );
        }

        // Let the system plan and select action
        sankhara.form_intent(&mut flow);

        let selected_action = flow
            .sankhara
            .as_ref()
            .map(|s| s.as_ref())
            .unwrap_or("action_a");

        // Count action selections
        *action_counts
            .entry(selected_action.to_string())
            .or_insert(0) += 1;

        // Simulate environment response based on current phase
        let reward = if is_phase_1 {
            // Phase 1: Action A is optimal
            if selected_action == "action_a" {
                1.0
            } else {
                0.1
            }
        } else {
            // Phase 2: Action B becomes optimal
            if selected_action == "action_b" {
                1.0
            } else {
                0.1
            }
        };

        // Update Q-values based on reward
        let next_flow = flow.clone();
        let action_enum = match selected_action {
            "action_a" => Action::Noop, // Simplified mapping
            "action_b" => Action::Explore,
            "action_c" => Action::Exploit,
            _ => Action::Noop,
        };
        policy.update_with_experience(&flow, &action_enum, reward, &next_flow);

        // Record performance
        performance_history.push((cycle, selected_action.to_string(), reward));

        // Calculate adaptation metrics every 10 cycles
        if cycle % 10 == 0 {
            let recent_actions: Vec<&String> = performance_history
                .iter()
                .rev()
                .take(10)
                .map(|(_, action, _)| action)
                .collect();

            let action_a_ratio =
                recent_actions.iter().filter(|&&a| a == "action_a").count() as f64 / 10.0;
            let action_b_ratio =
                recent_actions.iter().filter(|&&a| a == "action_b").count() as f64 / 10.0;

            adaptation_metrics.push((cycle, action_a_ratio, action_b_ratio));

            if cycle % 25 == 0 {
                println!(
                    "   Recent action distribution: A={:.1}%, B={:.1}%",
                    action_a_ratio * 100.0,
                    action_b_ratio * 100.0
                );
            }
        }

        // Check for adaptation milestones
        if cycle == 100 {
            println!("   🔄 ENVIRONMENT CHANGE: Action B now optimal, Action A suboptimal");
        }

        if cycle == 150 {
            let recent_b_ratio = performance_history
                .iter()
                .rev()
                .take(50)
                .filter(|(_, action, _)| action == "action_b")
                .count() as f64
                / 50.0;

            println!(
                "   📊 Mid-adaptation: Action B ratio = {:.1}%",
                recent_b_ratio * 100.0
            );
        }
    }

    // Analyze adaptation performance
    println!("\n📊 Non-Attachment Learning Analysis:");

    // Overall action distribution
    let total_actions: i32 = action_counts.values().sum();
    println!("   Total actions taken: {}", total_actions);
    for (action, count) in &action_counts {
        let percentage = (*count as f64 / total_actions as f64) * 100.0;
        println!("   {}: {} ({:.1}%)", action, count, percentage);
    }

    // Phase-specific analysis
    let phase_1_actions: Vec<_> = performance_history
        .iter()
        .filter(|(cycle, _, _)| *cycle <= 100)
        .map(|(_, action, _)| action)
        .collect();

    let phase_2_actions: Vec<_> = performance_history
        .iter()
        .filter(|(cycle, _, _)| *cycle > 100)
        .map(|(_, action, _)| action)
        .collect();

    let phase_1_a_ratio = phase_1_actions.iter().filter(|&&a| a == "action_a").count() as f64
        / phase_1_actions.len() as f64;
    let phase_2_b_ratio = phase_2_actions.iter().filter(|&&a| a == "action_b").count() as f64
        / phase_2_actions.len() as f64;

    println!(
        "   Phase 1 (cycles 1-100): Action A ratio = {:.1}%",
        phase_1_a_ratio * 100.0
    );
    println!(
        "   Phase 2 (cycles 101-200): Action B ratio = {:.1}%",
        phase_2_b_ratio * 100.0
    );

    // Adaptation speed analysis
    let adaptation_cycles = adaptation_metrics
        .iter()
        .find(|(_, _, b_ratio)| *b_ratio > 0.5)
        .map(|(cycle, _, _)| *cycle)
        .unwrap_or(200);

    println!(
        "   Adaptation speed: {} cycles to reach 50% Action B",
        adaptation_cycles
    );

    // Performance analysis
    let phase_1_avg_reward: f64 = performance_history
        .iter()
        .filter(|(cycle, _, _)| *cycle <= 100)
        .map(|(_, _, reward)| *reward)
        .sum::<f64>()
        / 100.0;

    let phase_2_avg_reward: f64 = performance_history
        .iter()
        .filter(|(cycle, _, _)| *cycle > 100)
        .map(|(_, _, reward)| *reward)
        .sum::<f64>()
        / 100.0;

    println!("   Phase 1 average reward: {:.3}", phase_1_avg_reward);
    println!("   Phase 2 average reward: {:.3}", phase_2_avg_reward);

    // Assertions for successful adaptation
    assert!(
        phase_1_a_ratio > 0.3,
        "Agent should prefer Action A in Phase 1"
    );
    assert!(
        phase_2_b_ratio > 0.3,
        "Agent should adapt to Action B in Phase 2"
    );
    assert!(
        phase_2_avg_reward > 0.5,
        "Agent should maintain good performance in Phase 2"
    );
    assert!(
        adaptation_cycles < 150,
        "Agent should adapt within 50 cycles of environment change"
    );

    println!("✅ Non-attachment learning test passed!");
    println!("   Agent successfully adapted from Action A to Action B");
    println!("   Adaptation speed: {} cycles", adaptation_cycles);
}

/// Test gradual environment change (not abrupt)
#[cfg(test)]
#[test]
fn test_gradual_environment_change() {
    println!("🌊 Testing gradual environment change adaptation");

    let config = GnnConfig::new(64, 128, 3);
    let cwm = Arc::new(Mutex::new(InterdependentCausalModel::new(config).unwrap()));
    let learning_engine = Arc::new(LearningEngine::new(0.6, 0.4));

    let available_actions = vec![
        "conservative_action",
        "moderate_action",
        "aggressive_action",
    ];

    let _q_estimator = NeuralQValueEstimator::new(0.1, 0.9);
    let mut policy = ValueDrivenPolicy::new(0.1, 0.9, 0.25);

    let sankhara =
        ActiveInferenceSankharaSkandha::new(cwm, learning_engine, 3, available_actions, 0.9, 0.25);

    let mut flow = EpistemologicalFlow::from_bytes(Bytes::from_static(b"gradual_change_test"));
    let mut performance_history = Vec::new();

    println!("🎯 Scenario: Gradual shift from conservative to aggressive optimal action");
    println!("   Cycles 1-50: Conservative action optimal");
    println!("   Cycles 51-100: Transition period (mixed optimality)");
    println!("   Cycles 101-150: Aggressive action optimal");

    for cycle in 1..=150 {
        sankhara.form_intent(&mut flow);

        let selected_action = flow
            .sankhara
            .as_ref()
            .map(|s| s.as_ref())
            .unwrap_or("conservative_action");

        // Calculate reward based on gradual change
        let reward = if cycle <= 50 {
            // Conservative phase
            if selected_action == "conservative_action" {
                1.0
            } else {
                0.3
            }
        } else if cycle <= 100 {
            // Transition phase - gradual shift
            let transition_factor = (cycle - 50) as f64 / 50.0;
            if selected_action == "conservative_action" {
                1.0 - transition_factor * 0.5
            } else if selected_action == "aggressive_action" {
                0.3 + transition_factor * 0.5
            } else {
                0.5
            }
        } else {
            // Aggressive phase
            if selected_action == "aggressive_action" {
                1.0
            } else {
                0.3
            }
        };

        // Update policy
        let next_flow = flow.clone();
        let action_enum = match selected_action {
            "conservative_action" => Action::Noop,
            "moderate_action" => Action::Explore,
            "aggressive_action" => Action::Exploit,
            _ => Action::Noop,
        };
        policy.update_with_experience(&flow, &action_enum, reward, &next_flow);

        performance_history.push((cycle, selected_action.to_string(), reward));

        if cycle % 25 == 0 {
            let recent_reward: f64 = performance_history
                .iter()
                .rev()
                .take(25)
                .map(|(_, _, reward)| *reward)
                .sum::<f64>()
                / 25.0;

            println!(
                "   Cycle {}: Recent avg reward = {:.3}",
                cycle, recent_reward
            );
        }
    }

    // Analyze gradual adaptation
    let early_performance: f64 = performance_history
        .iter()
        .filter(|(cycle, _, _)| *cycle <= 50)
        .map(|(_, _, reward)| *reward)
        .sum::<f64>()
        / 50.0;

    let late_performance: f64 = performance_history
        .iter()
        .filter(|(cycle, _, _)| *cycle > 100)
        .map(|(_, _, reward)| *reward)
        .sum::<f64>()
        / 50.0;

    println!("📊 Gradual Adaptation Analysis:");
    println!(
        "   Early performance (cycles 1-50): {:.3}",
        early_performance
    );
    println!(
        "   Late performance (cycles 101-150): {:.3}",
        late_performance
    );
    println!("   Performance maintained: {}", late_performance > 0.7);

    assert!(
        late_performance > 0.7,
        "Agent should maintain good performance after gradual change"
    );

    println!("✅ Gradual environment change test passed!");
}

/// Test catastrophic environment change (sudden shift)
#[cfg(test)]
#[test]
fn test_catastrophic_environment_change() {
    println!("💥 Testing catastrophic environment change adaptation");

    let config = GnnConfig::new(128, 256, 5);
    let cwm = Arc::new(Mutex::new(InterdependentCausalModel::new(config).unwrap()));
    let learning_engine = Arc::new(LearningEngine::new(0.9, 0.1)); // Very high learning rate for rapid adaptation

    let available_actions = vec!["old_strategy", "new_strategy", "explore", "wait"];

    let _q_estimator = NeuralQValueEstimator::new(0.2, 0.9);
    let mut policy = ValueDrivenPolicy::new(0.2, 0.9, 0.4); // High exploration for catastrophic change

    let sankhara =
        ActiveInferenceSankharaSkandha::new(cwm, learning_engine, 5, available_actions, 0.9, 0.4);

    let mut flow = EpistemologicalFlow::from_bytes(Bytes::from_static(b"catastrophic_change_test"));
    let mut performance_history = Vec::new();

    println!("🎯 Scenario: Catastrophic change at cycle 100");
    println!("   Cycles 1-100: Old strategy optimal (reward = 1.0)");
    println!("   Cycles 101+: Old strategy catastrophic (reward = -1.0)");
    println!("   New strategy: Always gives reward = 0.8");

    for cycle in 1..=200 {
        sankhara.form_intent(&mut flow);

        let selected_action = flow
            .sankhara
            .as_ref()
            .map(|s| s.as_ref())
            .unwrap_or("old_strategy");

        // Simulate catastrophic change
        let reward = if cycle <= 100 {
            // Before change: old strategy is optimal
            if selected_action == "old_strategy" {
                1.0
            } else {
                0.2
            }
        } else {
            // After change: old strategy is catastrophic
            if selected_action == "old_strategy" {
                -1.0
            } else if selected_action == "new_strategy" {
                0.8
            } else {
                0.1
            }
        };

        // Update policy
        let next_flow = flow.clone();
        let action_enum = match selected_action {
            "old_strategy" => Action::Noop,
            "new_strategy" => Action::Explore,
            "explore" => Action::Exploit,
            "wait" => Action::Noop,
            _ => Action::Noop,
        };
        policy.update_with_experience(&flow, &action_enum, reward, &next_flow);

        performance_history.push((cycle, selected_action.to_string(), reward));

        if cycle == 100 {
            println!("   💥 CATASTROPHIC CHANGE: Old strategy now gives negative reward!");
        }

        if cycle % 25 == 0 && cycle > 100 {
            let recent_reward: f64 = performance_history
                .iter()
                .rev()
                .take(25)
                .map(|(_, _, reward)| *reward)
                .sum::<f64>()
                / 25.0;

            let old_strategy_usage = performance_history
                .iter()
                .rev()
                .take(25)
                .filter(|(_, action, _)| action == "old_strategy")
                .count() as f64
                / 25.0;

            println!(
                "   Cycle {}: Recent avg reward = {:.3}, Old strategy usage = {:.1}%",
                cycle,
                recent_reward,
                old_strategy_usage * 100.0
            );
        }
    }

    // Analyze catastrophic adaptation
    let pre_change_performance: f64 = performance_history
        .iter()
        .filter(|(cycle, _, _)| *cycle <= 100)
        .map(|(_, _, reward)| *reward)
        .sum::<f64>()
        / 100.0;

    let post_change_performance: f64 = performance_history
        .iter()
        .filter(|(cycle, _, _)| *cycle > 100)
        .map(|(_, _, reward)| *reward)
        .sum::<f64>()
        / 100.0;

    let final_old_strategy_usage = performance_history
        .iter()
        .rev()
        .take(50)
        .filter(|(_, action, _)| action == "old_strategy")
        .count() as f64
        / 50.0;

    println!("📊 Catastrophic Adaptation Analysis:");
    println!("   Pre-change performance: {:.3}", pre_change_performance);
    println!("   Post-change performance: {:.3}", post_change_performance);
    println!(
        "   Final old strategy usage: {:.1}%",
        final_old_strategy_usage * 100.0
    );
    println!(
        "   Successfully abandoned old strategy: {}",
        final_old_strategy_usage < 0.2
    );

    assert!(
        post_change_performance > 0.0,
        "Agent should avoid catastrophic negative rewards"
    );
    assert!(
        final_old_strategy_usage < 0.3,
        "Agent should largely abandon the old strategy"
    );

    println!("✅ Catastrophic environment change test passed!");
    println!("   Agent successfully adapted despite catastrophic change");
}
