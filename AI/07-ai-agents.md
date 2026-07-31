# AI Agents

## Why this matters in discussions

Agents are popular, but many teams overuse them. discussions test whether you can separate useful automation from uncontrolled model loops.

## Definition

An AI agent uses a model to choose actions, call tools, observe results, and continue until a goal is met.

## Mental Model / Analogy

An agent is an LLM inside a control loop with tools and state.

## Architecture Diagram (ASCII)

```text
Goal
  |
Planner / LLM
  |
Action -> Tool -> Observation
  |                  |
  +------ Loop ------+
  |
Final Result
```

## How it works

The agent receives a goal, reasons about the next step, calls a tool, reads the result, and decides whether to continue. Some agents use memory, planning, or multiple specialized agents.

## Backend Engineer Perspective

Put hard boundaries around agent actions. Use approvals, budgets, timeouts, audit logs, idempotency, and tool-level permissions.

## Real-world Example

An incident assistant reads alerts, queries logs, summarizes likely causes, and creates a draft remediation plan for human approval.

## Common discussion Questions

- What is the difference between an LLM and an agent?
- When should you avoid agents?
- What is agentic AI?
- How do you make agents safe?

## Common Mistakes

- Letting agents run without budgets.
- Giving broad tool permissions.
- No audit trail.
- Using agents where a workflow would be simpler.

## Comparison Table (if applicable)

| AI Agent | Agentic AI |
|---|---|
| Concrete system with tools and loop | Broader design style for autonomous workflows |
| Can be simple or complex | Often includes planning and adaptation |
| Needs permissions | Needs governance |

## Key Takeaways

Agents are useful when the path is dynamic. Keep them bounded, observable, and permissioned.

