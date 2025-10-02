# Pandora Genesis SDK — Architecture Overview

## High-level Components

- `pandora_core`: Ontology and traits (FEPCell, Skandhas, WorldModel)
- `pandora_tools`: Skills and agents
- `pandora_orchestrator`: Skill orchestration, circuit breaker
- `pandora_cwm`: Causal World Model (VSA/NN/TDA)
- `pandora_learning_engine`: Rewards and learning loop
- `pandora_mcg`: Meta-Cognitive Governor
- `pandora_sie`: Self-Improvement Engine

## Cognitive Pipeline (Skandhas)

```mermaid
flowchart LR
  A[Rupa] --> B[Vedana]
  B --> C[Sanna]
  C --> D[Sankhara]
  D --> E[Vinnana]
```

## Self-Improvement Loop

```mermaid
sequenceDiagram
  participant Tools
  participant Orchestrator
  participant LE as Learning Engine
  participant MCG as Meta-Cognitive Governor
  participant SIE as Self-Improvement Engine

  Tools->>Orchestrator: Process
  Orchestrator->>LE: Report flow + models
  LE-->>Orchestrator: DualIntrinsicReward
  Orchestrator->>MCG: Metrics/State
  MCG-->>Orchestrator: ActionTrigger(L1-L4)
  Orchestrator->>SIE: Execute(strategy)
  SIE-->>Orchestrator: ImprovementAction
```

## Observability

- Tracing spans on critical paths (SIE/MCG)
- Optional Prometheus exporter (example `monitoring`): `:9000/metrics`

## Build Profiles & Features

- `pandora_cwm/ml`: ML stack
- `pandora_mcg/metrics_instrumentation`, `pandora_sie/metrics_instrumentation`
- `pandora_orchestrator/prometheus_export` (example only)


