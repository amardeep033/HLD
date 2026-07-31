# MCP

## Why this matters in discussions

MCP is becoming a common way to connect AI applications to tools, data, and external systems through a standard protocol.

## Definition

Model Context Protocol is a standard interface for exposing tools, resources, and prompts to AI clients.

## Mental Model / Analogy

MCP is like USB-C for AI integrations: one client can connect to many tool providers through a common shape.

## Architecture Diagram (ASCII)

```text
AI Client
  |
MCP Protocol
  |
MCP Server
  |-- Tools
  |-- Resources
  |-- Prompts
  |
External Systems
```

## How it works

An MCP client discovers capabilities from an MCP server, then invokes tools or reads resources. The server owns integration logic, auth, and access boundaries.

## Backend Engineer Perspective

Think about tenancy, permissions, auditability, timeouts, schema design, and error handling. MCP does not remove the need for secure backend design.

## Real-world Example

A coding assistant connects to an MCP server that exposes repository search, issue lookup, and build status tools.

## Common discussion Questions

- What problem does MCP solve?
- How is MCP different from REST?
- How is MCP related to tool calling?
- Where should auth live?

## Common Mistakes

- Treating MCP as magic model memory.
- Exposing unsafe tools.
- Ignoring least privilege.
- Designing vague tool schemas.

## Comparison Table (if applicable)

| MCP | REST API |
|---|---|
| AI-oriented capability protocol | General web API style |
| Exposes tools/resources/prompts | Exposes endpoints |
| Designed for discovery | Usually manually documented |

## Key Takeaways

MCP standardizes how AI clients access external capabilities. The backend still owns security, correctness, and operations.

