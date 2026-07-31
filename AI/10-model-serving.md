# Model Serving

## Why this matters in discussions

AI backend roles often involve serving models reliably, whether through hosted APIs or self-hosted infrastructure.

## Definition

Model serving is running a trained model behind an API for inference.

## Mental Model / Analogy

It is like operating a high-latency, GPU-hungry microservice with special batching and memory constraints.

## Architecture Diagram (ASCII)

```text
Client
  |
API Gateway
  |
Inference Service
  |
Model Runtime
  |
GPU / CPU
```

## How it works

Requests are queued, batched, tokenized, run through the model, decoded, and returned. Serving stacks may use quantization, caching, streaming, and autoscaling.

## Backend Engineer Perspective

Track p50/p95/p99 latency, tokens per second, GPU utilization, queue time, error rate, and cost. Decide between hosted models and self-hosting based on control, cost, compliance, and scale.

## Real-world Example

A company uses hosted APIs for general chat and self-hosts a smaller model for high-volume classification.

## Common discussion Questions

- Hosted model or self-hosted model?
- What is quantization?
- How does batching improve throughput?
- What metrics matter for serving?

## Common Mistakes

- Ignoring cold starts.
- No capacity planning.
- No fallback model.
- Optimizing cost before measuring quality.

## Comparison Table (if applicable)

| Hosted Model | Self-hosted Model |
|---|---|
| Fast to integrate | More control |
| Provider manages infra | You manage GPUs |
| Usage-based pricing | Capacity-based pricing |
| Less ops burden | More compliance options |

## Key Takeaways

Model serving is production infrastructure. Balance quality, latency, cost, compliance, and operational complexity.

