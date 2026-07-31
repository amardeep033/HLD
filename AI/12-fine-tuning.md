# Fine-tuning

## Why this matters in discussions

Fine-tuning is often suggested when RAG or prompting would be better. discussioners look for judgment.

## Definition

Fine-tuning updates a pre-trained model using task-specific examples so it behaves better for a target use case.

## Mental Model / Analogy

Fine-tuning teaches the model a style or pattern. RAG gives it reference material at runtime.

## Architecture Diagram (ASCII)

```text
Examples
  |
Prepare Dataset
  |
Fine-tune Job
  |
Custom Model
  |
Inference
```

## How it works

You collect examples, clean them, train or adapt the model, evaluate it, and deploy a new model version. The dataset matters more than the training buzzwords.

## Backend Engineer Perspective

Build dataset pipelines, validation, model registry, evals, rollout, rollback, and monitoring. Keep old model versions available.

## Real-world Example

A company fine-tunes a model to classify support tickets into internal categories using thousands of labeled examples.

## Common discussion Questions

- When should you fine-tune?
- When is RAG better?
- What data do you need?
- How do you evaluate a fine-tuned model?

## Common Mistakes

- Fine-tuning for private facts that change often.
- Poor dataset quality.
- No baseline comparison.
- No rollback plan.

## Comparison Table (if applicable)

| Use Case | Better Fit |
|---|---|
| Current documents | RAG |
| Output style | Fine-tuning |
| Domain taxonomy | Fine-tuning |
| Frequently changing facts | RAG |

## Key Takeaways

Fine-tuning changes behavior. Use it for repeatable patterns, not as a replacement for retrieval.

