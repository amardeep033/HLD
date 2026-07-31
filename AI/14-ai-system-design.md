# AI System Design

## Why this matters in discussions

System design is where backend experience becomes a major advantage. The goal is to show architecture, tradeoffs, and operational maturity.

## Definition

AI system design combines application services, models, retrieval, tools, evaluation, observability, security, and deployment.

## Mental Model / Analogy

An AI product is a distributed system where one dependency speaks natural language and may be wrong.

## Architecture Diagram (ASCII)

```text
Client
  |
API Gateway
  |
AI Orchestrator
  |-- Prompt Store
  |-- Retrieval
  |-- Tool Layer
  |-- Guardrails
  |-- Observability
  |
Model Provider / Model Serving
```

## How it works

The orchestrator receives user input, loads prompt config, retrieves context if needed, calls tools if needed, invokes the model, validates output, records telemetry, and returns a response.

## Backend Engineer Perspective

Always discuss SLOs, rate limits, data permissions, cost controls, evals, model versioning, fallback, and incident response.

## Real-world Example

### AI Chat Application

```text
Web App -> Chat API -> Conversation Store -> LLM -> Streamed Reply
```

### RAG System

```text
Docs -> Chunker -> Embeddings -> Vector DB
User -> Retriever -> LLM -> Answer + Sources
```

### AI Coding Assistant

```text
IDE -> Context Builder -> Repo Search -> LLM -> Patch Suggestion -> Tests
```

### Customer Support Bot

```text
User -> Intent -> KB Retrieval -> LLM -> Policy Check -> Reply / Human
```

### Enterprise AI Platform

```text
Apps -> AI Gateway -> Prompt Registry -> Model Router
                     |-> Retrieval
                     |-> Tools
                     |-> Evals
                     |-> Audit Logs
```

## Common discussion Questions

- Design a RAG-based support bot.
- How would you route between models?
- How do you evaluate answers?
- How do you control cost?

## Common Mistakes

- Drawing only LLM boxes.
- No data permissions.
- No evals.
- No fallback or rollback.

## Comparison Table (if applicable)

| Component | Purpose |
|---|---|
| AI Gateway | Auth, routing, quotas |
| Orchestrator | Prompt, retrieval, tools |
| Vector DB | Semantic retrieval |
| Evals | Quality checks |
| Observability | Debugging and SLOs |

## Key Takeaways

Great AI system design looks like strong backend design with extra attention to model quality, safety, and cost.

