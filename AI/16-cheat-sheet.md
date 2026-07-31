# Cheat Sheet

## Why this matters in discussions

This is the 10-minute revision guide before an AI backend discussion.

## Definition

AI backend work is building reliable systems around probabilistic models.

## Mental Model / Analogy

LLM output is not a database read. It is generated text that must be grounded, checked, and monitored.

## Architecture Diagram (ASCII)

```text
User
  |
API / Auth / Rate Limit
  |
AI Orchestrator
  |-- Prompt
  |-- Retrieval
  |-- Tools
  |-- Guardrails
  |
Model
  |
Evals + Logs + Metrics
```

## How it works

Prompts instruct. Tokens limit. RAG grounds. Embeddings retrieve. Vector DBs search. Tools act. Guardrails constrain. Evals measure.

## Backend Engineer Perspective

Say these words in design discussions: latency, cost, ACL, evals, observability, fallback, model version, prompt version, audit log, human approval.

## Real-world Example

```text
Support Bot = Auth + KB Retrieval + LLM + Citation + Policy Check + Escalation
```

## Common discussion Questions

- RAG vs fine-tuning?
- How do you prevent prompt injection?
- How do you evaluate a chatbot?
- Hosted model or self-hosted?
- How do you secure tool calling?

## Common Mistakes

- No source citations.
- No access control in retrieval.
- No evals.
- No cost controls.
- Too much autonomy for agents.

## Comparison Table (if applicable)

| Pair | Difference |
|---|---|
| Training vs Inference | Learn model vs use model |
| RAG vs Fine-tuning | Runtime facts vs behavior change |
| LLM vs Agent | Generates text vs takes actions |
| MCP vs Tool Calling | Exposes capabilities vs invokes them |
| Embedding vs Vector DB | Representation vs storage/search |

## Key Takeaways

Keep answers practical. AI systems are backend systems with probabilistic dependencies, new security risks, and quality measurement challenges.

