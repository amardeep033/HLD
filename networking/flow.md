# Complete HTTP Request → Response Flow

Every step in order, from browser cache check to origin server and back.

---

## Phase 1 — Browser Pre-checks (before any network call)

| # | Component | OSI Layer | Device / Process | Direction | What happens exactly | Hit → / Miss → |
|---|-----------|-----------|------------------|-----------|----------------------|----------------|
| 1 | **Browser cache** (HTTP response cache) | L7 Application | Browser memory / disk | local | Browser checks if it has a cached response for this exact URL. Evaluates `Cache-Control: max-age`, `Expires`, `ETag`, `Last-Modified`. If fresh → serve immediately, zero network. | **HIT** → done, return cached response / **MISS** → step 2 |
| 2 | **Service worker cache** (if PWA installed) | L7 Application | Browser JS sandbox | local | If site registered a service worker, it intercepts the fetch. Can serve from its own Cache Storage API, return offline fallback, or pass through to network. | **HIT** → done / **MISS** → step 3 |

---

## Phase 2 — DNS Resolution (name → IP address)

| # | Component | OSI Layer | Device / Process | Direction | What happens exactly | Hit → / Miss → |
|---|-----------|-----------|------------------|-----------|----------------------|----------------|
| 3 | **Browser DNS cache** | L7 Application | Browser process (RAM) | local | Browser maintains its own DNS cache separate from the OS. Checks if `api.example.com` was recently resolved. TTL respected. Inspect via `chrome://net-internals/#dns`. | **HIT** → skip to step 8 / **MISS** → step 4 |
| 4 | **OS DNS cache + hosts file** | L7 Application | OS (nscd / Windows DNS Client service) | local | Checks `hosts` file first (`C:\Windows\System32\drivers\etc\hosts` or `/etc/hosts`). If entry exists → that IP wins, no network DNS at all. Then checks OS-level DNS cache. | **HIT** → skip to step 8 / **MISS** → step 5 |
| 5 | **Router / local DNS resolver** | L7 Application | Home router or corp DNS server | LAN query | OS sends UDP query to configured DNS server (typically router at `192.168.1.1` or corp DNS). Router may cache. Corp DNS may apply split-horizon (internal domains resolve to private IPs). | **HIT** → skip to step 8 / **MISS** → step 6 |
| 6 | **Recursive resolver** (ISP or 8.8.8.8) | L7 Application | Remote DNS server | outbound query | Your upstream resolver (8.8.8.8, 1.1.1.1, ISP). Checks its own cache. If not cached → full recursive lookup: root nameserver → TLD nameserver (`.com`) → authoritative nameserver for `example.com`. | **HIT** (cached) → step 8 / **MISS** → step 7 |
| 7 | **Authoritative DNS** (domain's nameserver) | L7 Application | Cloudflare NS / Route 53 / etc. | outbound query | Source of truth for the domain. Returns the A record. **If Cloudflare CDN is on:** returns a Cloudflare edge IP (e.g. `104.21.x.x`), not your origin server IP. This DNS swap is the entire mechanism of CDN traffic steering. | Returns IP → step 8 |

---

## Phase 3 — TCP + TLS Connection Setup

| # | Component | OSI Layer | Device / Process | Direction | What happens exactly | Hit → / Miss → |
|---|-----------|-----------|------------------|-----------|----------------------|----------------|
| 8 | **Egress firewall check** | L3–L4 Network / Transport | OS iptables / Windows Firewall / corp appliance | outbound | Before any packet leaves, OS or router firewall checks outbound rules against destination IP + port 443. Corporate policy may block IP ranges or domain categories. Does **not** read HTTP content. | **ALLOWED** → step 9 / **BLOCKED** → connection dropped |
| 9 | **Forward proxy** (corp MITM — optional) | L7 Application | Squid / Zscaler / BlueCoat | outbound intercept | If configured via browser proxy settings or WPAD auto-discovery, all HTTPS is intercepted. Proxy performs its own TLS handshake using a corp-installed root cert (MITM), decrypts traffic, inspects URL + body, logs it, applies policy, re-encrypts onward. | **ALLOWED** → step 10 / **BLOCKED** → 407 / 403 from proxy |
| 10 | **VPN tunnel** (if VPN active) | L3 Network | Virtual NIC (`tun0` / WireGuard interface) | outbound wrap | All packets encapsulated in encrypted UDP/TCP tunnel to VPN server before leaving machine. Router and ISP see only encrypted traffic to VPN server IP — destination and content are hidden from them. VPN server decrypts and forwards. | Tunnelled → packets exit via VPN server IP |
| 11 | **ISP transit** | L1–L3 Physical / Network | ISP routers, fibre, submarine cables | both | Packets traverse ISP backbone via BGP routing. ISP sees destination IP and ports (not content over HTTPS). Some ISPs do deep packet inspection (DPI) for throttling or geo-blocking. BGP selects lowest-cost path to destination AS. | Routed → step 12 |
| 12 | **TCP 3-way handshake** | L4 Transport | OS TCP stack (both ends) | both | `SYN → SYN-ACK → ACK`. Establishes reliable connection to destination IP:443. **With CDN:** terminates at the nearest PoP (e.g. Mumbai), not origin in `us-east-1`. TCP handshake is ~5 ms to Mumbai vs ~200 ms to Virginia — this is the core latency win of CDN. | Connection established → step 13 |
| 13 | **TLS handshake** | L6 Presentation | OS TLS stack (both ends) | both | `ClientHello → ServerHello + Certificate → key exchange (ECDHE) → Finished`. Browser verifies cert chain up to trusted root CA. Negotiates cipher suite. Establishes symmetric session key. TLS 1.3 = 1 RTT; TLS 1.2 = 2 RTT. All subsequent data is encrypted. | Encrypted channel established → step 14 |

---

## Phase 4 — CDN Edge (if Cloudflare / CloudFront active)

| # | Component | OSI Layer | Device / Process | Direction | What happens exactly | Hit → / Miss → |
|---|-----------|-----------|------------------|-----------|----------------------|----------------|
| 14 | **CDN edge node (PoP)** — reverse proxy + cache | L7 Application | Cloudflare / CloudFront server at nearest PoP | inbound | Request lands at nearest PoP, not origin. CDN checks edge cache for this URL + query string. Evaluates `Cache-Control`, `Vary`, `Authorization` headers. `GET` with no auth → usually cacheable. `POST` → not cached. | **CACHE HIT** → return response, skip to step 20 / **CACHE MISS** → step 15 |
| 15 | **WAF rules check** (Web Application Firewall) | L7 Application | Cloudflare WAF / AWS WAF | inbound inspect | Inspects HTTP payload for: SQL injection in query params, XSS in body, path traversal, known bad user-agents, IP reputation (Tor exit nodes, scanners), rate limit violations per IP / API key, geo-blocking rules. Runs before request hits origin. | **CLEAN** → step 16 / **FLAGGED** → 403 or CAPTCHA challenge |

---

## Phase 5 — Origin Infrastructure

| # | Component | OSI Layer | Device / Process | Direction | What happens exactly | Hit → / Miss → |
|---|-----------|-----------|------------------|-----------|----------------------|----------------|
| 16 | **Load balancer** | L4 / L7 Transport / App | nginx / AWS ALB / HAProxy | inbound | Receives request from CDN (or directly from internet if no CDN). Terminates TLS if not done at CDN. Selects backend instance by algorithm (round-robin, least-connections, IP hash). Performs health checks. Adds `X-Forwarded-For` header with original client IP. | Forwarded to healthy instance → step 17 |
| 17 | **API gateway** (optional — microservices) | L7 Application | Kong / AWS API Gateway / Azure APIM / Traefik | inbound | Validates JWT / API key. Checks rate limit counter (e.g. Redis-backed: 100 req/min per key). Routes `GET /api/users` → users-service, `POST /api/orders` → orders-service. Can transform request headers, strip sensitive fields, log all API calls. Not just forwarding — understands your API contract. | **AUTHED + within limit** → step 18 / **FAILED** → 401 / 429 |
| 18 | **Ingress firewall** (server-side IP rules) | L3–L4 Network / Transport | AWS Security Group / Azure NSG / iptables on VM | inbound | Last IP-level gate. Allows only port 443 / 80 from load balancer IP range. Blocks all direct internet traffic to origin. If someone bypasses CDN and hits origin IP directly → packet dropped here. | **ALLOWED** → step 19 / **BLOCKED** → packet dropped silently |
| 19 | **Origin server** (your actual app) | L7 Application | ASP.NET Core / Node.js / Django in container or VM | processes | Receives fully decoded HTTP request. Runs middleware pipeline (auth, logging, validation). Executes business logic. Queries database (a separate network round-trip). Builds response. Sets `Cache-Control: public, max-age=60` so CDN knows what to cache and for how long. | Response built → step 20 |

---

## Phase 6 — Response Journey Back

| # | Component | OSI Layer | Device / Process | Direction | What happens exactly | Notes |
|---|-----------|-----------|------------------|-----------|----------------------|-------|
| 20 | **CDN caches + forwards response** | L7 Application | CDN edge node (same PoP) | outbound + store | CDN receives origin response. If `Cache-Control: public, max-age=60` → stores in edge cache, sets `CF-Cache-Status: MISS` on this first response. Next request from same region → `CF-Cache-Status: HIT`, origin is never called. Strips or adds headers per CDN config. | Cached + forwarded to client |
| 21 | **ISP return path** | L1–L3 Physical / Network | ISP backbone | inbound | Response packets routed back across internet to your public IP. BGP may choose a different physical path than outbound. TCP handles packet ordering and retransmission if any packets drop. | Delivered to your router |
| 22 | **TLS decrypt** | L6 Presentation | OS TLS stack | inbound | OS decrypts TLS records using session key from step 13. Verifies MAC (message authentication code). Passes plaintext HTTP response bytes up to browser process. | Plaintext response → step 23 |
| 23 | **Browser receives + renders response** | L7 Application | Browser process | inbound | Reads status code (200, 304, 301…). `304 Not Modified` → reuse existing cache entry, no body transferred. Stores response in browser cache per `Cache-Control` headers. Updates service worker cache if applicable. Parses HTML / JSON. Triggers sub-requests for every asset (CSS, JS, images, fonts) — each one independently restarts from step 1. | Rendered to user |

---

## Quick-reference: what each component is and is NOT

| Component | Layer | Reads HTTP body? | Direction | Caches? | Encrypts? |
|-----------|-------|------------------|-----------|---------|-----------|
| **Router** | L3 | No | Both | No | No (unless VPN router) |
| **Egress firewall** | L3–L4 | No | Outbound | No | No |
| **Forward proxy** | L7 | Yes (MITM) | Outbound | Yes | Re-encrypts |
| **VPN** | L3 | No | Both (tunnel) | No | Yes (tunnel) |
| **ISP** | L1–L3 | No (DPI possible) | Both | No | No |
| **DNS resolver** | L7 | No (DNS only) | Outbound query | Yes (TTL) | Optional (DoH) |
| **CDN / reverse proxy** | L7 | Yes | Inbound | Yes | Yes (TLS termination) |
| **WAF** | L7 | Yes | Inbound | No | No (reads decrypted) |
| **Load balancer** | L4/L7 | L7 mode: yes | Inbound | No | Optional (TLS termination) |
| **API gateway** | L7 | Yes | Inbound | Optional | No (reads decrypted) |
| **Ingress firewall** | L3–L4 | No | Inbound | No | No |
| **Origin server** | L7 | Yes | Inbound | No (app-level only) | No (handled by LB/CDN) |