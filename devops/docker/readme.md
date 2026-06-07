## 1. VM vs Container

**VM (Virtual Machine):** A software emulation of a physical computer. Each VM runs its own full guest OS (including its own kernel) on top of a hypervisor. This leads to higher resource consumption and slower startup times.

**Container:** A lightweight, standalone, executable package that includes everything needed to run a piece of software — code, runtime, system libraries, and config. Containers share the **host OS kernel**, making them more resource-efficient and faster to start than VMs.

| | VM | Container |
|---|---|---|
| OS | Full guest OS per VM | Shared host kernel |
| Startup | Minutes | Seconds |
| Size | GBs | MBs |
| Isolation | Strong (hypervisor-level) | Process-level (namespaces + cgroups) |

**How containers work internally:**
- **Linux namespaces** — provide isolation: each container gets its own process tree (PID), network stack, filesystem mount points, and hostname. Processes inside cannot see outside their namespace.
- **cgroups (control groups)** — enforce resource limits: how much CPU, memory, and I/O a container is allowed to consume.

> Together, namespaces + cgroups are what make a container feel like an isolated machine — without needing a separate kernel.

---

## 2. Why Do We Need Containers?

- **Portability:** Containers run consistently across dev, test, and prod environments — eliminates "works on my machine" issues.
- **Efficiency:** Containers share the host OS kernel, using fewer resources than VMs with faster startup times.
- **Scalability:** Containers can be scaled up or down quickly, making them ideal for microservices and cloud-native architectures.
- **Isolation:** Each container runs in its own namespace with resource limits via cgroups, improving security and preventing conflicts between services.
- **DevOps / CI/CD:** Containers are a core building block of CI/CD pipelines — the same image that passes tests is deployed to production.

---

## 3. What Is Docker?

Docker is an open-source platform that automates the building, shipping, and running of containers. It packages an application and all its dependencies into a **Docker image**, which can run as a **container** consistently across any environment.

Docker provides:
- A container runtime (`dockerd`)
- A CLI (`docker`)
- A build system (`docker build` via Dockerfile)
- A public registry (Docker Hub) for sharing images

---

## 4. Dockerfile vs Docker Compose

**Dockerfile** — defines *how to build* a single image.  
A text file with step-by-step instructions: base image, copy code, install dependencies, set the startup command.

```
Command: docker build -t myapp:latest .
```

**docker-compose.yml** — defines *how to run* a multi-container application.  
A YAML file that orchestrates multiple services (e.g., web app + database + cache), their networks, volumes, and environment variables, all as a single unit.

```
Command: docker-compose up -d
```

**Rule of thumb:**

| | Dockerfile | docker-compose.yml |
|---|---|---|
| Answers | How to BUILD the app | How to RUN/deploy the app |
| Scope | Single service image | Multi-service application |
| Owner | Dev team | DevOps team |
| Output | Docker image | Running containers |

> One container = one service. Docker Compose removes the need to run multiple `docker run` commands manually.

---

## 5. Dockerfile Instructions

```dockerfile
FROM node:18-alpine          # Base image to build on
WORKDIR /app                 # Set working directory inside the container
COPY . .                     # Copy files from host into the container
RUN npm install              # Execute commands at build time (installs deps)
EXPOSE 3000                  # Documents which port the container listens on (informational)
CMD ["node", "server.js"]    # Default command to run when the container starts
```

> `RUN` executes at **build time** (creates a new image layer). `CMD` executes at **container start time**. Only the last `CMD` takes effect.

### Multi-stage Builds

Used to reduce final image size. Common discussion question: *"How do you keep Docker images small?"*

The idea: use a heavy build image to compile, then copy only the output binary into a minimal runtime image.

```dockerfile
# Stage 1 — build (heavy image with full toolchain)
FROM rust:1.80 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2 — runtime (minimal image, no compiler)
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/myapp .
CMD ["./myapp"]
```

> The final image contains only the binary + slim OS — not the Rust compiler, source code, or build cache. Result: image drops from ~1.5GB to ~100MB.

**Answer for discussions:**
- Multi-stage builds
- Use minimal base images (`alpine`, `debian:slim`, `distroless`)
- Avoid copying unnecessary files (use `.dockerignore`)

---

## 6. docker-compose.yml Structure

```yaml
version: "3.9"

services:
  web:
    build: .                   # Build from local Dockerfile
    ports:
      - "3000:3000"
    environment:
      - NODE_ENV=production
    depends_on:
      - db

  db:
    image: postgres:15         # Pull pre-built image from registry
    volumes:
      - db-data:/var/lib/postgresql/data

networks:
  default:

volumes:
  db-data:
```

Key fields:
- `services` — each service maps to one container
- `build` — used for your own app (builds from Dockerfile); `image` — used for third-party services
- `ports` — maps `host:container`
- `volumes` — persistent storage
- `depends_on` — controls startup order (not readiness)
- `networks` — services on the same network can communicate by service name

---

## 7. Docker Build and Run Flow

```
Dockerfile
    │
    ▼  docker build -t myapp:latest .
Docker Image  (stored locally or in a registry)
    │
    ▼  docker run -p 3000:3000 myapp:latest
Container  (running instance of the image)
```

An **image** is immutable and read-only. A **container** is a running instance of an image with a writable layer on top.

---

## 8. Common Docker Commands

```bash
# Image management
docker build -t myapp:latest .    # Build image from Dockerfile in current dir
docker images                     # List local images
docker rmi myapp:latest           # Remove an image
docker pull nginx                 # Pull image from registry
docker push myapp:latest          # Push image to registry

# Container lifecycle
docker run -d -p 8080:80 nginx    # Run container in detached mode, map ports
docker ps                         # List running containers
docker ps -a                      # List all containers (including stopped)
docker stop <container_id>        # Gracefully stop a container
docker rm <container_id>          # Remove a stopped container
docker logs <container_id>        # View container logs
docker exec -it <id> /bin/sh      # Open interactive shell in a running container

# Docker Compose
docker-compose up -d              # Start all services in background
docker-compose down               # Stop and remove containers, networks
docker-compose logs -f            # Follow logs of all services
docker-compose ps                 # List service statuses
```

---

## 9. Docker Storage

Docker images are built in **layers** (one per Dockerfile instruction). Each layer is a filesystem diff on top of the previous one.

**Why layers?**
- **Reuse** — unchanged layers are shared across images and containers (e.g., the `node:18-alpine` base layer is downloaded once, reused by every image that uses it)
- **Storage efficiency** — only the changed layers are stored or transferred
- **Faster builds and pulls** — if a layer hasn't changed (e.g., `RUN npm install` with the same `package.json`), Docker reuses the cached layer instead of rebuilding it

> Practical tip: put instructions that change frequently (e.g., `COPY . .`) **after** instructions that change rarely (e.g., `RUN npm install`). This maximises cache hits.

```
/var/lib/docker/
 ├── overlay2/      ← union filesystem layers (image + container writable layer)
 ├── image/         ← image metadata and layer references
 ├── containers/    ← per-container config, logs, state
 ├── volumes/       ← named volumes (persistent data)
 └── networks/      ← network configs
```

**Volume types:**
- **Named volume** (`docker volume create`) — managed by Docker, persists across container restarts. Preferred for databases.
- **Bind mount** (`-v /host/path:/container/path`) — maps a host directory directly. Useful in development.
- **tmpfs** — in-memory only, not persisted.