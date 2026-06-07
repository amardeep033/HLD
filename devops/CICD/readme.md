# CI/CD Pipelines

## 1. The Problem CI/CD Solves — Why It Exists

Before CI/CD, teams worked on features in isolation for days or weeks. Then came **"integration day"** — the dreaded moment everyone merged their branches. Conflicts, broken builds, untested code shipping to production. Deployments were manual, error-prone, and terrifying.

CI/CD kills this problem. The core insight: **the longer you delay integration and feedback, the more expensive fixes become.** CI/CD forces you to integrate on every commit, test automatically, and deploy small incremental changes instead of giant risky releases.

| | No CI/CD | CI Only | CI + CD |
|---|---|---|---|
| Release cadence | Monthly / quarterly | Weekly | Daily / every merge |
| Risk per deploy | Very high (big batches) | Medium | Low (small changes) |
| Feedback speed | Days to weeks | Hours | Minutes |

> CI/CD is not a tool — it's a practice. Tools (GitHub Actions, Jenkins, GitLab CI) are just how you implement it.

---

## 2. The Pipeline Lifecycle: Build → Test → Package → Deploy

Think of the pipeline as an **assembly line**. Every commit enters at one end, and if it survives every station without failing, it exits as a deployed running service. No human intervention, no guesswork.

```
git push origin main
       │
       ▼  Trigger
 ┌─────────────────────────────────────────────────┐
 │  1. BUILD     — compile, resolve deps           │
 │  2. TEST      — unit, integration, lint, SAST   │
 │  3. PACKAGE   — docker build → push image       │
 │  4. DEPLOY    — update Deployment in Kubernetes │
 └─────────────────────────────────────────────────┘
       │
       ▼  Live in production (if all green)
```

### 2.1: Build

Compile the code, resolve dependencies. For .NET: `dotnet build`. For Node: `npm ci`.

**Use `npm ci` not `npm install` in pipelines.** `ci` installs exactly what's in the lockfile — deterministic and reproducible. `install` can silently update transitive deps and introduce flaky builds.

Fail fast here. There's no point running tests against code that doesn't compile.

### 2.2: Test

Your quality gate — the most valuable part of CI. Three layers:

- **Unit tests** — isolated, no I/O, milliseconds each. Run thousands in parallel. Test a single function or class.
- **Integration tests** — test your service against a real database or queue (spin up via Docker during the pipeline). Slower, but catch real wiring bugs.
- **E2E / smoke tests** — simulate a real user flow. Run sparingly; expensive and slow. Best run post-deploy against staging.
- **SAST (Static Application Security Testing)** — tools like `Semgrep`, `dotnet-sonarscanner` scan code for vulnerability patterns without running it.

> A test suite with no integration tests gives false confidence. A suite with only E2E tests is slow and unreliable. Balance all three.

### 2.3: Package

If tests pass, build the Docker image (`docker build`) and push it to a container registry (Docker Hub, AWS ECR, GCR). This image is the deployable artifact — **the same image that passed tests is what gets deployed.** No rebuilding in prod. This is what makes deployments reproducible.

Tag the image with the **git commit SHA** (e.g. `myapp:sha-a3f9c12`). Never use `:latest` — it's mutable, making rollbacks ambiguous. A SHA tag is immutable and permanently traceable.

### 2.4: Deploy

The pipeline updates the Kubernetes Deployment to use the new image tag via `kubectl set image` (or a GitOps tool like ArgoCD watches the repo and does it automatically). Kubernetes handles the rollout — gradually replacing old pods with new ones.

---

## 3. Branching & Release Strategies

Branching strategy is **not just a Git convention** — it determines how often you can ship, how risky your deployments are, and how your pipeline is structured.

### 3.1: Trunk-Based Development (TBD)

Everyone commits to `main` frequently — multiple times a day. No long-lived feature branches. Used by elite teams (Google, Facebook).

Incomplete features are hidden behind **feature flags** — the code ships but the feature is off until you flip a flag in your config. This decouples deployment from release.

- CI runs on every commit to main — instant feedback
- Small changes mean a broken build is easy to trace to its commit
- Feature flags let you test unreleased features with internal users before full rollout

### 3.2: Gitflow

Uses multiple long-lived branches: `main`, `develop`, `feature/*`, `release/*`, `hotfix/*`.

- Good for products with explicit versioned releases (libraries, mobile apps, firmware)
- Bad for web services that can ship continuously — the ceremony slows you down

### 3.3: Environment Promotion Model

Regardless of branching strategy, code flows through environments:

```
feature branch → dev env (auto deploy on PR open)
       │
       ▼
 merge to main → staging env (auto deploy)
       │
       ▼
 manual approval → production (deploy with strategy)
```

Each environment is a **separate Kubernetes namespace** (or cluster), using the same Helm chart / manifests but different config values — secrets, connection strings, replica counts.

---

## 4. Automated Testing in Pipelines

| Test Type | Scope | Speed | Where in Pipeline | Example |
|---|---|---|---|---|
| Unit | Single function/class | Milliseconds | Always — every commit | Test `OrderCalculator.Apply()` |
| Integration | Service + DB/queue | Seconds | After build | Verify repo correctly writes to Postgres |
| Contract | Service API contracts | Seconds | After build | Pact tests between services |
| E2E / Smoke | Full user journey | Minutes | Post-deploy to staging | Login → add item → checkout |
| Performance | Load / latency | Minutes | Nightly / on demand | k6 / Locust load test |

### 4.1: Test Parallelism

Run test stages in parallel where possible. In GitHub Actions, use the `matrix` strategy to fan out tests across multiple runners. A suite that takes 20 minutes serially can finish in 4 with 5-way parallelism.

### 4.2: Flaky Tests — The Silent Killer

A **flaky test** passes sometimes and fails sometimes with no code change. Teams start ignoring red pipelines because "it's probably just flaky". This destroys CI/CD value entirely. Treat flaky tests as bugs:

- Quarantine them (tag as flaky, skip in main pipeline, fix separately)
- Common causes: non-deterministic ordering, time-dependent assertions, shared mutable state between tests

---

## 5. Rollbacks — When Things Go Wrong

Every deployment strategy must answer upfront: **"How do I undo this in the next 5 minutes?"** If you don't have a clear answer, the deployment is not production-ready.

### 5.1: Kubernetes Native Rollback

Kubernetes Deployments keep a revision history. Rollback is a single command:

```bash
# Roll back to the previous revision
kubectl rollout undo deployment/myapp-deployment

# Roll back to a specific revision
kubectl rollout undo deployment/myapp-deployment --to-revision=3

# View revision history
kubectl rollout history deployment/myapp-deployment
```

This works because Kubernetes stores the previous ReplicaSet. Undo scales the old RS back up and scales the current one down — no re-pulling or re-building.

### 5.2: Why Rollbacks Fail in Practice: Database Migrations

Rollbacks fail when code changes are paired with **backward-incompatible schema migrations**:

```
v1 code  →  reads column "user_name"
Migration  →  renames "user_name" to "full_name"
v2 code  →  reads column "full_name"

Rollback to v1? Column is now "full_name". v1 crashes.
```

The fix is the **expand-contract pattern**:

1. **Expand** — add `full_name` column alongside `user_name` (v1 still works, both columns exist)
2. **Migrate** — backfill data, update app to use `full_name` (v2 deployed)
3. **Contract** — drop `user_name` only after v2 is fully stable (v3)

> Rule: never deploy a breaking schema change and a code change in the same release.

---

## 6. Deployment Strategies — Blue/Green & Canary

The naive way to deploy: stop old version, start new one. This causes **downtime** and an all-or-nothing risk. Modern strategies separate *when the code is running* from *when traffic reaches it*.

### 6.1: Rolling Update (Kubernetes Default)

Replace old pods one at a time. Always some old and some new pods running simultaneously. Zero downtime, but both versions serve traffic during rollout.

```yaml
spec:
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1        # allow 1 extra pod during update
      maxUnavailable: 0  # never kill a pod before the new one is healthy
```

Problem: if v2 has a bug, some real users hit it before you notice.

### 6.2: Blue/Green Deployment

Run two identical environments: **Blue** (current live) and **Green** (new version). Route all traffic to Blue. Deploy v2 to Green. Test Green. Then flip the load balancer — all traffic moves to Green instantly. Blue stays up for instant rollback.

```
Before:
Traffic  ──►  Load Balancer  ──►  Blue v1 (100%)   [live]
                                  Green v2           [idle, testing]

After flip:
Traffic  ──►  Load Balancer  ──►  Blue v1            [idle, rollback target]
                                  Green v2 (100%)    [live]
```

Rollback = flip the switch back. Cost: 2x infrastructure capacity during transition.

### 6.3: Canary Deployment

Release the new version to a **small percentage of real traffic** first — say 5%. Monitor error rates, latency, business metrics. If healthy, gradually increase: 5% → 20% → 50% → 100%. If something looks wrong, drain to 0% immediately.

```
During canary:
  95% ──►  v1 pods (stable)
   5% ──►  v2 pods (canary)

  [monitor for 15 minutes — error rate, p99 latency, business KPIs]
  [if stable → promote to 100%]
  [if degraded → drain canary to 0%]
```

In Kubernetes, implement with **Argo Rollouts** or a service mesh (Istio/Linkerd). A simpler approximation: run 19 pods of v1 and 1 pod of v2 — roughly 5% canary by replica weighting.

| Strategy | Risk | Rollback Speed | Cost | Best For |
|---|---|---|---|---|
| Recreate | High (downtime) | Slow (redeploy) | Cheap | Dev environments only |
| Rolling Update | Medium | `kubectl rollout undo` | Normal | Most production services |
| Blue/Green | Low | Instant (switch flip) | 2x infra | Critical services, zero-tolerance downtime |
| Canary | Very low | Drain to 0% instantly | Slightly more | High-traffic, uncertain changes |

---

## 7. GitHub Actions — How a Pipeline is Actually Written

GitHub Actions is the most common CI/CD tool for teams on GitHub. The model: **workflows** (YAML files in `.github/workflows/`) are triggered by events (push, PR, schedule) and run one or more **jobs**, each on a fresh runner VM, composed of **steps**.

```yaml
# .github/workflows/ci-cd.yml

name: CI/CD Pipeline

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  REGISTRY: ghcr.io
  IMAGE_NAME: ${{ github.repository }}

jobs:

  # ── Stage 1: Build & Test ──────────────────────────────────────────────────
  build-and-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup .NET
        uses: actions/setup-dotnet@v4
        with:
          dotnet-version: 8.0.x

      - name: Restore
        run: dotnet restore

      - name: Build
        run: dotnet build --no-restore --configuration Release

      - name: Test
        run: dotnet test --no-build --configuration Release

  # ── Stage 2: Package (only on main, not on PRs) ───────────────────────────
  package:
    needs: build-and-test          # only runs if tests passed
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    outputs:
      image-tag: ${{ steps.meta.outputs.tags }}
    steps:
      - uses: actions/checkout@v4

      - name: Login to registry
        uses: docker/login-action@v3
        with:
          registry: ${{ env.REGISTRY }}
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}

      - name: Tag image with commit SHA
        id: meta
        uses: docker/metadata-action@v5
        with:
          images: ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}
          tags: type=sha   # → myapp:sha-a3f9c12

      - name: Build and push
        uses: docker/build-push-action@v5
        with:
          push: true
          tags: ${{ steps.meta.outputs.tags }}

  # ── Stage 3: Deploy to staging ────────────────────────────────────────────
  deploy-staging:
    needs: package
    runs-on: ubuntu-latest
    environment: staging
    steps:
      - name: Deploy
        run: |
          kubectl set image deployment/myapp myapp=${{ needs.package.outputs.image-tag }}
          kubectl rollout status deployment/myapp

  # ── Stage 4: Deploy to production (manual approval gate) ──────────────────
  deploy-production:
    needs: deploy-staging
    runs-on: ubuntu-latest
    environment: production        # requires manual approval in GitHub UI
    steps:
      - name: Deploy
        run: |
          kubectl set image deployment/myapp myapp=${{ needs.package.outputs.image-tag }}
          kubectl rollout status deployment/myapp --timeout=5m
```

Key design decisions in the workflow above:

- `needs:` — creates a dependency graph. `package` only runs if `build-and-test` succeeded.
- `if: github.ref == ...` — prevents image builds on PRs. Tests run on every PR; packaging happens only on merge.
- `environment: production` — GitHub requires a human to approve before this job runs.
- **SHA tags** — `sha-a3f9c12` is permanently traceable. You always know exactly what code is running.

---

## 8. GitLab CI / Jenkins — The Other Tools

### 8.1: GitLab CI

Uses a single `.gitlab-ci.yml` file. Same concepts (stages, jobs, triggers) but with tighter vertical integration — built-in container registry, environments, and security scanning.

```yaml
# .gitlab-ci.yml

stages: [build, test, package, deploy]

variables:
  IMAGE_TAG: $CI_REGISTRY_IMAGE:$CI_COMMIT_SHORT_SHA

build:
  stage: build
  script: dotnet build --configuration Release

test:
  stage: test
  script: dotnet test

package:
  stage: package
  only: [main]
  script:
    - docker build -t $IMAGE_TAG .
    - docker push $IMAGE_TAG

deploy-production:
  stage: deploy
  only: [main]
  when: manual         # human must click "Deploy" in GitLab UI
  script:
    - kubectl set image deployment/myapp myapp=$IMAGE_TAG
```

### 8.2: Jenkins

Older, self-hosted, configured via a `Jenkinsfile`. More flexible but requires ops overhead to maintain the Jenkins server itself.

```groovy
// Jenkinsfile
pipeline {
  agent any
  stages {
    stage('Build') { steps { sh 'dotnet build' } }
    stage('Test')  { steps { sh 'dotnet test'  } }
    stage('Deploy') {
      when { branch 'main' }
      steps {
        input "Deploy to production?"    // manual approval
        sh 'kubectl apply -f k8s/'
      }
    }
  }
}
```

| Tool | Hosting | Config file | Best For | Weakness |
|---|---|---|---|---|
| GitHub Actions | Cloud (GitHub) | `.github/workflows/*.yml` | Teams on GitHub, fast setup | Vendor lock-in |
| GitLab CI | Cloud or self-hosted | `.gitlab-ci.yml` | Integrated DevSecOps platform | Heavy if only using CI |
| Jenkins | Self-hosted | `Jenkinsfile` | Legacy / enterprise, full control | High ops overhead |
| CircleCI | Cloud | `.circleci/config.yml` | Performance-focused teams | Cost at scale |

---

## 9. Secrets Management in Pipelines

**Never hardcode secrets in your pipeline YAML or Dockerfile.** They end up in git history and build logs. This is one of the most common security mistakes at SDE-2 level.

### 9.1: The Right Model

Secrets live in a secrets store. The pipeline fetches them at runtime, injects into the container as environment variables or mounted files. The image itself contains zero secrets.

```
Pipeline YAML  →  references secret by name (not value)
                        │
                        ▼
            Secrets Store (Vault / AWS Secrets Manager)
            fetches value at runtime
                        │
                        ▼
            Injected as env var into running container
```

### 9.2: GitHub Actions Secrets

Store secrets in GitHub repository or organisation settings. Reference them in workflows:

```yaml
- name: Deploy
  env:
    DB_PASSWORD: ${{ secrets.DB_PASSWORD }}     # injected at runtime, masked in logs
    API_KEY: ${{ secrets.EXTERNAL_API_KEY }}
  run: ./deploy.sh
```

Secrets are **masked in logs** — if they accidentally appear in output, GitHub replaces them with `***`.

### 9.3: Kubernetes Secrets

In production, secrets are Kubernetes Secrets (base64-encoded, not encrypted by default — use **Sealed Secrets** or **External Secrets Operator** to pull from AWS Secrets Manager / HashiCorp Vault):

```yaml
# In Deployment manifest
env:
  - name: DB_PASSWORD
    valueFrom:
      secretKeyRef:
        name: db-credentials
        key: password
```

> Rule: treat secrets like private keys. Rotate them regularly. Audit access. Never let them touch a build artifact.

---

## 10. GitOps — The Modern Deployment Model

**GitOps** is the pattern where your Git repository is the single source of truth for what should be running in production. Instead of your CI pipeline directly calling `kubectl apply`, it **commits the new image tag to a config repo**. A GitOps operator (ArgoCD or Flux) watches that repo and reconciles the cluster to match.

```
CI Pipeline                      Config Repo (Git)              Kubernetes
──────────                       ─────────────────              ──────────
docker build → push image   →    commit: image: myapp:sha-x   ←  ArgoCD watches
                                                               →  detects drift
                                                               →  applies manifest
                                                               →  cluster = desired state
```

### 10.1: Why GitOps over direct kubectl in CI

- **Auditable** — every production change is a git commit with an author, timestamp, and PR review. `git log` tells you exactly who deployed what and when.
- **Declarative** — the cluster always matches the repo. If someone manually changes something in the cluster (`kubectl edit`), ArgoCD detects "drift" and corrects it.
- **Rollback = git revert** — revert the commit that changed the image tag, ArgoCD deploys the previous image automatically.
- **No cluster credentials in CI** — the pipeline only writes to Git. Only the GitOps operator (inside the cluster) has `kubectl` access, reducing the attack surface.

| | Push-based (direct kubectl in CI) | Pull-based (GitOps) |
|---|---|---|
| Credentials | CI runner has cluster access | Only ArgoCD inside cluster has access |
| Rollback | `kubectl rollout undo` | `git revert` |
| Audit trail | CI logs | Git history |
| Drift detection | No | Yes (ArgoCD alerts on manual changes) |
| Complexity | Simple to start | More setup, better at scale |

---

## 11. Why Deployments Fail — Root Causes

Almost every production incident from a deployment traces back to one of these:

- **Breaking database migration** — column renamed/dropped, old code still reading it. Fix: expand-contract pattern (see Section 5).
- **Missing or wrong environment config** — connection string pointing to dev DB in prod, or a required secret not set. Fix: validate config at startup; fail loud before taking traffic.
- **Insufficient resource limits** — new version uses more memory than the old one, pod gets `OOMKilled` immediately. Fix: set K8s resource requests and limits, load test before deploy.
- **Dependency not ready** — service starts before the DB migration has run, or before a dependent service is healthy. Fix: init containers, readiness probes, proper `depends_on` in orchestration.
- **Rolled back code, not rolled back config** — code is v1 but environment variables are configured for v2. Fix: version config alongside code in the same PR.
- **Testing gaps** — bug only appears at production data volume or at production scale. Fix: contract tests between services, production-like data in staging.

### 11.1: Making Deployments Reproducible

Three requirements for a deployment that always produces the same result:

- **Immutable image tags** — tag by commit SHA, never use `:latest`
- **Infrastructure as code** — K8s manifests, Helm charts, Terraform in version control. "What's running in prod" should be answerable by reading a file.
- **Config separate from code** — environment-specific values in Secrets/ConfigMaps, not baked into the image. Same image runs in dev, staging, and prod with different config.

---

## 12. Pipeline Observability — Measuring Your Pipeline

Most teams track application uptime but ignore pipeline health. At SDE-2 level you should know what a healthy pipeline looks like and how to measure it. The four DORA metrics are the standard:

| Metric | Definition | Elite Target | What It Tells You |
|---|---|---|---|
| **Deployment Frequency** | How often you deploy to prod | Multiple times/day | How often you're shipping value |
| **Lead Time for Changes** | Commit → running in production | < 1 hour | How fast your pipeline is |
| **Change Failure Rate** | % of deployments causing a prod incident | < 5% | How reliable your releases are |
| **Mean Time to Restore (MTTR)** | How long to recover from a failure | < 1 hour | How good your rollback / alerting is |

> These four metrics are directly correlated with business outcomes. Teams with elite DORA metrics ship faster and have fewer outages — not a trade-off.

### 12.1: What to alert on in the pipeline itself

- Pipeline duration regression — if your build time goes from 8 min to 25 min, catch it before it becomes a productivity tax
- Flaky test rate — track the `%` of runs that had a test failure unrelated to code change
- Deployment success rate — `% of deploys that reached healthy` per service

---

## 13. discussion: Design a CI/CD Pipeline for Microservices

The most common SDE-2 discussion question on this topic. The answer must cover: per-service pipelines, shared tooling, inter-service testing, and safe deployment.

### 13.1: Architecture

```
services/
├── order-service/     ← own Dockerfile, own pipeline, own image tag
├── payment-service/   ← own Dockerfile, own pipeline, own image tag
└── user-service/      ← own Dockerfile, own pipeline, own image tag

k8s/                   ← shared Helm charts / manifests
.github/workflows/     ← per-service workflow YAMLs
```

### 13.2: Per-Service Pipeline

- Triggered **only when that service's directory changes** — no rebuilding `user-service` when `order-service` changes
- Runs unit tests + integration tests for that service
- Builds and pushes image tagged with commit SHA
- Auto-deploys to dev → staging
- Canary + manual approval gate for production

### 13.3: Contract Testing Between Services

Microservices talk over HTTP/gRPC. If `order-service` changes its API response shape, `payment-service` breaks. Catch this with **contract tests** using Pact:

- Each consumer defines what it expects from a provider (the "contract")
- The provider's CI pipeline verifies it satisfies all consumers' contracts
- If the contract breaks, the pipeline fails — before anything deploys

> Contract tests are what makes independent deployability of microservices safe. Without them, you're hoping services stay compatible.

### 13.4: Full Flow Diagram

```
Developer pushes code to order-service/
       │
       ▼  GitHub Actions detects change in order-service/
  Build & Test
  ├── dotnet build
  ├── unit tests
  ├── integration tests (Docker Compose spins up Postgres)
  └── Pact contract verification (against payment-service contract)
       │ (all green)
       ▼
  Package
  ├── docker build (multi-stage, small image)
  ├── tag: order-service:sha-a3f9c12
  └── push → Container Registry
       │
       ▼  auto-deploy
  Staging (K8s namespace: staging)
  ├── kubectl set image deployment/order-service :sha-a3f9c12
  ├── rollout status — wait for all pods healthy
  └── smoke tests
       │
       ▼  canary + manual approval
  Production
  ├── 5% traffic → new pods (Argo Rollouts)
  ├── monitor error rate + p99 latency for 15 min
  ├── promote → 100% traffic to new pods
  └── old pods decommissioned

  Rollback at any stage:
  ├── kubectl rollout undo
  ├── flip load balancer (blue/green)
  └── drain canary to 0%
```

---

## 14. SDE-2 Mental Model — The Full Picture

CI/CD is the bridge between your code and running software. Docker/K8s is what runs in production. CI/CD is how it gets there, reliably, every time.

```
Developer commits
       │
       ▼
  CI Pipeline (GitHub Actions / GitLab CI)
  ├── Build & Test           — fast feedback on correctness
  ├── SAST / lint            — fast feedback on security & quality
  ├── docker build + push    — reproducible deployable artifact
  └── update config repo     — (GitOps) declares desired state
       │
       ▼
  GitOps Operator (ArgoCD)   — or direct kubectl in simpler setups
  └── reconciles cluster to desired state
       │
       ▼
  Kubernetes
  ├── Rolling update / Canary / Blue-Green
  ├── Readiness probes gate traffic until healthy
  └── Rollback = kubectl rollout undo / git revert
       │
       ▼
  Production — observable, auditable, reversible
```

**Image flow (always the same):**
```
Dockerfile → docker build → Image:sha-xxxxx → push → Registry
                                                          │
                                                          ▼
                                              Kubernetes pulls → Pod
```

> CI/CD is not about automation for its own sake. It's about making the feedback loop so short that shipping becomes boring — small, safe, reversible, and frequent.