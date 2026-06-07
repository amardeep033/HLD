# Deployment Strategies — A Mental Model

You have a new version of your service. It works in staging. Now you need to get it to production without breaking anything for real users.

The question is not *whether* to deploy — it's **how**.

---

## 1. The Problem All Strategies Solve

Naive deployment: stop the old version, start the new one.

```
v1 running
  ↓  (kill)
[downtime gap]
  ↓  (start)
v2 running
```

Problems:
- Users see errors during the gap
- If v2 is broken, **all** users are affected immediately
- Rolling back means another round of downtime

Modern strategies answer: **"How do I ship v2 without all-or-nothing risk?"**

---

## 2. Rolling Deployment (Kubernetes Default)

Replace pods **one at a time**. New pods come up, old pods go down. At any moment, some pods run v1 and some run v2.

```
Start:   [v1] [v1] [v1] [v1]
Step 1:  [v1] [v1] [v1] [v2]
Step 2:  [v1] [v1] [v2] [v2]
Step 3:  [v1] [v2] [v2] [v2]
Done:    [v2] [v2] [v2] [v2]
```

Kubernetes handles this automatically. You control the pace:

```yaml
spec:
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1        # allow 1 extra pod during update
      maxUnavailable: 0  # never kill a pod before the new one is healthy
```

**Rollback:**

```bash
kubectl rollout undo deployment/myapp-deployment
```

Kubernetes keeps the previous ReplicaSet. Undo scales old pods back up, new pods down — no redeploy needed.

**Limitation:** Both versions serve live traffic simultaneously. If v2 has a bug in a shared DB path, some real requests will fail before you notice.

---

## 3. Blue/Green Deployment

Two identical environments run in parallel. **Blue** is live. **Green** has v2 deployed and tested. When you're confident, flip the load balancer — all traffic moves to Green instantly.

```
Before:
  Traffic ──► Load Balancer ──► Blue v1 (100%)   [live]
                                Green v2           [idle, pre-tested]

After flip:
  Traffic ──► Load Balancer ──► Blue v1            [idle, rollback target]
                                Green v2 (100%)    [live]
```

**Rollback:** flip the switch back. Instant. Blue v1 was never torn down.

**How it works in Kubernetes:** deploy v2 to a separate Deployment, run smoke tests against the green Service directly, then update the Service selector:

```yaml
# Switch traffic by patching the Service selector
kubectl patch service myapp -p '{"spec":{"selector":{"version":"green"}}}'
```

| Aspect | Detail |
|---|---|
| Downtime | Zero |
| Rollback | Instant — patch selector back to `blue` |
| Cost | 2× infrastructure while both environments are live |
| Risk | Very low — v2 is fully tested before any user sees it |

> Blue/Green is ideal for **critical services** where zero-tolerance downtime is required and infrastructure cost is secondary.

---

## 4. Canary Deployment

Release v2 to a **small percentage of real traffic** first — say 5%. Monitor error rates, latency, and business metrics. If healthy, gradually increase. If something looks wrong, drain to 0% immediately.

```
5% canary phase:
  95% ──► [v1 pods]  (stable)
   5% ──► [v2 pods]  (canary)

  [monitor 15 min — error rate, p99 latency, business KPIs]

  [healthy?  → 20% → 50% → 100%]
  [degraded? → drain canary to 0%, investigate]
```

**Simple approximation in Kubernetes:** run 19 pods of v1 and 1 pod of v2. Default round-robin load balancing gives ~5% canary traffic by replica weighting.

**Production approach:** use **Argo Rollouts** or a service mesh (Istio / Linkerd) for precise traffic splitting and automated analysis:

```yaml
# Argo Rollouts — canary with progressive traffic shift
apiVersion: argoproj.io/v1alpha1
kind: Rollout
spec:
  strategy:
    canary:
      steps:
        - setWeight: 5
        - pause: {duration: 10m}
        - setWeight: 20
        - pause: {duration: 10m}
        - setWeight: 50
        - pause: {duration: 10m}
```

| Aspect | Detail |
|---|---|
| Downtime | Zero |
| Rollback | Drain canary weight to 0% |
| Cost | Slightly more than baseline |
| Risk | Very low — real users test it, but only a fraction |

> Canary is the **gold standard** for high-traffic production services. You get real signal from real users before committing fully.

---

## 5. Comparison: When to Use Which

| Strategy | Risk | Rollback | Cost | Best For |
|---|---|---|---|---|
| Recreate | High (downtime) | Redeploy | Cheapest | Dev/test only |
| Rolling Update | Medium | `kubectl rollout undo` | Normal | Most production services |
| Blue/Green | Low | Instant selector flip | 2× infra | Critical services, zero downtime |
| Canary | Very low | Drain weight to 0% | Slightly more | High-traffic, uncertain changes |

The right strategy depends on your **risk tolerance**, **traffic volume**, and **infrastructure cost**.

---

## 6. Chaos Testing

Deployment strategies protect you during **planned** releases. Chaos testing asks a harder question: **"What happens when things fail unexpectedly?"**

The idea: deliberately inject failure into production (or a production-like environment) and observe whether your system self-heals.

```
Chaos experiments:

  kill a pod           →  does Kubernetes reschedule it?
  inject network delay →  do your timeouts fire correctly?
  take down a node     →  do other nodes absorb the load?
  kill the database    →  does circuit breaking activate?
```

### 6.1: Why You Need It

Staging never perfectly mirrors production. Tests tell you the system works under normal conditions. Chaos testing tells you it works under **failure**.

> A system that has never been tested under failure is a system that will fail unpredictably in production.

### 6.2: The Chaos Monkey Principle

Netflix invented **Chaos Monkey** — a tool that randomly terminates EC2 instances during business hours. The forcing function: if failure is frequent enough, engineers have no choice but to build systems that tolerate it.

Modern tools extend this beyond instance killing:

| Tool | What It Does |
|---|---|
| Chaos Monkey | Randomly terminates AWS EC2 instances |
| Gremlin | CPU/memory stress, network drops, latency injection |
| Chaos Mesh | Pod kill, network chaos, I/O faults — Kubernetes-native |
| Litmus | Open-source chaos experiments for Kubernetes workloads |
| Toxiproxy | Simulate latency, packet loss, and connection drops at the network layer |

### 6.3: How to Run a Chaos Experiment

Follow the scientific method:

1. **Define steady state** — what does healthy look like? (e.g., p99 latency < 200ms, error rate < 0.1%)
2. **Hypothesize** — "killing one pod should not change steady state because K8s will reschedule it"
3. **Inject failure** — kill the pod, drop the network, throttle CPU
4. **Observe** — did the system return to steady state? How long did it take?
5. **Fix the gaps** — if recovery failed, fix it. If recovery was slow, tune it.

```yaml
# Chaos Mesh — kill one pod matching a label selector
apiVersion: chaos-mesh.org/v1alpha1
kind: PodChaos
metadata:
  name: pod-kill-example
spec:
  action: pod-kill
  mode: one
  selector:
    namespaces: [production]
    labelSelectors:
      app: myapp
```

### 6.4: Blast Radius — Start Small

Start in staging. When running chaos in production, limit the **blast radius**:

- Target a single namespace or a single service, not the whole cluster
- Run during low-traffic windows, not peak hours
- Have a kill switch ready to stop the experiment immediately
- Only run chaos when your rollback and monitoring are solid

> Never run chaos experiments without solid observability in place. Chaos without monitoring is just an outage with extra steps.
