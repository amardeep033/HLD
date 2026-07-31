# Vector Databases

## Why this matters in discussions

Vector databases are common infrastructure in RAG and semantic search systems. discussioners care about indexing, filtering, latency, and consistency.

## Definition

A vector database stores embeddings and supports nearest-neighbor search over high-dimensional vectors.

## Mental Model / Analogy

It is a search engine for meaning instead of exact words.

## Architecture Diagram (ASCII)

```text
Documents -> Chunks -> Embeddings -> Vector DB
                                      |
User Query -> Query Embedding --------+
                                      |
                                  Top Matches
```

## How it works

Vector databases use approximate nearest-neighbor indexes for fast similarity search. They often support metadata filters, namespaces, hybrid search, and payload storage.

## Backend Engineer Perspective

Design for ingestion throughput, query latency, ACL filtering, index rebuilds, backups, and observability. Hybrid keyword plus vector search often performs better than vector-only search.

## Real-world Example

An enterprise assistant stores document chunks with team permissions and retrieves only chunks the user can access.

## Common discussion Questions

- What is approximate nearest-neighbor search?
- Why use metadata filters?
- What is hybrid search?
- How do you handle deletes and updates?

## Common Mistakes

- Skipping tenant isolation.
- No re-indexing plan.
- Ignoring data deletion requirements.
- Returning too many irrelevant chunks.

## Comparison Table (if applicable)

| Embeddings | Vector Database |
|---|---|
| Numeric representation | Storage and search system |
| Produced by model | Operated like infra |
| Captures meaning | Retrieves similar items |

## Key Takeaways

Vector DBs are infrastructure. Treat them with the same discipline as search, cache, and database systems.

