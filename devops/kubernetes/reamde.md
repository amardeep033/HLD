## 1. Docker vs Kubernetes

| | Docker | Kubernetes (K8s) |
|---|---|---|
| Scope | Single machine | Cluster of machines |
| Role | Build and run containers | Orchestrate containers at scale |
| Scheduling | Manual | Automatic |
| Self-healing | No | Yes (restarts failed pods) |
| Load balancing | Manual / Compose | Built-in |
| Scaling | Manual | Auto-scaling (HPA) |

> **Key point:** Kubernetes does **not** build images. You still use Docker (or another tool like Buildah/Kaniko) to build images and push them to a container registry (Docker Hub, ECR, GCR). Kubernetes then pulls and deploys those images.

---

## 2. Kubernetes Core Concepts

- **Pod** — smallest deployable unit in K8s. **A Pod is NOT a container.** A Pod wraps one or more tightly coupled containers that share the same network namespace (i.e., same `localhost`) and volumes. Usually 1 container per pod; multi-container pods are for sidecars (e.g., a log shipper alongside the main app).
- **Deployment** — manages a desired number of pod replicas, handles rolling updates and rollbacks.
- **Service** — stable network endpoint to expose pods. Abstracts over pod IP changes. Types: `ClusterIP`, `NodePort`, `LoadBalancer`.
- **Namespace** — logical isolation within a cluster (e.g., `dev`, `staging`, `prod`).
- **ConfigMap / Secret** — inject config and sensitive data (env vars, credentials) into pods without baking them into the image.
- **PersistentVolume (PV) / PersistentVolumeClaim (PVC)** — storage abstraction for pods that need durable data.

**K8s flow:**
```
Docker Image → pushed to Registry
                        │
                        ▼
          Kubernetes pulls image → creates Pod(s)
                        │
                        ▼
          Deployment manages replicas → Service exposes them
```

---

## 3. Kubernetes Setup Example (Local with minikube)

This example deploys a simple Node.js app locally using minikube.

### 3.1 Step 1 — Prerequisites

```bash
# Install minikube (local single-node K8s cluster)
brew install minikube        # macOS
# or: https://minikube.sigs.k8s.io/docs/start/

# Install kubectl (K8s CLI)
brew install kubectl

# Start the cluster
minikube start
```

### 3.2 Step 2 — Build and push your Docker image

```bash
# Build the image
docker build -t myapp:latest .

# For minikube: load image directly instead of pushing to a registry
minikube image load myapp:latest
```

### 3.3 Step 3 — Create a Deployment

`deployment.yaml`
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: myapp-deployment
spec:
  replicas: 2                        # Run 2 pods
  selector:
    matchLabels:
      app: myapp
  template:
    metadata:
      labels:
        app: myapp
    spec:
      containers:
        - name: myapp
          image: myapp:latest
          imagePullPolicy: Never     # Use local image (minikube only)
          ports:
            - containerPort: 3000
```

### 3.4 Step 4 — Expose it with a Service

`service.yaml`
```yaml
apiVersion: v1
kind: Service
metadata:
  name: myapp-service
spec:
  type: NodePort                     # Exposes on a port on the node
  selector:
    app: myapp                       # Routes traffic to pods with this label
  ports:
    - port: 80                       # Service port
      targetPort: 3000               # Container port
      nodePort: 30080                # External port (30000–32767)
```

### 3.5 Step 5 — Apply and verify

```bash
kubectl apply -f deployment.yaml
kubectl apply -f service.yaml

# Check pods are running
kubectl get pods

# Check service
kubectl get service myapp-service

# Open in browser (minikube only)
minikube service myapp-service
```

### 3.6 Step 6 — Useful kubectl commands

```bash
kubectl get pods                          # List pods
kubectl get deployments                   # List deployments
kubectl describe pod <pod-name>           # Detailed pod info (good for debugging)
kubectl logs <pod-name>                   # View pod logs
kubectl exec -it <pod-name> -- /bin/sh    # Shell into a pod
kubectl scale deployment myapp-deployment --replicas=4   # Scale up
kubectl rollout restart deployment myapp-deployment      # Rolling restart
kubectl delete -f deployment.yaml         # Tear down
```

---

## 4. Practical SDE-2 Mental Model

For most backend discussions, you should be able to map this end-to-end:

**Stack example: Rust Actix service + PostgreSQL + Redis**

```
Local development
└── docker-compose.yml
    ├── actix service  (built from Dockerfile, multi-stage)
    ├── postgres       (image: postgres:15)
    └── redis          (image: redis:7-alpine)

Production
└── Kubernetes
    ├── Deployment  → runs actix pods (2+ replicas)
    ├── Service     → exposes actix to the outside
    ├── Secret      → holds DB credentials
    └── postgres/redis as managed cloud services (RDS, Elasticache)
         or separate Deployments with PersistentVolumeClaims
```

**Image flow (always the same):**
```
Dockerfile → docker build → Image → docker push → Registry
                                                      │
                                                      ▼
                                              Kubernetes pulls → Pod
```