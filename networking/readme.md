# Networking Components — SDE-2 Deep Study Notes

Story-driven notes covering CDN, Proxy, Load Balancer, API Gateway, VPN, DNS, Cache, and OSI Layers from first principles.

---

## Table of Contents

0. [The Core Confusion](#0-the-core-confusion)
1. [The Internet Before These Existed](#1-the-internet-before-these-existed)
2. [The Big Mental Model](#2-the-big-mental-model)
3. [DNS — The Phone Book](#3-dns--the-phone-book)
4. [CDN — Content Delivery Network](#4-cdn--content-delivery-network)
5. [Cache — Store What You Already Fetched](#5-cache--store-what-you-already-fetched)
6. [Proxy and Reverse Proxy](#6-proxy-and-reverse-proxy)
7. [Load Balancer](#7-load-balancer)
8. [API Gateway](#8-api-gateway)
9. [VPN — Virtual Private Network](#9-vpn--virtual-private-network)
10. [OSI Layer Mapping](#10-osi-layer-mapping)
11. [The Most Common Confusions](#11-the-most-common-confusions)
12. [Real Production Request Flow](#12-real-production-request-flow)
13. [When You Add Each Component](#13-when-you-add-each-component)
14. [Interview-Level Insights](#14-interview-level-insights)
15. [Practical NGINX Examples](#15-practical-nginx-examples)
16. [Final Mental Shortcuts](#16-final-mental-shortcuts)

---

## 0. The Core Confusion

### 0.1 Why Everyone Gets Confused

Most developers confuse these components because:

- They all **"sit in the middle"** of a request
- They all **"forward traffic"** in some way
- Many tools **do multiple jobs** — NGINX can be a reverse proxy, load balancer, API gateway, AND a cache simultaneously
- Cloud providers blur boundaries even more

### 0.2 The Right Mental Model

Instead of memorizing definitions, ask one question:

> **"What problem is this component protecting the system from?"**

That one question instantly clarifies where each component belongs and why it exists.

| Component | Protects Against |
|---|---|
| CDN | Latency for global users; origin server overload |
| DNS | Humans having to remember IP addresses |
| Cache | Repeating expensive computations or fetches |
| Forward Proxy | Lack of anonymity; unfiltered client access |
| Reverse Proxy | Backend exposure; SSL complexity; routing chaos |
| Load Balancer | Single point of failure; server overload |
| API Gateway | Auth duplication; insecure microservices; scattered observability |
| VPN | Unencrypted traffic; exposed internal systems |

---

## 1. The Internet Before These Existed

### 1.1 The Pain

Imagine 2005 YouTube without modern infrastructure.

A user in India opens a video hosted in California.

```text
India User
    |
Single US Server
```

What actually happened:

- **Huge latency** — packets traveled thousands of miles each way
- **Video buffering** — bandwidth shared across millions of concurrent users
- **Server overload** — one server handled everything
- **One crash = website down** — no redundancy whatsoever
- **DDoS destroys everything** — no edge protection
- **APIs exposed directly to internet** — no authentication layer
- **Every microservice handles auth separately** — copy-paste security nightmare
- **Internal company traffic routed over public internet** — massive security risk

### 1.2 What Was Actually Breaking

As systems scaled, engineers started inserting **"smart middle layers"** between users and servers.

Every component in this document is one of those smart middle layers — invented to fix a specific pain.

> The core insight: **you don't add these components for fun. You add them the moment a specific pain appears.**

---

## 2. The Big Mental Model

### 2.1 The City Analogy

Think of modern systems like a city:

```text
                    INTERNET
                        |
                      [DNS]          <- Phone book: domain -> IP
                        |
                  +-----------+
                  |    CDN    |      <- Local delivery stores around the world
                  +-----------+
                        |
              +------------------+
              |  Reverse Proxy   |  <- Security guard at the front door
              |  / API Gateway   |  <- Smart concierge who checks IDs
              +------------------+
                        |
               +----------------+
               |  Load Balancer |   <- Traffic cop distributing cars
               +----------------+
                        |
          +-------------+--------------+
          |             |              |
        App1          App2           App3   <- The actual workers
          |
        Cache          <- Notepad on the desk
          |
          DB            <- The filing cabinet
```

And separately:

```text
Employee Laptop
      |
     VPN            <- Encrypted private tunnel to the office
      |
Internal Company Network
```

### 2.2 One-Line Job Description Per Component

| Component | One-Line Job |
|---|---|
| DNS | Convert domain name -> IP address |
| CDN | Serve content from the nearest location to the user |
| Cache | Store a result so you don't recompute or refetch it |
| Forward Proxy | Forward requests on behalf of the client |
| Reverse Proxy | Mediate requests on behalf of the backend |
| Load Balancer | Distribute traffic across multiple servers |
| API Gateway | Centralized entry point for all API traffic |
| VPN | Create an encrypted private tunnel |

---

## 3. DNS — The Phone Book

### 3.1 The Problem Before DNS

In the early internet, to reach a server you had to know its **IP address**:

```text
142.251.42.78
```

Nobody could remember IP addresses. And IPs change when servers move. You needed a stable name that always resolved to the right server.

### 3.2 What DNS Does

DNS converts human-readable domain names into machine-readable IP addresses.

```text
google.com  ->  142.251.42.78
```

That is the entire job.

### 3.3 Why DNS Matters for Everything Else

DNS is the **first thing that runs** when you type a URL. Every other component (CDN, load balancer, gateway) depends on traffic arriving — and DNS controls where that traffic points.

CDNs specifically **hijack DNS** to redirect users to the nearest edge server. More on this in Section 4.

### 3.4 DNS Resolution Flow

```text
You type: google.com
    |
Browser checks local cache
    | (cache miss)
OS checks /etc/hosts
    | (not there)
Recursive DNS resolver (your ISP)
    |
Root Name Server -> TLD server (.com)
    |
Authoritative DNS for google.com
    |
Returns IP: 142.251.42.78
    |
Browser connects
```

### 3.5 OSI Layer

DNS operates at **Layer 7 (Application)** practically — it uses UDP/TCP port 53 (Layer 4 transport), but the DNS protocol itself is an application-layer service.

---

## 4. CDN — Content Delivery Network

### 4.1 The Problem Before CDN

Netflix has one origin server in the US. Users are in India, Japan, Germany, Brazil.

```text
India User   ---------------------->  US Server  (200ms+)
Japan User   ---------------------->  US Server  (150ms+)
Germany User ---------------------->  US Server  (120ms+)
Brazil User  ---------------------->  US Server  (180ms+)
```

Every request travels thousands of miles. Every user feels it.

**Pain:**

- High latency — especially for large files (video, images, CSS, JS)
- Massive bandwidth costs on the origin server
- Origin server overloaded serving the same static files repeatedly to everyone
- One origin server crash = global outage

### 4.2 Why CDN Was Invented

To move content **physically closer to users**.

Instead of:

```text
India -> US Server  (200ms)
```

You do:

```text
India -> Mumbai CDN Edge  (10ms)
```

### 4.3 What a CDN Actually Does

> A CDN **caches and serves content from geographically distributed edge servers** close to users.

That is the complete job description.

Your origin server is the **warehouse**. CDN edges are **local delivery stores**.

### 4.4 How CDN Works — The Full Request Flow

```text
Step 1: User types  video.netcinema.com/video.mp4

Step 2: DNS resolution
        -> NetCinema's DNS says: "Don't hit me directly, use KingCDN"
        -> Returns: a1105.kingcdn.com

Step 3: CDN DNS picks nearest edge
        -> Checks user IP, geography, latency, server load
        -> Returns: Mumbai Edge IP

Step 4: User hits Mumbai CDN Edge
        -> Cached?     -> Serve instantly from edge
        -> Not cached? -> Pull from origin, cache locally, return to user

Step 5: Future users in Mumbai get it from cache instantly
```

### 4.5 CDN OSI Layers

CDNs primarily operate at:

- **Layer 7 (HTTP/HTTPS)** — inspects URLs, headers, cookies, cache-control directives, content type
- **Layer 4 (TCP/UDP)** — sometimes also for TCP acceleration and DDoS mitigation

### 4.6 When You Need CDN

You reach for CDN when:

- You have **global users** and are serving static files (images, videos, JS, CSS)
- **Bandwidth is expensive** and you want to reduce origin server load
- You need **DDoS protection** — CDN edge absorbs the attack before it reaches your origin
- **Frontend assets are large** and need caching close to users

You do NOT need CDN when:

- Internal admin tools with no public traffic
- Low-traffic MVP still finding product-market fit
- Tiny backend-only APIs with no static assets
- Region-locked products where global delivery adds no value

### 4.7 CDN vs Cache — The Confusion

> People confuse CDN and Cache constantly. Here is the clean distinction.

| | Cache | CDN |
|---|---|---|
| Concept | Generic "store for reuse" | Specialized distributed HTTP cache |
| Location | Anywhere (Redis, browser, server) | Globally distributed edge servers |
| Protocol-aware | No | Yes — understands HTTP cache headers |
| Geographic distribution | No | Yes — core purpose |
| Use case | Any repeated data | Internet content delivery |

**Key insight:** A CDN *is* a cache — but a cache is not a CDN. CDN is a globally distributed, HTTP-aware, geographically optimized caching network.

### 4.8 Common CDNs Used in Industry

- **Cloudflare** — most common for SDE interviews
- **Akamai** — enterprise-grade
- **AWS CloudFront** — tight AWS integration
- **Fastly** — popular for edge computing

---

## 5. Cache — Store What You Already Fetched

### 5.1 The Problem Before Caching

Every user request hits the database. Every time.

```text
User A -> Server -> DB -> (compute result) -> return
User B -> Server -> DB -> (same result)    -> return  <- wasteful
User C -> Server -> DB -> (same result)    -> return  <- wasteful
```

At scale, 80% of requests fetch the same data. The DB gets crushed for no reason.

### 5.2 What Cache Does

> A cache **stores the result of an expensive operation temporarily** so future requests return it instantly without recomputing.

```text
User A -> Server -> DB -> result -> STORE in Cache
User B -> Server -> Cache -> return instantly
User C -> Server -> Cache -> return instantly
```

### 5.3 Types of Cache

| Cache Type | Where It Lives | What It Caches | Example |
|---|---|---|---|
| Browser cache | User's browser | HTML, JS, CSS, images | Browser localStorage |
| CDN cache | Edge servers worldwide | Static assets | Cloudflare |
| Application cache | Your server memory | DB query results, API responses | Redis, Memcached |
| CPU cache | CPU chip | Instructions and data | L1/L2/L3 cache |
| DB buffer cache | DB process memory | Index pages, query results | PostgreSQL shared buffers |

### 5.4 Cache Invalidation — The Hard Problem

> **"There are only two hard things in computer science: cache invalidation and naming things."** — Phil Karlton

When data changes in the DB, the cache still holds the old value. You must invalidate it.

Strategies:

- **TTL (Time-to-live)** — cache expires automatically after N seconds
- **Write-through** — update cache and DB simultaneously on every write
- **Write-back** — update cache first, flush to DB asynchronously (risk of data loss on crash)
- **Cache-aside (lazy loading)** — only populate cache on a cache miss; app manages invalidation

### 5.5 OSI Layer

Cache is an **application-level concept** — it does not map to a specific OSI layer. The application logic at Layer 7 decides what to cache, when, and for how long.

---

## 6. Proxy and Reverse Proxy

### 6.1 The Problem Before Proxies

Clients directly accessed the internet. No filter. No control. No anonymity.

```text
Employee Laptop -> directly -> Internet (any site, any request, no audit trail)
```

**Pain for corporations:**

- No way to block inappropriate or malicious websites
- No audit trail of employee internet traffic
- Employees could leak data or pull in malware

**Pain for backend servers:**

- Directly exposed to the internet
- Every client knows the real server IP
- SSL certificates must be managed on every individual backend service
- A vulnerability in one service exposed everything

### 6.2 The Two Completely Different Proxies

> This is where most confusion starts. There are TWO types of proxy, and they face **opposite directions**.

```text
Forward Proxy:   Client  ->  [Forward Proxy]  ->  Internet
Reverse Proxy:   Internet  ->  [Reverse Proxy]  ->  Backend Servers
```

### 6.3 Forward Proxy

#### 6.3.1 What It Does

A forward proxy sits in front of **clients**. It forwards client requests to the internet on their behalf.

```text
Employee -> Corporate Proxy -> Internet
```

The website sees the **proxy's IP**, not the employee's IP.

#### 6.3.2 Use Cases

- **Corporate filtering** — block social media, adult sites, malware domains
- **Anonymity** — hide user IP from external servers
- **Geo bypass** — access region-restricted content
- **Traffic monitoring** — audit and log all employee requests

#### 6.3.3 Examples

- Squid proxy
- Corporate or school firewall proxy

### 6.4 Reverse Proxy

#### 6.4.1 What It Does

A reverse proxy sits in front of **backend servers**. It intercepts internet traffic before it reaches your servers.

```text
Internet -> Reverse Proxy -> Backend Servers
```

Users never directly see or reach your backend servers.

#### 6.4.2 What Reverse Proxy Handles

- **SSL termination** — handles HTTPS externally; backend stays plain HTTP internally
- **Routing** — `/api` goes to backend cluster; `/static` served directly
- **Compression** — gzip responses before sending to clients
- **Caching** — serve repeated responses without hitting backend
- **Security** — hides real server IPs; absorbs malformed requests early

#### 6.4.3 Examples

- **NGINX** — most common
- **Traefik** — popular in Kubernetes environments
- **HAProxy** — high-performance TCP + HTTP

> **Key insight:** A reverse proxy protects and mediates **server-side** traffic. A forward proxy protects and controls **client-side** traffic.

### 6.5 Proxy vs VPN — The Confusion

| | Forward Proxy | VPN |
|---|---|---|
| What it covers | Only specific app traffic (browser) | All device traffic |
| Encryption | Usually none | Always encrypted |
| OSI Layer | Layer 7 (application) | Layer 3/4 (network) |
| Speed | Faster | Slower due to encryption overhead |
| Use case | Filtering, anonymity | Secure private tunneling |

> **Simple rule:** Proxy forwards traffic. VPN encrypts and tunnels all traffic.

---

## 7. Load Balancer

### 7.1 The Problem Before Load Balancers

One server handled all traffic.

```text
All Users -> Single Server
```

At peak load:

- CPU maxed out
- Memory exhausted
- One crash = complete downtime for everyone
- Vertical scaling (buying a bigger server) has a hard ceiling

You cannot buy your way out of this forever.

### 7.2 Why Load Balancer Was Invented

To scale **horizontally** — add more servers instead of a bigger server.

```text
           Load Balancer
          /      |       \
        S1      S2       S3
```

### 7.3 What a Load Balancer Actually Does

> A load balancer **distributes incoming traffic across multiple backend servers** to prevent overload and ensure availability.

### 7.4 Load Balancing Strategies

#### 7.4.1 Round Robin

```text
Request 1 -> Server 1
Request 2 -> Server 2
Request 3 -> Server 3
Request 4 -> Server 1  (repeats)
```

Simple. Equal distribution. Does not account for varying server load.

#### 7.4.2 Least Connections

Send each new request to the server with the fewest active connections. Good for workloads that vary in processing time.

#### 7.4.3 Weighted Round Robin

```text
Server 1 (weight=3) -> gets 3x requests
Server 2 (weight=1) -> gets 1x requests
```

Powerful servers with more CPU/RAM get a higher weight.

#### 7.4.4 IP Hash

Same client IP always routes to the same server. Useful for **session affinity** when session state is server-local.

#### 7.4.5 Health Checks

```text
LB constantly sends: GET /health -> each server
    -> 200 OK   -> server stays in rotation
    -> timeout  -> server removed automatically
    -> 500      -> server removed automatically
```

> **Health checks are a critical interview topic.** They are how LBs achieve zero-downtime deploys and survive server crashes silently.

### 7.5 L4 vs L7 Load Balancers

This is a very common interview question.

| | L4 Load Balancer | L7 Load Balancer |
|---|---|---|
| OSI Layer | Layer 4 (TCP/UDP) | Layer 7 (HTTP/HTTPS) |
| What it inspects | IP address + port only | URL, headers, cookies, body |
| Content-aware routing | No | Yes |
| Speed | Faster | Slightly slower (HTTP parsing) |
| Use case | Raw TCP balancing | HTTP-aware intelligent routing |
| Examples | AWS NLB | AWS ALB, NGINX |

**L7 can do path-based routing — L4 cannot:**

```text
/api      -> API server cluster
/images   -> image server cluster
/checkout -> payment server cluster
```

L4 sees only IP and port. It has no idea what URL is in the request.

### 7.6 Sticky Sessions

Sometimes you need the **same user to always hit the same server** — for example when session state is stored in server memory or when using WebSockets.

```text
User A -> always routes to Server 1
User B -> always routes to Server 2
```

The LB uses a **session cookie** to track which server each user belongs to.

> **Tradeoff:** Sticky sessions break perfect load distribution. If Server 1 gets heavy users, it becomes a hotspot while others sit idle.
>
> **Modern fix:** Externalize session state to Redis. Then any server can serve any user, and sticky sessions become unnecessary.

### 7.7 LB vs Reverse Proxy — The Confusion

> Almost everyone confuses these. Here is the clean line.

| | Load Balancer | Reverse Proxy |
|---|---|---|
| Primary concern | Traffic distribution | Request mediation |
| What it solves | Scalability, availability | Security, SSL, routing, caching |
| Examples | AWS NLB, ALB | NGINX, Traefik |

> **The catch:** A reverse proxy CAN load balance. NGINX does both. That is exactly why the boundaries blur — but their **primary purpose** is different.

---

## 8. API Gateway

### 8.1 The Problem Before API Gateway

Microservices exploded. You now have 10 services and each client calls them directly:

```text
Mobile App -> /auth-service
Mobile App -> /user-service
Mobile App -> /payment-service
Mobile App -> /order-service
Mobile App -> /search-service
```

**Pain:**

- Every service reimplements JWT validation (duplicated 10 times)
- Inconsistent rate limiting — one service allows abuse, another does not
- Clients must know every internal service URL
- No central observability — logs scattered across 10 services
- Security vulnerabilities multiply because every service is its own perimeter

### 8.2 Why API Gateway Was Invented

To create a **single, smart entry point** that handles cross-cutting concerns once — not 10 times.

```text
Mobile App -> API Gateway -> /auth-service
                          -> /user-service
                          -> /payment-service
                          -> /order-service
```

### 8.3 What an API Gateway Actually Does

> An API Gateway is a **centralized entry point that manages, secures, routes, and transforms API traffic** before it reaches your microservices.

### 8.4 Responsibilities of an API Gateway

| Responsibility | What It Does |
|---|---|
| Authentication | Validates JWT/OAuth tokens centrally — once for all services |
| Rate Limiting | 100 req/min/user — enforced at the gateway, not in each service |
| Routing | /orders -> order-service; /auth -> auth-service |
| Request Transformation | Convert JSON to XML, add headers, reshape payloads |
| SSL Termination | HTTPS externally, plain HTTP internally |
| Logging & Monitoring | Centralized observability for all API traffic |
| Circuit Breaking | Stop forwarding to a failed downstream service |

### 8.5 API Gateway vs Reverse Proxy — The Confusion

> These look identical from the outside. The difference is in intelligence.

| | Reverse Proxy | API Gateway |
|---|---|---|
| Awareness | Generic traffic mediator | Application-aware API manager |
| Auth | No | Yes |
| Rate limiting | No | Yes |
| Analytics | No | Yes |
| Routing basis | URL path | URL + headers + JWT claims |
| Use case | Any HTTP traffic | Specifically API management |
| Examples | NGINX (basic config) | Kong, AWS API Gateway, Apigee |

> **Simple rule:** Reverse proxy is a dumb forwarder. API Gateway is a smart forwarder that understands APIs.

### 8.6 OSI Layer

API Gateway operates at **Layer 7** — it inspects full HTTP requests, headers, and JWT tokens.

### 8.7 When You Need API Gateway

You reach for an API Gateway when:

- You have **multiple microservices** that clients need to reach
- You need **centralized auth** instead of reimplementing JWT in every service
- You need **rate limiting** to protect services from abuse
- You want a **single external URL** mapping to many internal services

You do not need it when:

- You have a monolith
- You have one or two backend services
- NGINX-level routing is already sufficient

---

## 9. VPN — Virtual Private Network

### 9.1 The Problem Before VPN

Employees working remotely connected to company resources over the public internet.

```text
Employee Home -> Public Internet -> Company Server
```

**Pain:**

- All traffic visible to ISPs and network attackers
- Internal company systems (databases, dashboards, CI/CD) forced to be internet-exposed
- Remote employee looks like an external stranger to the company network
- No way to enforce company firewall or security policies on remote traffic

### 9.2 Why VPN Was Invented

To make **remote traffic behave like it is physically inside the office network**.

```text
Employee Home -> [Encrypted Tunnel] -> VPN Server -> Company Internal Network
```

From the company network's perspective, the employee is sitting at their office desk.

### 9.3 What a VPN Actually Does

> A VPN creates an **encrypted private tunnel** between your device and a VPN server, so all your traffic is secure and appears to originate from the VPN server's network.

### 9.4 VPN Protocols

| Protocol | Layer | Notes |
|---|---|---|
| OpenVPN | L3 (IP) | Widely used, open source, solid |
| WireGuard | L3/L4 | Modern, fast, minimal codebase |
| IPSec | L3 | Enterprise standard, complex to configure |
| PPTP | L2 | Old, insecure — avoid entirely |

### 9.5 OSI Layer

VPN primarily operates at **Layer 3 (Network)** — it creates a virtual network interface with a private IP address. WireGuard operates at L3/L4.

### 9.6 What Happens Without VPN

- Internal systems (databases, internal dashboards, CI/CD pipelines) must be exposed to the internet
- Attackers can attempt direct connections to internal services
- Employee traffic over a coffee shop Wi-Fi is readable by anyone on the same network

### 9.7 VPN vs Proxy — Final Clarity

| | Proxy | VPN |
|---|---|---|
| Scope | Usually one application (browser) | Entire device |
| Encryption | Usually none | Always |
| OSI Layer | L7 | L3 |
| Hides IP | Yes | Yes |
| Hides content | No | Yes |
| Use case | Filtering, anonymity, geo bypass | Secure private networking |
| Examples | Squid, corporate firewall | NordVPN, WireGuard, OpenVPN |

---

## 10. OSI Layer Mapping

### 10.1 Why This Matters for Interviews

Interviewers ask: *"At what layer does an API Gateway operate?"*

Knowing this shows you understand **what data each component can see and act on**.

- A **Layer 4** component sees: IP address + port. That is it.
- A **Layer 7** component sees: the full HTTP request — URL, headers, cookies, body.

> **Interview anchor:** When asked "what layer does X operate at?", trace it — Can it see the HTTP URL? -> L7. Can it only see TCP ports? -> L4. Does it deal with IPs and routing? -> L3.

### 10.2 Component to OSI Layer Mapping

| Component | Operates At | Why |
|---|---|---|
| CDN | L7 (+ L4 sometimes) | Inspects HTTP cache headers, URLs, content type |
| API Gateway | L7 | Routes based on URLs, JWT tokens, headers |
| Reverse Proxy | L7 | Handles HTTP, SSL termination, cookies |
| L7 Load Balancer | L7 | Path-based routing, cookie inspection |
| L4 Load Balancer | L4 | Routes by IP + port, no HTTP inspection |
| VPN | L3 | Creates virtual IP network tunnel |
| DNS | L7 (application service) | Name resolution protocol |
| Redis Cache | Application / L7 | App-level data caching |
| TLS/SSL | L6 | Encryption and decryption layer |
| NGINX | L4 / L7 | Can proxy raw TCP or full HTTP |
| HAProxy | L4 / L7 | TCP and HTTP load balancing |
| QUIC | L4-ish | Transport protocol over UDP |
| WebSocket | L7 over TCP | Persistent bidirectional communication |

### 10.3 Full OSI Model Reference

| Layer | Name | What It Does | Protocols / Tech |
|---|---|---|---|
| 7 | Application | User-facing network services | HTTP, HTTPS, DNS, FTP, SMTP, SSH, DHCP, WebSocket, gRPC, GraphQL |
| 6 | Presentation | Encryption, encoding, serialization | TLS, SSL, JPEG, PNG, JSON, Protobuf, UTF-8 |
| 5 | Session | Maintains communication sessions | NetBIOS, RPC, TLS Session Resumption |
| 4 | Transport | End-to-end host communication | TCP, UDP, QUIC, SCTP |
| 3 | Network | Routing packets across networks | IPv4, IPv6, ICMP, IPSec, BGP, OSPF |
| 2 | Data Link | Communication within local network | Ethernet, Wi-Fi (802.11), ARP, VLAN, MAC |
| 1 | Physical | Physical bit transmission | Fiber optics, Ethernet cables, Radio/Wi-Fi signals |

---

## 11. The Most Common Confusions

### 11.1 CDN vs Load Balancer

| | CDN | Load Balancer |
|---|---|---|
| Purpose | Content delivery speed | Traffic distribution |
| Location | Edge — globally distributed PoPs | Data center — centralized |
| What it distributes | Content (static files) | Requests across backend servers |
| Caches content? | Yes — core purpose | No |
| OSI layer | L7 (+ L4) | L4 or L7 |

> **Mental model:** CDN distributes *copies of your content* across the globe. LB distributes *requests* across your servers.

### 11.2 CDN vs Reverse Proxy

| | CDN | Reverse Proxy |
|---|---|---|
| Location | Globally distributed edges | Centralized in your data center |
| Primary purpose | Cache and serve static content | Mediate and secure backend traffic |
| Geographic distribution | Yes — core purpose | No |

### 11.3 API Gateway vs Load Balancer

| | API Gateway | Load Balancer |
|---|---|---|
| Purpose | Manage and secure APIs | Distribute traffic |
| Auth | Yes | No |
| Rate limiting | Yes | No |
| Routing intelligence | High (JWT, path, headers) | Low-to-medium (URL path or IP) |

### 11.4 Forward Proxy vs Reverse Proxy

| | Forward Proxy | Reverse Proxy |
|---|---|---|
| Protects | Clients | Servers |
| Who configures it | End user / IT department | System architect / DevOps |
| What it hides | Client IP from internet | Server IP from internet |
| Placement | In front of clients | In front of servers |

> **One-liner:** Forward proxy = client's bodyguard. Reverse proxy = server's bodyguard.

### 11.5 VPN vs Proxy

| | VPN | Proxy |
|---|---|---|
| Scope | All device traffic | Specific app only |
| Encryption | Yes | No |
| Layer | L3 | L7 |
| Hides IP | Yes | Yes |
| Hides content | Yes | No |

---

## 12. Real Production Request Flow

### 12.1 The Full Journey — "User Opens Amazon"

This is what interviewers love.

```text
1. You type: amazon.com
   -> Browser asks DNS resolver

2. DNS resolver returns CDN edge IP
   -> Static assets: images, JS, CSS served from CDN edge

3. Dynamic requests go through:
   -> WAF (Web Application Firewall) — blocks SQLi, XSS, bad bots
   -> API Gateway — authenticates JWT, rate-limits per user
   -> Load Balancer — distributes to available backend server

4. Backend services respond
   -> Check Redis cache first
   -> Cache miss -> query DB
   -> Return response up the chain
```

### 12.2 Architecture Diagram

```text
User
  |
DNS
  |
CDN                         (static assets: images, CSS, JS, fonts)
  |
WAF                         (Web Application Firewall — blocks SQLi, XSS)
  |
API Gateway / Reverse Proxy (auth, rate-limit, routing)
  |
Load Balancer
  |
Microservices (App1, App2, App3)
  |
Redis Cache  ->  DB (Postgres, MySQL)
             ->  Message Queue (Kafka)
```

### 12.3 Where Each Component Actually Lives

```text
Layer: Edge  (CDN PoPs worldwide)
    CDN

Layer: Entry point  (your data center)
    WAF -> API Gateway -> Load Balancer

Layer: Compute  (your servers)
    Microservices

Layer: Storage
    Redis / DB / Kafka
```

---

## 13. When You Add Each Component

### 13.1 The Evolution of a System

#### Phase 1 — Startup (Day 1)

```text
User -> App Server -> DB
```

Simple. One server. Works fine. Ship it.

#### Phase 2 — Growing (First scale problem)

```text
User -> Load Balancer -> [App1, App2, App3] -> DB
```

You add a **Load Balancer** when:

- One server keeps crashing under peak load
- You need zero-downtime deploys
- You are ready to scale horizontally

#### Phase 3 — Global Traffic

```text
User -> CDN -> Load Balancer -> App Servers -> DB
```

You add a **CDN** when:

- Users outside your region complain about latency
- Your origin server is overwhelmed serving the same static files repeatedly
- Bandwidth costs are ballooning

#### Phase 4 — Microservices

```text
User -> API Gateway -> Load Balancer -> [Service1, Service2, Service3]
```

You add an **API Gateway** when:

- You split into microservices
- Every service reimplements auth
- You need centralized rate limiting and observability

#### Phase 5 — Enterprise Security

```text
Remote Employee -> VPN -> Internal Network -> Databases/Services
```

You add a **VPN** when:

- Employees need remote access to internal systems
- Internal systems must not be public-internet accessible

### 13.2 Summary Table

| Problem | Component to Add |
|---|---|
| Server crashes under load | Load Balancer |
| Global users have high latency | CDN |
| Auth duplicated across services | API Gateway |
| Backend needs SSL + routing + security | Reverse Proxy |
| Employees need remote internal access | VPN |
| Repeated DB queries overloading DB | Cache (Redis) |
| DNS/routing control needed | DNS configuration |

---

## 14. Interview-Level Insights

These separate SDE-1 from SDE-2 answers.

### 14.1 CDN Is Bad for Highly Dynamic Personalized Data

CDN caches the same content for all users. If the response is personalized (user profile, cart, recommendations), caching becomes:

- **Incorrect** — User A could see User B's cached data
- **Complex** — cache keys per user defeat the purpose of a CDN

> CDN is for static or **shared** content. Never blindly cache dynamic, user-specific data at the CDN layer.

### 14.2 L4 LBs Are Faster Than L7

L4 load balancers work at the TCP level — they do not open the HTTP packet at all.

- **Faster** — less processing per packet
- **Lower latency** — no HTTP parsing overhead
- **Better for non-HTTP protocols** — raw TCP, UDP, gRPC streams

Use L4 for raw throughput. Use L7 when you need intelligent HTTP routing.

### 14.3 API Gateway Can Become a Single Point of Failure

The API Gateway handles ALL traffic. If it goes down, everything goes down.

Fix:

- Run multiple gateway instances behind a load balancer
- Enable health checks
- Design for horizontal scaling from day one

### 14.4 Reverse Proxy Handles SSL Termination

```text
Client  <-- HTTPS -->  Reverse Proxy  <-- HTTP -->  Backend
```

Backend services stay plain HTTP internally. Certificates are managed only at the proxy. This:

- Simplifies certificate management (one place, not everywhere)
- Reduces CPU overhead on backend servers (SSL/TLS is expensive)
- Allows internal HTTP-only service communication

### 14.5 CDN Absorbs DDoS

CDN edges are distributed across hundreds of PoPs worldwide. A DDoS attack hits the edge network — which has massive bandwidth capacity — not your origin. Cloudflare regularly absorbs terabit-scale attacks this way.

### 14.6 Sticky Sessions Break Equal Distribution

When LB routes the same user to the same server (for session state), it creates **hotspots** — heavy users pile onto one server while others sit idle.

> **The modern fix:** Externalize session state to Redis. Then any server can serve any user, sticky sessions are unnecessary, and your LB distributes perfectly again.

### 14.7 Health Checks Enable Zero-Downtime Deploys

```text
Blue-Green Deploy:
1. Deploy new version to Server 2 (Green)
2. Health check: Green passes -> becomes healthy
3. LB gradually shifts traffic from Blue to Green
4. Server 1 (Blue) drains to zero traffic
5. Blue updated -> becomes new standby
```

No downtime. No dropped requests. Health checks make this automatic.

---

## 15. Practical NGINX Examples

### 15.1 Reverse Proxy

```nginx
server {
    listen 80;

    location / {
        proxy_pass http://backend:8080;
    }
}
```

### 15.2 Load Balancer

```nginx
upstream backend {
    server app1:8080;
    server app2:8080;
    server app3:8080;
}

server {
    listen 80;

    location / {
        proxy_pass http://backend;
    }
}
```

### 15.3 Weighted Load Balancer

```nginx
upstream backend {
    server app1:8080 weight=3;
    server app2:8080 weight=1;
}
```

### 15.4 API Gateway Style Routing

```nginx
server {
    listen 80;

    location /auth {
        proxy_pass http://auth-service:8080;
    }

    location /orders {
        proxy_pass http://order-service:8081;
    }

    location /payments {
        proxy_pass http://payment-service:8082;
    }
}
```

### 15.5 CDN Cache Headers

```nginx
location /static/ {
    # Cache standard assets for 1 hour
    add_header Cache-Control "public, max-age=3600";
}

location /assets/ {
    # Immutable assets (hashed filenames like app.a1b2c3.js) — cache forever
    add_header Cache-Control "public, max-age=31536000, immutable";
}
```

### 15.6 SSL Termination at Reverse Proxy

```nginx
server {
    listen 443 ssl;
    ssl_certificate     /etc/ssl/certs/cert.pem;
    ssl_certificate_key /etc/ssl/private/key.pem;

    location / {
        proxy_pass http://backend:8080;  # internal plain HTTP
    }
}
```

---

## 16. Final Mental Shortcuts

### 16.1 The Decision Tree

When confused, ask these questions in order:

```text
Is this helping USERS reach content faster?
    -> CDN

Is this translating domain names to IPs?
    -> DNS

Is this distributing traffic across servers?
    -> Load Balancer

Is this managing APIs / auth / rate-limiting?
    -> API Gateway

Is this forwarding requests ON BEHALF OF clients?
    -> Forward Proxy

Is this protecting / hiding backend servers?
    -> Reverse Proxy

Is this creating secure private networking?
    -> VPN

Is this storing results to avoid re-fetching?
    -> Cache
```

### 16.2 The One-Liner Cheat Sheet

| Component | One Liner |
|---|---|
| CDN | Deliver content from the nearest edge to the user |
| DNS | Convert domain name -> IP address |
| Cache | Store results so you do not recompute them |
| Forward Proxy | Forward requests on behalf of clients |
| Reverse Proxy | Protect and mediate backend server traffic |
| Load Balancer | Spread incoming traffic across multiple servers |
| API Gateway | Centralized smart entry point for all API traffic |
| VPN | Encrypted private tunnel between device and network |

### 16.3 What Interviewers Actually Test

They do not test definitions. They test reasoning.

| What They Ask | What They Are Testing |
|---|---|
| "Design a URL shortener at scale" | Do you add CDN? LB? Cache? In the right order? |
| "System overloaded — what do you add?" | Do you reason from pain to solution? |
| "Where does the API Gateway sit?" | Do you understand the full request flow? |
| "What happens when a server crashes?" | Do you mention health checks and automatic LB removal? |
| "Why use L4 over L7 LB?" | Do you understand the performance tradeoff? |
| "Why not cache everything on CDN?" | Do you know CDN is bad for personalized data? |

> **The interview pattern:** They give you a broken system. You reason from the pain to the component. You place it correctly in the architecture. You explain the tradeoff. That is SDE-2 thinking.
