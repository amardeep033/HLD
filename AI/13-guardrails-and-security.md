# Guardrails and Security

## Why this matters in discussions

AI systems introduce new security and safety risks on top of normal backend risks.

## Definition

Guardrails are controls that reduce unsafe, incorrect, or unauthorized AI behavior. AI security includes prompt injection, data leakage, unsafe tool use, and model abuse.

## Mental Model / Analogy

Guardrails are validation, authorization, rate limiting, and policy checks for probabilistic systems.

## Architecture Diagram (ASCII)

```text
User Input
  |
Input Validation
  |
Retrieval / Tools
  |
LLM
  |
Output Policy Check
  |
Response / Escalation
```

## How it works

Controls can run before retrieval, before tool execution, after model output, and during logging. Sensitive systems add human approval and audit trails.

## Backend Engineer Perspective

Separate trusted instructions from untrusted user and document text. Enforce authorization in code, not prompts. Redact secrets and monitor abuse patterns.

## Real-world Example

A support bot refuses to reveal internal notes, filters retrieved content by user permissions, and escalates refund requests above a threshold.

## Common discussion Questions

- What is prompt injection?
- How do you secure tool calling?
- How do you prevent data leakage?
- What should be logged?

## Common Mistakes

- Relying only on prompts for security.
- Logging secrets.
- Tool access without authorization.
- No human review for high-impact actions.

## Comparison Table (if applicable)

| Risk | Control |
|---|---|
| Prompt injection | Instruction isolation and filters |
| Data leakage | ACLs, redaction, least privilege |
| Unsafe tool call | Validation and approval |
| Hallucination | RAG, citations, evals |

## Key Takeaways

AI security is backend security plus model-specific controls. Prompts help, but code must enforce policy.

