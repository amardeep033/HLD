# AI Fundamentals

## Why this matters in discussions

AI platform discussions expect you to connect AI vocabulary with backend system design. You do not need to derive algorithms, but you should know what each layer means and where it appears in production.

## Definition

Artificial Intelligence is software that performs tasks that usually require human judgment. Machine Learning is a way to build AI by learning patterns from data. Deep Learning uses neural networks. Generative AI creates new text, code, images, or other outputs.

## Mental Model / Analogy

Think of AI as a decision service. Traditional code follows explicit rules; ML learns the rules from examples; GenAI predicts useful output from context.

## Architecture Diagram (ASCII)

```text
User Request
  |
Backend API
  |
AI Capability
  |-- Rules
  |-- ML model
  |-- LLM / GenAI
  |
Business Response
```

## How it works

AI systems consume inputs, transform them into features or tokens, run a model, and return predictions or generated content. Production systems add validation, retrieval, logging, safety checks, and human review where needed.

## Backend Engineer Perspective

Treat AI as an unreliable dependency with variable latency, cost, and quality. Design retries, timeouts, rate limits, caching, observability, and fallback paths.

## Real-world Example

A customer support bot classifies intent, retrieves policy documents, asks an LLM to draft an answer, checks for unsafe content, and logs the interaction for evaluation.

## Common discussion Questions

- What is the difference between AI, ML, Deep Learning, and GenAI?
- Where does an LLM fit in a backend architecture?
- Why are AI systems harder to test than deterministic services?

## Common Mistakes

- Treating AI responses as always correct.
- Ignoring cost and latency.
- Confusing model training with model inference.
- Skipping evaluation and monitoring.

## Comparison Table (if applicable)

| Term | Meaning | Backend analogy |
|---|---|---|
| AI | Broad intelligent behavior | Product capability |
| ML | Learns from data | Prediction service |
| Deep Learning | Neural network ML | Complex model backend |
| GenAI | Generates new content | Probabilistic content service |

## Key Takeaways

AI is a capability, ML is one implementation path, and LLMs are a major GenAI building block. Backend engineers should focus on reliability, integration, safety, and measurable quality.

