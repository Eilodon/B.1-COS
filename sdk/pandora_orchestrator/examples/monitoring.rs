use pandora_orchestrator::{Orchestrator, SkillRegistry};
use pandora_orchestrator::OrchestratorTrait;
use pandora_tools::skills::arithmetic_skill::ArithmeticSkill;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Setup orchestrator
    let mut registry = SkillRegistry::new();
    registry.register(Arc::new(ArithmeticSkill));
    
    let orchestrator = Arc::new(Orchestrator::new(Arc::new(registry)));
    
    // Start cleanup task
    let _cleanup_handle = orchestrator.clone().start_cleanup_task();

    // Simulate load
    println!("🚀 Starting load simulation...");
    
    for i in 0..100 {
        let orch = orchestrator.clone();
        tokio::spawn(async move {
            let input = serde_json::json!({"expression": format!("{} + {}", i, i)});
            let _ = orch.process_request("arithmetic", input).await;
        });
    }

    // Monitor stats periodically
    for _ in 0..10 {
        sleep(Duration::from_secs(2)).await;
        let stats = orchestrator.circuit_stats();
        println!(
            "📊 Circuits: {} total | {} closed | {} open | {} half-open | {}/{} capacity",
            stats.total_circuits,
            stats.closed,
            stats.open,
            stats.half_open,
            stats.total_circuits,
            stats.capacity
        );
    }

    Ok(())
}


