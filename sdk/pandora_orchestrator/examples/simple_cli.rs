use pandora_orchestrator::{Orchestrator, OrchestratorTrait, SkillRegistry};
use pandora_tools::skills::{
    analogy_reasoning_skill::AnalogyReasoningSkill,
    // Temporarily disabled due to dependency conflicts
    // information_retrieval_skill::InformationRetrievalSkill,
    arithmetic_skill::ArithmeticSkill,
    logical_reasoning_skill::LogicalReasoningSkill,
    pattern_matching_skill::PatternMatchingSkill,
};
use std::io::{self, Write};
use tracing_subscriber::{fmt, EnvFilter};
use std::sync::Arc;

fn init_logging() {
    let filter = EnvFilter::from_default_env()
        .add_directive("pandora_core=info".parse().unwrap())
        .add_directive("pandora_simulation=info".parse().unwrap())
        .add_directive("pandora_orchestrator=info".parse().unwrap());

    fmt().with_env_filter(filter).init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    println!("🔱 Pandora Genesis SDK - CLI Demo");
    println!("=====================================\n");

    // Khởi tạo Skill Registry
    let mut registry = SkillRegistry::new();

    // Đăng ký các skills
    registry.register(Arc::new(ArithmeticSkill));
    registry.register(Arc::new(LogicalReasoningSkill));
    registry.register(Arc::new(PatternMatchingSkill));
    registry.register(Arc::new(AnalogyReasoningSkill));
    // Temporarily disabled due to dependency conflicts
    // registry.register(Arc::new(InformationRetrievalSkill));

    let orchestrator = Orchestrator::new(Arc::new(registry));

    println!("Available skills:");
    println!("- arithmetic: Perform arithmetic calculations");
    println!("- logical_reasoning: Evaluate logical expressions");
    println!("- pattern_matching: Match patterns in strings");
    println!("- analogy_reasoning: Solve analogy problems");
    // Temporarily disabled due to dependency conflicts
    // println!("- information_retrieval: Search in text documents");
    println!("\nType 'help' for examples, 'quit' to exit.\n");

    loop {
        print!("pandora> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "quit" {
            println!("Goodbye! 👋");
            break;
        }

        if input == "help" {
            show_help();
            continue;
        }

        if input.is_empty() {
            continue;
        }

        // Parse command: skill_name input_json
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        if parts.len() != 2 {
            println!("❌ Usage: <skill_name> <json_input>");
            println!("   Example: arithmetic '{{\"expression\": \"2 + 2\"}}'");
            continue;
        }

        let skill_name = parts[0];
        let json_input = parts[1];

        // Parse JSON input
        let input_value: serde_json::Value = match serde_json::from_str(json_input) {
            Ok(value) => value,
            Err(e) => {
                println!("❌ Invalid JSON: {}", e);
                continue;
            }
        };

        // Execute skill
        match orchestrator.process_request(skill_name, input_value).await {
            Ok(result) => {
                println!("✅ Result: {}", serde_json::to_string_pretty(&result)?);
            }
            Err(e) => {
                println!("❌ Error: {}", e);
            }
        }
        println!();
    }

    Ok(())
}

fn show_help() {
    println!("\n📖 Examples:");
    println!("arithmetic '{{\"expression\": \"2 + 3 * 4\"}}'");
    println!("logical_reasoning '{{\"ast\": {{\"type\": \"AND\", \"children\": [{{\"type\": \"CONST\", \"value\": true}}, {{\"type\": \"CONST\", \"value\": false}}]}}, \"context\": {{}}}}'");
    println!("pattern_matching '{{\"pattern\": \"a*b\", \"candidates\": [\"ab\", \"aab\", \"b\", \"acb\"]}}'");
    println!("analogy_reasoning '{{\"a\": \"man\", \"b\": \"king\", \"c\": \"woman\", \"candidates\": [\"queen\", \"prince\", \"duke\"]}}'");
    // Temporarily disabled due to dependency conflicts
    // println!("information_retrieval '{{\"query\": \"test\", \"documents\": [\"test document\", \"another doc\", \"test again\"]}}'");
    println!();
}
