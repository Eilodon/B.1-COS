# Pandora Genesis SDK - Python Bindings

Advanced AI Cognitive Architecture with Python support.

## Installation

```bash
# Install from source
cd python
pip install -e .

# Or build wheel
maturin build --release
pip install target/wheels/pandora_sdk-1.0.0-*.whl
```

## Quick Start

```python
from pandora_sdk import (
    AutomaticScientistOrchestrator,
    InterdependentCausalModel,
    SelfImprovementEngine,
    hello,
    get_version
)

# Basic usage
print(get_version())  # Pandora Genesis SDK v1.0.0
print(hello("World"))  # Hello from Pandora SDK, World!

# Create intelligent agent
scientist = AutomaticScientistOrchestrator()
cwm = InterdependentCausalModel()
sie = SelfImprovementEngine()

# Process events and discover causal relationships
scientist.process_event("agent_picks_key")
cwm.learn_relations("key -> door_unlock")
sie.execute_improvement(1)  # Level 1 refinement
```

## Features

- **Active Inference**: Cognitive architecture for intelligent agents
- **Causal World Modeling**: Graph Neural Network-based causal reasoning
- **Self-Improvement Engine**: 5 levels of adaptive improvement
- **Automatic Discovery**: Scientific hypothesis generation and testing
- **Python Integration**: Native Python bindings with full API access

## Examples

See `examples/main.py` for a complete demonstration.

## Documentation

Full documentation available at: https://pandora-ai.github.io/pandora-genesis-sdk
