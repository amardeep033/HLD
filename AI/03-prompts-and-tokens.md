# Prompts and Tokens

## Why this matters in discussions

Prompting is the API contract between your backend and the model. Tokens drive context size, latency, and cost.

## Definition

A prompt is the instruction and context sent to a model. Tokens are chunks of text the model reads and writes. A context window is the maximum number of tokens the model can handle in one request.

## Mental Model / Analogy

The prompt is a request payload. Tokens are payload size. The context window is the max request plus response size.

## Architecture Diagram (ASCII)

```text
System Prompt
  +
Developer Rules
  +
User Message
  +
Retrieved Context
  |
LLM Request
  |
Response Tokens
```

## How it works

Prompts usually include role, task, constraints, examples, and context. Temperature controls randomness. Lower temperature is useful for deterministic tasks; higher temperature helps brainstorming.

## Backend Engineer Perspective

Store prompts like versioned configuration. Add tests for prompt changes. Keep prompts concise, separate trusted and untrusted content, and measure token usage.

## Real-world Example

A backend sends a support prompt with: "Answer only from provided policy snippets. If missing, say you do not know." This reduces hallucinated policy answers.

## Common discussion Questions

- What is a system prompt?
- How does temperature affect output?
- What happens when context exceeds the window?
- How do you prevent prompt injection?

## Common Mistakes

- Stuffing too much context into every request.
- Mixing user text with trusted instructions.
- No prompt versioning.
- No regression tests for prompts.

## Comparison Table (if applicable)

| Concept | Purpose |
|---|---|
| System prompt | High-level behavior |
| User prompt | Current user request |
| Retrieved context | External facts |
| Temperature | Output randomness |

## Key Takeaways

Prompts are production inputs. Treat them as versioned, tested, observable configuration.

