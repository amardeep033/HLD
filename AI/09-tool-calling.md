# Tool Calling

## Why this matters in discussions

Tool calling is how LLM applications interact with real systems instead of only generating text.

## Definition

Tool calling lets a model request a structured function call. The application executes the tool and gives the result back to the model.

## Mental Model / Analogy

The model chooses an API call, but your backend is the only component that actually executes it.

## Architecture Diagram (ASCII)

```text
User
  |
Backend API
  |
LLM chooses tool
  |
Backend executes function
  |
Tool result
  |
LLM final answer
```

## How it works

The backend declares tool names, descriptions, and JSON schemas. The model emits arguments. The backend validates, authorizes, executes, and returns the result.

## Backend Engineer Perspective

Use strict schemas, validation, idempotency keys, timeouts, and audit logs. Never let the model bypass authorization.

## Real-world Example

A travel assistant calls `search_flights`, `hold_booking`, and `send_confirmation`, with human approval before purchase.

## Common discussion Questions

- What is function calling?
- Who executes the tool?
- How do you prevent unsafe tool use?
- How do tool calling and MCP differ?

## Common Mistakes

- Trusting model-generated arguments.
- No idempotency for write tools.
- Missing authorization checks.
- Too many overlapping tools.

## Comparison Table (if applicable)

| Tool Calling | MCP |
|---|---|
| Model requests a function call | Protocol for exposing tools/resources |
| Usually app-specific | Standardized integration layer |
| Backend executes call | MCP server hosts capability |

## Key Takeaways

Tool calling turns LLMs into workflow participants. The backend must validate, authorize, execute, and record every action.

