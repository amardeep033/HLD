# LLMs

## Why this matters in discussions

LLMs are the core dependency in many AI backend systems. discussioners expect you to explain what they do, what they cannot guarantee, and how to wrap them safely.

## Definition

A Large Language Model predicts the next token based on input context. Foundation models are large general-purpose models that can be adapted to many tasks.

## Mental Model / Analogy

An LLM is like a highly capable autocomplete engine with broad world knowledge, but no built-in truth database or business authority.

## Architecture Diagram (ASCII)

```text
Prompt
  |
Tokenizer
  |
LLM
  |
Token Probabilities
  |
Generated Answer
```

## How it works

Text is split into tokens. The model reads those tokens and repeatedly predicts the next likely token. Parameters like temperature affect randomness. The output is generated until a stop condition or token limit.

## Backend Engineer Perspective

Wrap LLMs behind an application service. Track prompts, model versions, latency, token usage, errors, and quality metrics. Keep business rules outside the model when correctness matters.

## Real-world Example

An internal assistant receives a question, retrieves employee handbook chunks, asks the LLM to answer using only those chunks, and returns citations.

## Common discussion Questions

- What is an LLM?
- Why do LLMs hallucinate?
- What is a context window?
- How do you reduce latency and cost?

## Common Mistakes

- Assuming the model has current private data.
- Using long prompts without measuring cost.
- Forgetting model versioning.
- Treating fluent output as verified output.

## Comparison Table (if applicable)

| Item | What it is |
|---|---|
| ChatGPT | User-facing application |
| LLM | Underlying text model |
| AI Agent | LLM plus tools, memory, and control loop |

## Key Takeaways

LLMs are powerful reasoning and language interfaces, but they need backend controls, retrieval, monitoring, and guardrails to become reliable products.

