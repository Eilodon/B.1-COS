# 🔱 Pandora Genesis SDK

[![Rust CI/CD](https://github.com/OWNER/REPO/actions/workflows/rust.yml/badge.svg)](https://github.com/OWNER/REPO/actions/workflows/rust.yml)
[![Coverage](https://github.com/OWNER/REPO/actions/workflows/coverage.yml/badge.svg)](https://github.com/OWNER/REPO/actions/workflows/coverage.yml)

**Pandora Genesis** is a Software Development Kit (SDK) written in Rust, designed to be the foundation for building Artificial Intelligence systems with recursive self-improvement capabilities.

This project is built on the central thesis: **True intelligence is not knowledge, but mastery in learning how to learn (Meta-Learning Mastery).**

## 🏛️ "Soul-Body-Spirit" SDK Architecture

The SDK is structured according to a clear three-part philosophy:

1. **Soul (`pandora_core`)**: Defines ontological concepts, interfaces (traits), and immutable data structures. This is the philosophical DNA of the system.
2. **Spirit (`pandora_protocols`)**: Uses Protocol Buffers to define a common communication language, allowing different components (written in Rust, Python, Kotlin...) to interact seamlessly.
3. **Body (`pandora_tools`)**: Provides toolkits and sample "skills", which are reference implementations of the concepts in `pandora_core`.

## 🚀 Getting Started

To build the project, make sure you have the Rust toolchain installed:

```bash
cargo build --workspace
```

To run all tests:

```bash
cargo test --workspace --all-features
```

### Running Coverage Report

To generate coverage report:

```bash
./scripts/coverage.sh
```

Or run directly:

```bash
cd sdk
cargo llvm-cov --workspace --all-features --html --output-dir ../coverage
```

### Enable ML Features (Optional)

Modules related to Machine Learning (such as Uncertainty Quantification) are placed behind the `ml` feature flag to allow users to choose, avoiding heavy dependencies when not needed.

To build the SDK with ML features, use the following command:

```bash
cargo build --workspace --features pandora_cwm/ml
```

This will activate the `ml` feature in `pandora_cwm` and other dependent crates like `pandora_mcg`.

### Select Skills with Feature Flags

`pandora_tools` supports feature flags to optionally compile skills for size optimization:

- Default enabled: `arithmetic`, `logical_reasoning`, `information_retrieval`, `pattern_matching`, `analogy_reasoning`.
- Example to enable only 2 skills:

```bash
cargo build -p pandora_tools --no-default-features --features "arithmetic,pattern_matching"
```

## 🧠 Core Concepts

### Free Energy Principle (FEP)
The SDK implements the Free Energy Principle as the foundation for cognitive processing. Every living entity in B.ONE must implement the `FepCell` trait, which defines the "Belief - Perception - Action" loop.

### Skandha Pipeline
The cognitive processing follows the Buddhist concept of the Five Aggregates (Skandhas):
1. **Rupa** (Form): Raw event processing
2. **Vedana** (Feeling): Emotional/moral evaluation
3. **Sanna** (Perception): Pattern recognition and knowledge retrieval
4. **Sankhara** (Mental Formations): Intent formation
5. **Vinnana** (Consciousness): Synthesis and rebirth

### Self-Improvement Loop
The system includes a complete self-improvement pipeline:
- **Learning Engine**: Calculates dual intrinsic rewards (exploration vs. transcendence)
- **Meta-Cognitive Governor (MCG)**: Monitors system state and triggers improvements
- **Self-Improvement Engine (SIE)**: Executes improvement strategies

## 📚 Usage Examples

### Basic FEP Cell
```rust
use pandora_tools::agents::fep_seed::FepSeedRust;
use std::collections::HashMap;

let mut seed = FepSeedRust::new();
let observation = HashMap::from([("is_lit".to_string(), false)]);
let prediction_error = seed.perceive(observation).await;
if let Some(action) = seed.act().await {
    println!("Action: {}", action);
}
```

### Skill Orchestration
```rust
use pandora_orchestrator::{Orchestrator, SkillRegistry};
use pandora_tools::skills::arithmetic_skill::ArithmeticSkill;

let mut registry = SkillRegistry::new();
registry.register(Arc::new(ArithmeticSkill));
let orchestrator = Orchestrator::new(Arc::new(registry));

let result = orchestrator.process_request(
    "arithmetic",
    serde_json::json!({"expression": "2 + 3 * 4"})
).await?;
```

### CLI Demo
Run the interactive CLI demo:

```bash
cargo run --example simple_cli -p pandora_orchestrator
```

## 🏗️ Project Structure

```
sdk/
├── pandora_core/          # Core interfaces and ontology
├── pandora_protocols/     # Protocol Buffers definitions
├── pandora_tools/         # Skills and agents
├── pandora_orchestrator/  # Skill orchestration
├── pandora_cwm/          # Causal World Model
├── pandora_mcg/          # Meta-Cognitive Governor
├── pandora_sie/          # Self-Improvement Engine
├── pandora_learning_engine/ # Learning and rewards
└── pandora_uniffi/       # FFI bindings
```

## 🤝 Contributing

We welcome all contributions. Please follow the code formatting and quality rules by running the following commands before creating a Pull Request:

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test --workspace --all-features
```

## 📄 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## 🔬 Research Background

This SDK is inspired by:
- **Free Energy Principle** (Karl Friston)
- **Active Inference** (Friston et al.)
- **Meta-Learning** and **Learning to Learn**
- **Buddhist Philosophy** (Five Aggregates, Dependent Origination)
- **Recursive Self-Improvement** in AI systems

## 🚧 Status

This project is in active development. The core architecture is stable, but many components are still in placeholder/scaffold state as we build out the full cognitive pipeline.

## 📖 Documentation

- [API Documentation](https://docs.rs/pandora-core) (Coming soon)
- [Architecture Guide](docs/architecture.md) (Coming soon)
- [Contributing Guide](CONTRIBUTING.md)

# 🚩 Observability (Prometheus + Tracing)

Optional and feature-gated to avoid extra system deps by default.

- Prometheus exporter example
  - Requires OpenSSL dev on Linux (Ubuntu/Debian): `sudo apt-get install -y libssl-dev pkg-config`
  - Run exporter (exposes `:9000/metrics`):
    ```bash
    cargo run -p pandora_orchestrator --example monitoring --features pandora_orchestrator/prometheus_export
    ```

- Metrics instrumentation in MCG/SIE
  - Disabled by default. Enable:
    ```bash
    cargo test --features pandora_mcg/metrics_instrumentation,pandora_sie/metrics_instrumentation
    ```
  - Emits counters/histograms via the exporter when enabled.
