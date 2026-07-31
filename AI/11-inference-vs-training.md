# Inference vs Training

## Why this matters in discussions

Many candidates mix up training, fine-tuning, and inference. Clear separation signals strong platform judgment.

## Definition

Training teaches a model from data. Inference uses a trained model to produce outputs for new inputs.

## Mental Model / Analogy

Training is building the binary. Inference is serving requests with that binary.

## Architecture Diagram (ASCII)

```text
Training:
Data -> Training Job -> Model Artifact

Inference:
Request -> Model Artifact -> Prediction
```

## How it works

Training is batch-oriented and compute-heavy. Inference is request-oriented and latency-sensitive. Fine-tuning is additional training on a specific dataset.

## Backend Engineer Perspective

Training pipelines need data quality, reproducibility, experiment tracking, and artifact management. Inference needs SLOs, scaling, monitoring, and rollback.

## Real-world Example

A fraud model is trained weekly on historical transactions. The inference service scores live transactions in milliseconds.

## Common discussion Questions

- What is the difference between training and inference?
- Why is inference usually latency-sensitive?
- What is model drift?
- How do you roll back a model?

## Common Mistakes

- Saying RAG trains the model.
- No model registry.
- No evaluation before deployment.
- Confusing batch jobs with online serving.

## Comparison Table (if applicable)

| Training | Inference |
|---|---|
| Learns from data | Uses learned model |
| Offline or batch | Online or batch |
| Expensive compute | Latency and cost sensitive |
| Produces artifact | Produces prediction |

## Key Takeaways

Training creates model artifacts; inference serves them. They have different platforms, metrics, and failure modes.

