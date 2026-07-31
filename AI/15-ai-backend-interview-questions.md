# AI Backend discussion Questions

## Why this matters in discussions

discussion practice helps convert vague AI knowledge into crisp backend answers.

## Definition

This file is a compact question bank for AI platform/backend discussions.

## Mental Model / Analogy

Answer like a backend engineer: define the concept, place it in architecture, name tradeoffs, and mention failure modes.

## Architecture Diagram (ASCII)

```text
Question -> Definition -> Architecture Impact -> Tradeoff -> Risk
```

## How it works

Use these as flashcards. Keep answers concise and practical.

## Backend Engineer Perspective

Favor answers that mention APIs, data flow, latency, cost, security, reliability, evals, and observability.

## Real-world Example

For "RAG vs fine-tuning", answer: RAG supplies facts at runtime; fine-tuning changes behavior. Use RAG for changing documents and fine-tuning for repeated task patterns.

## Common discussion Questions

1. What is AI? Software that performs tasks requiring human-like judgment.
2. What is ML? AI learned from data instead of handwritten rules.
3. What is deep learning? ML using neural networks with many layers.
4. What is GenAI? AI that creates new content.
5. What is an LLM? A model that predicts and generates text tokens.
6. What is a foundation model? A large general model adaptable to many tasks.
7. What is a token? A text chunk processed by a model.
8. What is context window? The token limit for input plus output.
9. What is a system prompt? High-priority instruction for model behavior.
10. What is temperature? A randomness control for generation.
11. Why do LLMs hallucinate? They predict plausible text, not guaranteed truth.
12. How reduce hallucination? RAG, citations, constraints, evals, and fallback.
13. What is RAG? Retrieval plus generation using external context.
14. Why use RAG? To answer from current or private data.
15. What is chunking? Splitting documents into retrievable pieces.
16. What is an embedding? A vector representation of meaning.
17. What is semantic search? Search by meaning instead of exact terms.
18. What is a vector DB? A database optimized for vector similarity search.
19. What is hybrid search? Keyword plus vector search.
20. What is reranking? Reordering retrieved results with a stronger scorer.
21. What is an AI agent? A model-driven loop that can use tools.
22. What is agentic AI? Systems designed for autonomous multi-step behavior.
23. What is tool calling? Model requests structured function execution.
24. Who executes tools? The backend, after validation and authorization.
25. What is MCP? A standard protocol for AI clients to access tools and resources.
26. MCP vs REST? MCP is AI capability discovery; REST is general endpoint style.
27. MCP vs tool calling? MCP exposes tools; tool calling invokes them.
28. What is inference? Running a trained model to produce output.
29. What is training? Learning model parameters from data.
30. What is fine-tuning? Additional training for a specific task or style.
31. RAG vs fine-tuning? RAG adds facts; fine-tuning changes behavior.
32. What is quantization? Smaller numeric precision to reduce memory and cost.
33. Hosted vs self-hosted? Hosted is faster; self-hosted gives control.
34. What is model routing? Choosing a model per request based on need.
35. What is prompt injection? Untrusted text attempts to override instructions.
36. How prevent prompt injection? Isolate instructions, validate, restrict tools.
37. What is AI observability? Tracking prompts, outputs, latency, cost, quality.
38. What are evals? Tests that measure model quality and safety.
39. What is offline eval? Evaluation on a fixed dataset before release.
40. What is online eval? Monitoring live behavior and feedback.
41. What is model drift? Performance changes as real-world data changes.
42. What is grounding? Tying answers to provided evidence.
43. Why citations? They improve trust and debuggability.
44. What is top-k retrieval? Return k most similar chunks.
45. What is metadata filtering? Restrict retrieval using fields like tenant or ACL.
46. Why ACLs in RAG? Prevent leaking documents between users.
47. How update RAG data? Re-ingest, embed, index, and version documents.
48. What is prompt versioning? Tracking prompt changes like code/config.
49. What is model versioning? Tracking model deployments and behavior.
50. What is fallback? Safer response path when model or retrieval fails.
51. How control cost? Token budgets, caching, routing, quotas, smaller models.
52. How reduce latency? Streaming, batching, caching, smaller context.
53. What is streaming? Sending tokens as generated.
54. What is batching? Grouping inference requests for throughput.
55. What is rate limiting? Protecting service and cost budgets.
56. What is idempotency? Safe retries for write actions.
57. Why human approval? For high-impact or irreversible actions.
58. What is a guardrail? A control around model input/output/action.
59. What is output validation? Checking model response format or policy.
60. What is structured output? Model response constrained to schema.
61. Why schemas for tools? Safer validation and execution.
62. What is a prompt store? Versioned storage for prompt templates.
63. What is an AI gateway? Shared layer for auth, routing, quota, logging.
64. What is conversation memory? Stored prior interactions or summaries.
65. What is long-term memory? Persisted user or task facts.
66. Risk of memory? Privacy, staleness, and incorrect personalization.
67. What is multi-agent system? Multiple agents with specialized roles.
68. Risk of multi-agent? Complexity, cost, loops, harder debugging.
69. How test agents? Tool mocks, budgets, traces, scenario evals.
70. What is retrieval precision? Retrieved chunks are relevant.
71. What is retrieval recall? Relevant chunks are retrieved.
72. What is answer faithfulness? Answer matches provided evidence.
73. What is toxicity filtering? Blocking harmful outputs.
74. What is PII redaction? Removing personal data from prompts/logs.
75. What is tenant isolation? Keeping customer data separated.
76. What is least privilege? Tools only get needed access.
77. What is audit logging? Recording actions for review.
78. How handle secrets? Never put them in prompts or logs.
79. What is data residency? Data stored/processed in required region.
80. What is model registry? System for model artifacts and metadata.
81. What is rollback? Reverting to a known good model or prompt.
82. What is canary release? Limited rollout before full deployment.
83. What is A/B testing? Comparing model or prompt variants live.
84. What is LLM cache? Reuse responses or intermediate results.
85. Cache risk? Stale or permission-inappropriate answers.
86. What is context compression? Summarizing or trimming context.
87. What is few-shot prompting? Including examples in prompt.
88. What is zero-shot prompting? Asking without examples.
89. What is chain-of-thought risk? Exposing hidden reasoning or sensitive data.
90. What is classification with LLMs? Mapping input to labels.
91. When use smaller models? High-volume simple tasks.
92. When use larger models? Complex reasoning or generation.
93. What is OCR in AI systems? Extracting text from documents/images.
94. What is multimodal AI? Models using text, images, audio, or video.
95. What is moderation? Checking content against safety policy.
96. How design support bot? RAG, policy guardrails, escalation, logging.
97. How design coding assistant? Context builder, repo search, patch generation, tests.
98. How design enterprise platform? Gateway, model router, tools, RAG, evals, audit.
99. Biggest AI backend risk? Trusting unverified model output.
100. Best discussion habit? State tradeoffs and failure modes clearly.

## Common Mistakes

- Giving academic definitions only.
- Ignoring production constraints.
- Forgetting permissions and safety.
- Saying "just use an LLM" without architecture.

## Comparison Table (if applicable)

| discussion Area | What to emphasize |
|---|---|
| Concepts | Simple definitions |
| Architecture | Data flow and components |
| Reliability | SLOs, fallback, observability |
| Security | ACLs, injection, audit |
| Quality | Evals and feedback |

## Key Takeaways

Practice short answers. The best responses connect AI concepts to production backend tradeoffs.

