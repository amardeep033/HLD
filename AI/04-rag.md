# RAG

## Why this matters in discussions

RAG is one of the most common production patterns for grounding LLMs in company-specific data without retraining the model.

## Definition

Retrieval-Augmented Generation retrieves relevant documents at request time and gives them to an LLM as context.

## Mental Model / Analogy

RAG is open-book answering. The LLM writes the answer, but the backend chooses the book pages.

## Architecture Diagram (ASCII)

```text
User Question
  |
Embed Query
  |
Vector Search
  |
Top Chunks
  |
Prompt + Context
  |
LLM Answer
```

## How it works

Documents are chunked, embedded, and indexed. At query time, the question is embedded, similar chunks are retrieved, optionally reranked, then passed to the model.

## Backend Engineer Perspective

Most RAG quality comes from ingestion, chunking, metadata, ranking, and evaluation. Build freshness, ACL filtering, observability, and fallback behavior.

## Real-world Example

A legal policy assistant indexes policy PDFs, retrieves the top passages, and answers with citations.

## Common discussion Questions

- When would you use RAG instead of fine-tuning?
- How do you handle stale documents?
- How do you evaluate RAG quality?
- What is chunking?

## Common Mistakes

- Assuming vector similarity alone is enough.
- Not filtering by permissions.
- Poor chunk sizes.
- No citations or source traceability.

## Comparison Table (if applicable)

| RAG | Fine-tuning |
|---|---|
| Adds external knowledge at runtime | Changes model behavior |
| Easier to update | Requires training pipeline |
| Good for factual corpora | Good for style or task pattern |
| Needs retrieval quality | Needs dataset quality |

## Key Takeaways

RAG is a backend architecture pattern: ingestion, indexing, retrieval, generation, evaluation, and access control.

