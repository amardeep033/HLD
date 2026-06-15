# Networking Reference — SDE2 Interview

> **How to use this**: Don't memorize. Build the mental map.
> For every concept: *what is it → where does it live → who owns it → how does data move through it → what changes when it's enabled → where does it break.*

---

## Table of Contents

1. [The Big Picture — Full Request Flow](#1-the-big-picture--full-request-flow)
2. [OSI Model — Layers as Responsibilities](#2-osi-model--layers-as-responsibilities)
3. [Physical vs Logical — Hardware vs Software](#3-physical-vs-logical--hardware-vs-software)
4. [DNS — Domain to IP Resolution](#4-dns--domain-to-ip-resolution)
5. [DHCP — How Your Laptop Gets Config](#5-dhcp--how-your-laptop-gets-config)
6. [Gateway & Router](#6-gateway--router)
7. [Firewall](#7-firewall)
8. [Proxy — Forward and Reverse](#8-proxy--forward-and-reverse)
9. [VPN — Encrypted Overlay Network](#9-vpn--encrypted-overlay-network)
10. [CDN — Content Delivery Network](#10-cdn--content-delivery-network)
11. [Load Balancer](#11-load-balancer)
12. [API Gateway](#12-api-gateway)
13. [NAT — Network Address Translation](#13-nat--network-address-translation)
14. [TLS/HTTPS — Encryption in Transit](#14-tlshttps--encryption-in-transit)
15. [TCP vs UDP](#15-tcp-vs-udp)
16. [HTTP/1.1 vs HTTP/2 vs HTTP/3](#16-http11-vs-http2-vs-http3)
17. [ISP, BGP, and the Internet Backbone](#17-isp-bgp-and-the-internet-backbone)
18. [Component Location Map](#18-component-location-map)
19. [Comparison Tables — Quick Recall](#19-comparison-tables--quick-recall)
20. [Interview Mental Models — Layered Answers](#20-interview-mental-models--layered-answers)
21. [Failure Scenarios — Where Things Break](#21-failure-scenarios--where-things-break)
22. [Observability Commands](#22-observability-commands)

---

## 1. The Big Picture — Full Request Flow

When you open `https://chess.com` from your office laptop:

```
Browser
  ↓  (1) DNS lookup — domain → IP
OS DNS Resolver
  ↓
DNS Server (via UDP port 53)
  ↓  IP returned
Browser
  ↓  (2) TCP handshake to IP
Wi-Fi NIC
  ↓
Wi-Fi Access Point (wireless → wired)
  ↓
LAN Switch (Layer 2 forwarding)
  ↓
Gateway Router (routes to outside network)
  ↓
Firewall (packet filtering rules)
  ↓
Forward Proxy / Secure Web Gateway (content filtering, logging)
  ↓
Internet (BGP hops across ISP → IXP → destination network)
  ↓
CDN Edge Node (serves cached response if hit)
  ↓
Load Balancer (distributes to backend servers)
  ↓
Reverse Proxy / API Gateway (routes to correct service)
  ↓
Application Server
```

### With Corporate VPN active

VPN intercepts BEFORE traffic leaves your machine:

```
Browser
  ↓
OS routing table — VPN route takes over
  ↓
VPN client (FortiClient, Cisco AnyConnect) — encrypts packet
  ↓  outer packet destination = VPN Gateway IP
Wi-Fi → ISP carries encrypted outer packet
  ↓
Company VPN Gateway — decrypts, unwraps
  ↓
Company internal network
  ↓
App server / internal service
```

**Key insight**: ISP carries the *outer* encrypted packet. VPN carries the *inner* original traffic *logically*. Both participate. Neither replaces the other.

---

## 2. OSI Model — Layers as Responsibilities

> Think of it as: **who is responsible for what, at what level of abstraction.**

| Layer | # | Name | Job | Real Examples | Protocol/Unit |
|---|---|---|---|---|---|
| Application | 7 | Application | User-facing data exchange | HTTP, gRPC, WebSocket, DNS, SMTP | Message |
| Presentation | 6 | Presentation | Encoding, encryption, compression | TLS (partially), JSON serialization | Data |
| Session | 5 | Session | Session management, auth tokens | TLS session (partially), cookies | Data |
| Transport | 4 | Transport | End-to-end delivery, ports, reliability | TCP, UDP | Segment |
| Network | 3 | Network | IP routing, logical addressing | IP, ICMP, BGP | Packet |
| Data Link | 2 | Data Link | Frame delivery on local network, MAC | Ethernet, Wi-Fi (802.11), ARP | Frame |
| Physical | 1 | Physical | Raw bits over medium | Fiber, copper, radio waves | Bits |

### How to use OSI in interviews

**Firewall** = operates at Layer 3 (IP filtering) + Layer 4 (port filtering) + Layer 7 (DPI)

**Load Balancer**:
- L4 LB = routes by IP + TCP port (fast, no TLS termination)
- L7 LB = routes by HTTP headers, URL path, cookies (smarter, terminates TLS)

**Proxy** = Layer 7 (understands HTTP, can inspect/modify requests)

**Switch** = Layer 2 (MAC address table, local network)

**Router** = Layer 3 (IP routing table, across networks)

**VPN** = creates virtual Layer 3 network over existing Layer 3/4 infrastructure

### OSI in debugging

```
Can't reach server?

L1: Is cable/Wi-Fi connected?
L2: ARP resolving? MAC address known?
L3: IP route correct? ping works?
L4: Port open? TCP handshake completing?
L7: HTTP returning correct response?
```

---

## 3. Physical vs Logical — Hardware vs Software

> **Hardware moves packets physically. Software decides where they go.**

| Component | Hardware? | Software? | Lives Where | Famous Examples |
|---|---|---|---|---|
| Browser/App | ❌ | ✅ | Your laptop | Chrome, Firefox |
| OS Networking Stack | ❌ | ✅ | Your laptop | Windows TCP/IP, Linux netstack |
| Wi-Fi NIC | ✅ | ✅ (firmware/driver) | Inside laptop | Intel Wi-Fi 6, Realtek |
| Wi-Fi Access Point | ✅ | ✅ (firmware) | Office ceiling | Cisco AP, Ubiquiti UniFi |
| LAN Switch | ✅ | Sometimes managed | Office network room | Cisco Catalyst, Juniper |
| Gateway Router | ✅ | ✅ (routing OS) | Office edge | Cisco ISR, MikroTik |
| Firewall | Often appliance | ✅ (filtering logic) | Company network/DC | Palo Alto, Fortinet |
| VPN Client | ❌ | ✅ | Your laptop | FortiClient, AnyConnect |
| VPN Gateway | Sometimes appliance | ✅ | Company DC / cloud | FortiGate, OpenVPN server |
| Forward Proxy | Sometimes appliance | ✅ | Company / cloud | Zscaler, Squid |
| Reverse Proxy | Usually cloud VM | ✅ | Near backend | Nginx, Envoy, HAProxy |
| Load Balancer | Sometimes hardware | ✅ | Cloud / DC | AWS ALB, Nginx |
| CDN Edge | Physical PoP servers | ✅ | Globally distributed | Cloudflare, Akamai, AWS CloudFront |
| DNS Server | Physical servers | ✅ | Cloud / DC | Route53, Cloudflare DNS |
| Internet Backbone | Fiber, routers | BGP software | Global | ISPs, undersea cables |

### Modern reality

A single corporate appliance box (e.g. Fortinet FortiGate) often runs:
- Linux-based OS
- Routing software
- Firewall rules engine
- VPN server
- Proxy/inspection software

**Network appliances = specialized computers, not magic boxes.**

---

## 4. DNS — Domain to IP Resolution

### What it is

DNS answers one question: *"What IP address belongs to this domain name?"*

It is NOT magic browser logic. It is a network protocol like HTTP or SSH. Just specialized for name→IP mapping.

### How DNS protocol works

```
Traditional DNS:
  UDP port 53 (default — lightweight, fast)
  TCP port 53 (fallback — for large responses or zone transfers)

Modern secure DNS:
  DoH — DNS over HTTPS (port 443, encrypted)
  DoT — DNS over TLS (port 853, encrypted)
```

### DNS lookup flow (full)

```
Browser wants chess.com
  ↓
Browser cache? → HIT → done
  ↓ MISS
OS DNS cache? → HIT → done
  ↓ MISS
OS reads /etc/resolv.conf (Linux) or registry (Windows) for configured DNS server
  ↓
DNS query packet → UDP port 53 → configured DNS resolver
  ↓
Resolver checks its cache → HIT → returns IP
  ↓ MISS
Resolver performs recursive lookup:
  → Root DNS (13 root server clusters globally)
  → TLD DNS (.com, .in, .io)
  → Authoritative DNS for chess.com (owned by chess.com / their DNS provider)
  ↓
IP returned all the way back to browser
```

### DNS hierarchy — who maintains what

| Layer | Maintained By | Example |
|---|---|---|
| Your configured resolver | ISP / company / you manually | Airtel DNS, 8.8.8.8 |
| Root DNS | ICANN / global orgs | 13 root server clusters |
| TLD DNS | Registry operators | Verisign (.com), NIXI (.in) |
| Authoritative DNS | Domain owner | Google runs google.com DNS |

### How DNS server is assigned to your machine

**Via DHCP** (default, automatic):
- Join Wi-Fi → DHCP assigns IP, gateway, subnet mask, DNS server
- Home network: router IP (e.g. 192.168.1.1) is often the DNS forwarder
- Corporate: DHCP points to internal company DNS

**Via VPN** (override):
- VPN connects → pushes new DNS config to OS
- Now internal domains like `grafana.internal.company` resolve

**Manually** (override):
- Set 8.8.8.8 (Google) or 1.1.1.1 (Cloudflare) directly
- Bypasses ISP DNS — useful when ISP blocks via DNS

### Common public DNS servers

| Provider | IP |
|---|---|
| Google Public DNS | 8.8.8.8 / 8.8.4.4 |
| Cloudflare DNS | 1.1.1.1 / 1.0.0.1 |
| Quad9 | 9.9.9.9 |
| OpenDNS | 208.67.222.222 |

### DNS record types

| Record | Meaning | Example |
|---|---|---|
| A | Domain → IPv4 | chess.com → 1.2.3.4 |
| AAAA | Domain → IPv6 | chess.com → 2001:db8::1 |
| CNAME | Alias → another domain | www.chess.com → chess.com |
| MX | Mail server for domain | chess.com → mail.chess.com |
| TXT | Arbitrary text (SPF, DKIM, verification) | |
| NS | Authoritative nameservers for domain | |
| PTR | Reverse DNS (IP → domain) | |
| SRV | Service location (used by Kubernetes, SIP) | |

### Corporate DNS filtering

Company DNS intercepts queries and returns:
- `NXDOMAIN` (domain doesn't exist) — for blocked sites
- Fake IP pointing to block page
- Correct IP for allowed sites
- Private IPs for internal services (invisible to public DNS)

**Blocking happens BEFORE connection is even attempted.**

### DNS TTL

Each DNS record has a TTL (Time To Live in seconds). Resolver caches the answer for that duration. Lowering TTL before infrastructure change = faster propagation.

---

## 5. DHCP — How Your Laptop Gets Config

When you join a network, your machine has no IP yet. It broadcasts:

```
"Is there a DHCP server? I need network config."
```

DHCP server (usually your router) responds with:

| Config Item | Example |
|---|---|
| Your IP address | 192.168.1.25 |
| Subnet mask | 255.255.255.0 |
| Default gateway | 192.168.1.1 |
| DNS server | 192.168.1.1 or 8.8.8.8 |
| Lease duration | 24 hours |

DHCP uses UDP. Client port 68, server port 67.

---

## 6. Gateway & Router

### Gateway

The exit point of your local network. When your machine wants to reach an IP outside the local subnet, it sends the packet to the gateway.

**How machine knows what's local vs external:**
- Subnet mask defines local range (e.g. 192.168.1.0/24 = anything 192.168.1.x)
- Anything outside that range → send to gateway

### Router

Routes packets between networks using a **routing table**:

```
Destination     Gateway         Interface
0.0.0.0/0       192.168.1.1     eth0   ← default route (everything else)
192.168.1.0/24  0.0.0.0         eth0   ← local subnet, direct delivery
10.0.0.0/8      10.0.0.1        vpn0   ← VPN tunnel route
```

When VPN is active, VPN client injects new routes. Corporate traffic → vpn0 interface instead of default.

### In small/home networks

Single device acts as all of:
- Wi-Fi Access Point
- Gateway
- DHCP server
- DNS forwarder
- NAT device
- Basic firewall

### In enterprise networks

These are **separate physical devices** in separate racks:

```
Wi-Fi AP → Switch → Core Router → Firewall → Proxy → Internet
```

---

## 7. Firewall

### What it is

A packet filtering system. Decides: **allow or drop** packets based on rules.

**Not a magic box. Mostly software logic.**

### Types

**Stateless firewall (L3/L4)**
- Filters by: source IP, destination IP, port, protocol
- No memory of connections
- Fast but dumb

**Stateful firewall (L3/L4)**
- Tracks connection state (SYN, established, etc.)
- Allows return traffic for established connections automatically
- Most common in practice

**Application firewall / NGFW (L7)**
- Inspects actual payload
- Can detect: HTTP methods, domains, file types, malware signatures
- Deep Packet Inspection (DPI)
- Products: Palo Alto Networks, Fortinet FortiGate, Check Point

### Where firewalls live

| Location | Purpose |
|---|---|
| Your OS (Windows Defender Firewall, iptables) | Host-based, protects the machine |
| Network perimeter (company edge) | Protects company network from internet |
| Cloud security group (AWS SG, GCP Firewall) | Protects VMs/services in cloud |
| WAF (Web Application Firewall) | Protects HTTP apps (SQLi, XSS, DDoS) |

### How ISPs block websites

1. **DNS blocking** — ISP's DNS returns NXDOMAIN or fake IP
2. **IP blocking** — Firewall drops packets to specific destination IPs
3. **SNI filtering** — During TLS handshake, domain name is exposed in cleartext (SNI field); ISP inspects and drops
4. **DPI** — Deep inspection of packet patterns; detects VPNs, protocols, apps

**Changing DNS bypasses method 1 only.** VPN bypasses all of them (traffic appears as encrypted tunnel to VPN server IP).

---

## 8. Proxy — Forward and Reverse

### The key distinction

| | Forward Proxy | Reverse Proxy |
|---|---|---|
| Sits between | Client → Internet | Internet → Backend servers |
| Who configures it | Client / IT department | Server operator |
| Client aware? | Usually yes | Usually no |
| Purpose | Filtering, anonymity, caching | Load balancing, SSL termination, routing |
| Examples | Squid, Zscaler, corporate proxy | Nginx, Envoy, HAProxy, AWS ALB |

### Forward Proxy

```
You → Forward Proxy → Internet
```

- Corporate: IT forces all traffic through proxy for filtering/logging
- Your app must either be configured to use the proxy, or OS proxy settings are applied
- Proxy makes the request on your behalf; destination sees proxy's IP

**Where it lives:**
- Inside company network (on-prem)
- In cloud (Zscaler runs in global cloud edge DCs)
- On your machine (local debugging proxies like Fiddler, Charles)

### Reverse Proxy

```
Internet → Reverse Proxy → [Service A, Service B, Service C]
```

- Client thinks it's talking to one server; reverse proxy routes internally
- Terminates TLS — backend services get plain HTTP
- Can add: auth, rate limiting, caching, compression
- Nginx, Envoy, Traefik are common in production microservices

### Proxy vs VPN

| | Proxy | VPN |
|---|---|---|
| Layer | L7 (application) | L3 (network) |
| Traffic covered | Specific app only | All traffic from machine |
| Encryption | Optional | Yes (always) |
| Routing change | No OS routing change | Modifies OS routing table |
| Common use | Corporate filtering, scraping | Remote access, privacy |

---

## 9. VPN — Encrypted Overlay Network

### What it actually does

1. Creates a **virtual network interface** on your OS (e.g. `tun0`, `ppp0`)
2. **Injects routes** into OS routing table so specific traffic goes to that interface
3. **Encrypts packets** and sends them to VPN gateway as outer UDP/TCP packets
4. VPN gateway **decrypts, unwraps**, routes to internal network

### The encapsulation mental model

```
Your original packet:
  [IP header: dst=internal.company] [TCP] [HTTP payload]

After VPN wraps it:
  [IP header: dst=VPN Gateway] [UDP/TCP] [ENCRYPTED: original packet]

What ISP sees:
  Encrypted traffic to VPN Gateway IP. Nothing else.
```

### Split Tunnel vs Full Tunnel

**Split Tunnel:**
```
internal.company.com → VPN interface → company network
chess.com           → default interface → ISP → internet (direct)
```
Better performance. Personal traffic not routed through company.

**Full Tunnel:**
```
ALL traffic → VPN interface → company network → company forwards to internet
```
Company sees everything. More secure from company's perspective. Slower.

### VPN protocols

| Protocol | Port | Notes |
|---|---|---|
| IPSec/IKEv2 | UDP 500, 4500 | Common enterprise, fast |
| OpenVPN | UDP 1194 (configurable) | Open source, flexible |
| WireGuard | UDP 51820 | Modern, fast, minimal codebase |
| SSL/TLS VPN | TCP 443 | Looks like HTTPS traffic; hard to block |

### VPN + ISP relationship

VPN does NOT replace ISP. VPN overlays on top of ISP:

```
ISP provides:   physical internet connectivity
VPN provides:   encrypted logical private routing over that connectivity

Without ISP → VPN cannot exist.
```

Airtel carries the encrypted outer packet. It cannot see inner contents.

### DNS and VPN

When VPN connects, it pushes new DNS config to OS:
- Before VPN: DNS = 8.8.8.8
- After VPN: DNS = 10.0.0.1 (company internal DNS)

This enables resolving internal domains like `kafka.prod.internal`.

---

## 10. CDN — Content Delivery Network

### What it is

A globally distributed network of **edge servers** that cache content close to users.

```
User in Bangalore → chess.com → CDN edge node in Mumbai → serves cached response
                                 (instead of: → chess.com origin in US)
```

### What gets cached

- Static assets: JS, CSS, images, fonts
- API responses (with correct Cache-Control headers)
- Entire pages (for static sites)

### How it works technically

1. Domain DNS points to CDN (CNAME to CDN hostname)
2. CDN DNS resolves to nearest edge PoP based on user's IP/location
3. Edge node serves cache hit or forwards (origin pull) to origin server
4. Response cached at edge with TTL

### CDN as security layer

Modern CDNs (Cloudflare, Akamai) also provide:
- DDoS protection (absorb volumetric attacks at edge)
- WAF (block SQLi, XSS before reaching origin)
- Bot mitigation
- TLS termination at edge

### CDN vs Reverse Proxy

CDN = distributed global reverse proxy network with caching focus.

| | CDN | Reverse Proxy |
|---|---|---|
| Location | Global edge (100s of PoPs) | Single cluster / region |
| Primary purpose | Caching + proximity | Routing + LB + SSL termination |
| Examples | Cloudflare, Akamai, CloudFront | Nginx, Envoy, HAProxy |

---

## 11. Load Balancer

### What it is

Distributes incoming traffic across multiple backend servers.

```
Client → Load Balancer → [Server A, Server B, Server C]
```

### L4 vs L7 Load Balancer

**L4 (Transport layer):**
- Routes based on IP + TCP/UDP port
- Doesn't inspect HTTP content
- Faster, lower overhead
- Cannot route based on URL path or headers
- Example: AWS NLB, hardware LBs

**L7 (Application layer):**
- Routes based on: URL path, Host header, cookies, request body
- Terminates TLS
- Can do: sticky sessions, A/B routing, canary deploys
- Example: AWS ALB, Nginx, Envoy

### Load balancing algorithms

| Algorithm | How it works | Good for |
|---|---|---|
| Round Robin | Rotate through servers in order | Uniform requests |
| Weighted Round Robin | More weight = more traffic | Servers with different capacity |
| Least Connections | Route to server with fewest active conns | Long-lived connections |
| IP Hash | Hash client IP → always same server | Session stickiness |
| Random | Random selection | Simple, surprisingly effective |

### Health checks

LB periodically pings each backend. If health check fails → remove from pool. When recovered → re-add.

---

## 12. API Gateway

### What it is

Sits in front of your microservices. Single entry point for all API traffic.

```
Client → API Gateway → [Auth Service, Product Service, Order Service, ...]
```

### What it does

- **Authentication / Authorization** (validate JWT, OAuth tokens)
- **Rate limiting** (100 req/sec per client)
- **Request routing** (route /orders → Order Service)
- **Request transformation** (add headers, transform body)
- **SSL termination**
- **Logging / Metrics** (centralized observability)
- **Circuit breaking**

### API Gateway vs Load Balancer vs Reverse Proxy

| | API Gateway | Load Balancer | Reverse Proxy |
|---|---|---|---|
| Routing | By URL path + method + headers | By IP/port or URL | By URL/host |
| Auth | ✅ Built-in | ❌ | Sometimes |
| Rate limiting | ✅ Built-in | ❌ | Sometimes |
| Multiple backends | ✅ Routes to different services | Single backend pool | Usually one upstream |
| Examples | AWS API Gateway, Kong, Apigee | AWS ALB/NLB | Nginx, Envoy |

---

## 13. NAT — Network Address Translation

### The problem it solves

IPv4 has ~4 billion addresses. There are more than 4 billion devices. NAT allows an entire network to share a single public IP.

### How it works

Your home network: 192.168.1.0/24 (private, not routable on internet)
Your public IP: 1.2.3.4 (one IP for everything)

```
Your laptop: 192.168.1.10:54321 → wants to connect to 93.184.216.34:443
Router NAT table: maps 192.168.1.10:54321 → 1.2.3.4:7001
Internet sees request from: 1.2.3.4:7001
Response comes back to: 1.2.3.4:7001
Router translates back to: 192.168.1.10:54321
```

### Private IP ranges (never routed on public internet)

| Range | Common use |
|---|---|
| 10.0.0.0/8 | Corporate networks, cloud VPCs |
| 172.16.0.0/12 | Corporate, Docker default |
| 192.168.0.0/16 | Home networks |

---

## 14. TLS/HTTPS — Encryption in Transit

### TLS handshake (simplified)

```
Client → Server: "Hello. I support these cipher suites. Here's my random nonce."
Server → Client: "Hello. Here's my chosen cipher. Here's my certificate. Here's my random nonce."
Client:           Validates certificate (chain to trusted CA)
Client → Server: "Here's pre-master secret encrypted with your public key."
Both:             Derive session keys from pre-master + nonces
Client → Server: "Finished (encrypted)"
Server → Client: "Finished (encrypted)"
--- encrypted data exchange begins ---
```

### What TLS protects

- **Confidentiality**: payload encrypted (ISP can't read it)
- **Integrity**: tampering detected (MAC)
- **Authentication**: server proves identity via certificate

### What TLS does NOT hide

- **Destination IP** — visible in IP header (always)
- **SNI (Server Name Indication)** — domain name sent in cleartext during TLS handshake so servers can serve correct cert for multi-hosted domains
  - Unless **ECH (Encrypted Client Hello)** is used — TLS 1.3 extension

### Certificate chain

```
Your cert (chess.com)
  ↑ signed by
Intermediate CA (Let's Encrypt R3)
  ↑ signed by
Root CA (ISRG Root X1)
  ← browser trusts root CAs (baked into OS/browser)
```

### TLS termination

Reverse proxy / LB / CDN edge decrypts TLS. Backend gets plain HTTP. Avoids distributing certs to every backend service.

---

## 15. TCP vs UDP

| | TCP | UDP |
|---|---|---|
| Connection | Connection-oriented (handshake) | Connectionless |
| Reliability | Guaranteed delivery + ordering | No guarantee |
| Speed | Slower (overhead) | Faster |
| Error handling | Retransmit lost packets | Application handles or ignores |
| Use cases | HTTP, HTTPS, SSH, databases | DNS, video streaming, gaming, VoIP |
| Header size | 20 bytes | 8 bytes |

### TCP three-way handshake

```
Client → Server: SYN
Server → Client: SYN-ACK
Client → Server: ACK
--- connection established ---
```

### TCP connection teardown

```
Client → Server: FIN
Server → Client: ACK
Server → Client: FIN
Client → Server: ACK
```

---

## 16. HTTP/1.1 vs HTTP/2 vs HTTP/3

| Feature | HTTP/1.1 | HTTP/2 | HTTP/3 |
|---|---|---|---|
| Transport | TCP | TCP | QUIC (UDP-based) |
| Multiplexing | ❌ (one request per connection, pipelining unreliable) | ✅ (multiple streams over one connection) | ✅ |
| Head-of-line blocking | ✅ (yes, problem) | Partially solved | ✅ Solved (QUIC stream isolation) |
| Header compression | ❌ | ✅ HPACK | ✅ QPACK |
| Server push | ❌ | ✅ | ✅ |
| Encryption | Optional | Required in practice (HTTPS) | Built-in (TLS 1.3) |
| Connection setup | TCP + TLS (2 round trips) | TCP + TLS (2 round trips) | QUIC (0-1 round trip) |

### WebSocket

Upgrade from HTTP/1.1 to persistent full-duplex TCP connection:
```
GET /chat HTTP/1.1
Upgrade: websocket
Connection: Upgrade
```
Used for: real-time chat, live dashboards, collaborative tools.

### Long Polling vs SSE vs WebSocket

| | Long Polling | SSE | WebSocket |
|---|---|---|---|
| Direction | Server → Client (via repeated HTTP) | Server → Client (one-way stream) | Full duplex |
| Protocol | HTTP | HTTP | WS upgrade |
| Use case | Simple push, older browsers | Notifications, live feeds | Chat, gaming |

---

## 17. ISP, BGP, and the Internet Backbone

### Internet = network of networks

Every major company/ISP has its own **Autonomous System (AS)** — a network under one administrative control with a unique AS number.

Examples:
- Airtel AS9498
- Google AS15169
- Cloudflare AS13335
- Amazon AS16509

### BGP (Border Gateway Protocol)

The protocol that stitches the internet together. ASes announce:
- "I can reach these IP ranges"
- "This is my path to reach them"

Routers use BGP to build routing tables. Every router makes a local forwarding decision: "which interface do I send this packet out?"

### Packet journey: Bangalore → Google

```
Your laptop
  → Home router
  → Airtel local access network (DSLAM/OLT)
  → Airtel backbone
  → Mumbai Internet Exchange (NIXI)  ← where ISPs peer
  → Google's network (peering agreement)
  → Google datacenter
  → Server
```

Airtel does NOT carry packets to every server in the world. It carries them until another network takes over.

### Peering vs Transit

**Peering**: Two networks exchange traffic directly, free of charge (or paid settlement). Common at IXPs (Internet Exchange Points). Example: Airtel ↔ Google peering in Mumbai.

**Transit**: Smaller network pays larger network to carry traffic to the rest of internet. Example: Small ISP buys transit from Tata Communications.

---

## 18. Component Location Map

```
┌─────────────────────────────────────────────────────────────────────┐
│ YOUR MACHINE                                                        │
│  Browser → OS networking stack → Wi-Fi NIC → VPN client            │
└────────────────────────┬────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────────────┐
│ LOCAL NETWORK (office / home)                                       │
│  Wi-Fi AP → Switch → Gateway Router → (Firewall) → (Forward Proxy) │
└────────────────────────┬────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────────────┐
│ ISP NETWORK                                                         │
│  Access nodes → Backbone routers → DNS clusters → IXP peering      │
└────────────────────────┬────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────────────┐
│ INTERNET / CLOUD EDGE                                               │
│  CDN edge PoPs → DDoS scrubbing → WAF                              │
└────────────────────────┬────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────────────┐
│ DESTINATION (company cloud / data center)                           │
│  Load Balancer → API Gateway → Reverse Proxy → App Servers         │
│  + internal: Service Mesh, DB, Cache, Message Queue                │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 19. Comparison Tables — Quick Recall

### Traffic interceptors — who sees what

| Component | Sees Source IP? | Sees Destination IP? | Sees HTTP Payload? | Sees Domain? |
|---|---|---|---|---|
| ISP (no VPN) | ✅ (your IP) | ✅ | ❌ (HTTPS) | ✅ (SNI, DNS) |
| ISP (with VPN) | ✅ (your IP) | VPN server IP only | ❌ | ❌ |
| Forward Proxy | ✅ | ✅ | ✅ (HTTP) / ❌ (HTTPS tunnel) | ✅ |
| Corporate Proxy (TLS intercept) | ✅ | ✅ | ✅ (MITM cert) | ✅ |
| CDN Edge | ✅ | Itself | ✅ | ✅ |
| Load Balancer | ✅ (or X-Forwarded-For) | ✅ | L7 LB only | L7 LB only |
| VPN Gateway | ✅ (your IP) | ✅ (after decrypt) | ✅ (after decrypt) | ✅ (after decrypt) |

### Blocking mechanisms

| Method | What it blocks | Bypass |
|---|---|---|
| DNS blocking | Domain → NXDOMAIN / fake IP | Change DNS to 8.8.8.8 / 1.1.1.1 |
| IP blocking | All traffic to specific IPs | VPN (traffic goes to VPN IP instead) |
| SNI filtering | Domain name in TLS handshake | ECH, VPN |
| DPI | Protocols, VPN patterns, apps | VPN obfuscation, Tor |

### Caching layers in a request

```
Browser cache → OS DNS cache → CDN edge cache → Reverse proxy cache → App cache → DB
```

---

## 20. Interview Mental Models — Layered Answers

### When asked "How does a request reach a server?"

**Layer 1 answer (30 seconds):**
Browser resolves domain via DNS, opens TCP+TLS connection to server IP, sends HTTP request, receives response.

**Layer 2 answer (2 minutes):**
Add: DNS hierarchy, TCP handshake, TLS negotiation, CDN edge serving cached response, load balancer distributing to backend.

**Layer 3 answer (go deep if pushed):**
Add: DHCP-assigned DNS, recursive resolver, BGP routing across ASes, NAT at home router, VPN encapsulation, SSL termination at LB, reverse proxy routing to microservice.

### When asked "What is a VPN?"

**Weak**: "It encrypts your traffic."

**Strong**: "VPN creates a virtual network interface on your OS and modifies routing so traffic matching configured routes is encapsulated and encrypted, then sent to a VPN gateway. The ISP sees only an encrypted outer packet destined for the VPN gateway IP. At the gateway, packets are decrypted and routed to the internal network. This gives remote machines the appearance of being inside the private network, including DNS resolution for internal hostnames."

### When asked "What happens when you type a URL?"

Full answer structure:
1. Browser parses URL (scheme, host, path, query)
2. DNS resolution (cache → OS resolver → recursive lookup)
3. TCP connection to resolved IP + port
4. TLS handshake (if HTTPS)
5. HTTP request sent
6. Request travels: NIC → switch → router → firewall → proxy → ISP → internet → CDN → LB → app server
7. Server processes request, returns HTTP response
8. Browser parses HTML, sub-resources (CSS/JS/images) trigger more requests
9. Rendering pipeline (DOM + CSSOM → Render tree → Layout → Paint)

---

## 21. Failure Scenarios — Where Things Break

### DNS failures

| Symptom | Likely cause |
|---|---|
| `NXDOMAIN` | Domain doesn't exist OR DNS blocking |
| Slow resolution | Resolver too far away, TTL expired, upstream slow |
| Inconsistent results | DNS propagation delay after record change (old TTL still cached) |
| Internal domains don't resolve | VPN not connected, wrong DNS server configured |

### Network failures

| Symptom | Likely cause |
|---|---|
| ping works, HTTPS fails | Firewall blocking port 443, TLS cert issue |
| ping fails, traceroute stops at hop N | Router/firewall blocking ICMP at that hop |
| Connection timeout (no response) | Packet dropped by firewall, wrong IP, server down |
| Connection refused | Server is up but port not listening |
| Intermittent failures | Load balancer removing unhealthy instance mid-flight |

### VPN failures

| Symptom | Likely cause |
|---|---|
| Connected but internal domains don't resolve | DNS not pushed by VPN, or using public DNS over VPN |
| Connected but slow | Full tunnel VPN routing all traffic through company |
| VPN fails on certain networks | ISP blocking VPN protocol's port |

### TLS failures

| Symptom | Likely cause |
|---|---|
| `SSL certificate error` | Expired cert, wrong domain, self-signed |
| Corporate MITM warning | Company proxy doing TLS inspection with their cert |
| `ERR_CERT_AUTHORITY_INVALID` | Cert chain not trusted by OS/browser |

---

## 22. Observability Commands

### DNS

```bash
# Basic lookup
nslookup chess.com

# Detailed DNS trace with record type
dig chess.com A
dig chess.com MX
dig +trace chess.com        # full recursive trace from root
dig @8.8.8.8 chess.com      # query specific DNS server

# Check which DNS server your OS uses
cat /etc/resolv.conf         # Linux
ipconfig /all                # Windows (look for "DNS Servers")
```

### Network routing

```bash
# Show routing table
ip route show                # Linux
route print                  # Windows
netstat -rn                  # Linux/Mac

# Trace packet path
traceroute chess.com         # Linux/Mac
tracert chess.com            # Windows

# Test connectivity
ping chess.com
curl -v https://chess.com    # verbose HTTP including TLS

# Check open ports
ss -tulnp                    # Linux: listening sockets
netstat -an                  # All connections
```

### TLS

```bash
# Inspect TLS cert of server
openssl s_client -connect chess.com:443 -servername chess.com

# Check cert expiry
echo | openssl s_client -connect chess.com:443 2>/dev/null | openssl x509 -noout -dates
```

### VPN / routing inspection

```bash
# See all interfaces including VPN virtual interfaces
ip link show
ifconfig -a

# See which interface VPN traffic goes through
ip route get 10.0.0.1        # replace with internal IP
```

### Packet capture

```bash
# Capture DNS packets
tcpdump -i eth0 port 53

# Capture all traffic to/from IP
tcpdump host 1.2.3.4

# Capture HTTP traffic
tcpdump -A -s 0 'tcp port 80'
```

---

## Quick Reference — Mental Model Summary

| Concept | One-line mental model |
|---|---|
| DNS | Phone book: name → number (domain → IP) |
| DHCP | Network orientation: here's your IP, gateway, DNS |
| Gateway | City exit: your packet leaves local network through here |
| Router | Traffic director: reads destination IP, decides next hop |
| Firewall | Security checkpoint: allow or drop based on rules |
| Forward Proxy | Courier: fetches things on your behalf |
| Reverse Proxy | Receptionist: routes external requests to correct internal service |
| VPN | Sealed armored container: ISP carries it, can't see inside |
| CDN | Local warehouse: content cached near user |
| Load Balancer | Queue manager: distributes work across servers |
| API Gateway | Border control: auth, rate limit, route to correct service |
| NAT | One postal address for whole building: translates internal IPs |
| TLS | Encrypted envelope: only recipient can read, integrity guaranteed |
| BGP | Internet highway system: networks advertise reachable IP ranges |

---

*Built from systems-first understanding. For each concept: flow → ownership → boundaries → failure cases → production behavior.*