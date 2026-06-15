# OSI Model — Complete Reference

All 7 layers with protocols, devices, message/frame contents, and real-world network components.

---

## Layer 7 — Application

| Field | Detail |
|-------|--------|
| **PDU (data unit)** | Message / Data |
| **Protocols** | `HTTP/1.1` `HTTP/2` `HTTP/3` `HTTPS` `DNS` `SMTP` `FTP` `WebSocket` `gRPC` |
| **Devices / software** | Browser, curl, Postman · **Forward proxy** (Squid, Zscaler) · **Reverse proxy / CDN** (Cloudflare, nginx) · **API gateway** (Kong, AWS Gateway) · **WAF** (Cloudflare WAF) |

### Message contents

**HTTP Request:**
```
GET /api/users HTTP/1.1
Host: api.example.com
Authorization: Bearer eyJ…
Content-Type: application/json
```

**HTTP Response:**
```
HTTP/1.1 200 OK
Cache-Control: public, max-age=60
CF-Cache-Status: HIT
Content-Type: application/json

{"users":[…]}
```

### What happens here

- **Browser** constructs HTTP request, verifies TLS certificate, parses HTML/JSON, triggers sub-requests for every asset (CSS, JS, images) — each restarts the whole flow.
- **DNS** also lives at L7 — resolves domain to IP via UDP port 53. Resolution order: browser DNS cache → OS DNS cache → `hosts` file → router DNS → recursive resolver (8.8.8.8) → authoritative nameserver. With CDN active, authoritative NS returns a Cloudflare edge IP (`104.21.x.x`) instead of origin IP — this DNS swap is the entire mechanism of CDN traffic steering.
- **Forward proxy** intercepts outbound HTTPS. Uses a corp-installed root cert for MITM TLS termination, decrypts and inspects URL + body, applies policy, logs, re-encrypts onward. Configured via browser proxy settings or WPAD auto-discovery.
- **CDN edge node** terminates your connection at the nearest PoP. Checks edge cache for URL. CACHE HIT → serves response, origin never called. CACHE MISS → fetches from origin, stores it, returns it. Cacheability determined by `Cache-Control` response header.
- **WAF** inspects HTTP payload for SQLi patterns, XSS, bad user-agents, rate abuse. Blocks before request reaches origin.
- **API gateway** validates JWT/API keys, enforces rate limits per key, routes `/orders` → orders-service, `/users` → users-service, transforms request headers.

---

## Layer 6 — Presentation

| Field | Detail |
|-------|--------|
| **PDU** | Message (formatted / encrypted) |
| **Protocols** | `TLS 1.3` `TLS 1.2` `SSL` (deprecated) · `JPEG` `PNG` `gzip` `Base64` `JSON encoding` |
| **Devices / software** | OS TLS stack (SChannel / OpenSSL / BoringSSL) · Browser TLS engine · Certificate authorities (Let's Encrypt, DigiCert) |

### Message contents

**TLS ClientHello:**
```
Supported TLS versions
Cipher suites offered
Random nonce (32 bytes)
SNI: api.example.com
Client key share (ECDHE public key)
```

**TLS ServerHello:**
```
Chosen cipher suite
Server certificate (X.509)
Server random nonce
Server key share (ECDHE public key)
```

**TLS Finished:**
```
Encrypted with derived session key
MAC verification data
```

### What happens here

**TLS 1.3 handshake (1 RTT):**
1. Client → Server: `ClientHello` (supported ciphers + ECDHE key share)
2. Server → Client: `ServerHello` + certificate + `Finished` (all in one flight)
3. Client verifies cert chain up to trusted root CA. Both sides derive symmetric session key from ECDHE exchange.
4. Client → Server: `Finished` (encrypted). Symmetric encryption begins.

**With CDN:** TLS terminates at the PoP — your session key is shared with Cloudflare's edge server. A second, separate TLS connection goes from the PoP to your origin. The CDN edge sees your plaintext HTTP between the two TLS segments.

**Certificate pinning:** mobile apps can pin the cert's public key fingerprint. A corporate MITM proxy will break pinning — the app will refuse the connection.

---

## Layer 5 — Session

| Field | Detail |
|-------|--------|
| **PDU** | Message (session data) |
| **Protocols** | `TLS session resumption` · `HTTP/2 streams` · `WebSocket` · `RPC sessions` · `NetBIOS` |
| **Devices / software** | Browser session manager · Server session store (Redis, DB) · Load balancer sticky sessions · WebSocket upgrade handler |

### Message contents

**TLS session ticket (resumption):**
```
Encrypted session state (server-issued opaque blob)
Used to resume without full handshake
```

**HTTP/2 HEADERS frame:**
```
Stream ID: 1
:method: GET
:path: /api/users
:authority: api.example.com
:scheme: https
```

**WebSocket upgrade:**
```
GET /ws HTTP/1.1
Upgrade: websocket
Connection: Upgrade
Sec-WebSocket-Key: dGhlIHNhbXBsZQ==
Sec-WebSocket-Version: 13
```

### What happens here

- **TLS session resumption:** browser reuses a session ticket (TLS 1.2) or pre-shared key (TLS 1.3) on repeat connections — skips the full handshake for 0-RTT reconnection.
- **HTTP/2 multiplexing:** multiple requests share one TCP connection as separate numbered streams. Browser sends 50 asset requests in parallel — no head-of-line blocking per request (TCP still blocks per packet though; fixed in HTTP/3).
- **Load balancer sticky sessions:** inserts a cookie or uses IP hash to route the same user to the same backend instance — required for stateful application servers.

> Note: TCP/IP does not have a distinct session layer in practice. L5 functionality is absorbed into TLS (L6) and application protocols (L7).

---

## Layer 4 — Transport

| Field | Detail |
|-------|--------|
| **PDU** | Segment (TCP) / Datagram (UDP) |
| **Protocols** | `TCP` `UDP` `QUIC` (HTTP/3) `SCTP` |
| **Devices / software** | OS TCP/UDP stack · **Stateful firewall (L4)** · **Load balancer L4** (AWS NLB, HAProxy TCP mode) · **VPN** (WireGuard uses UDP 51820) |

### Message contents

**TCP segment header:**
```
Source port:      54321
Destination port: 443
Sequence number:  1001
ACK number:       2001
Flags:            SYN | ACK | FIN | RST | PSH
Window size:      65535
Checksum:         0x1a2b
Payload:          [TLS record bytes]
```

**TCP 3-way handshake messages:**
```
Client → Server:  SYN  (seq=x)
Server → Client:  SYN-ACK  (seq=y, ack=x+1)
Client → Server:  ACK  (ack=y+1)
```

**TCP teardown:**
```
FIN → FIN-ACK → FIN → ACK  (4-way)
```

### What happens here

- **TCP 3-way handshake:** SYN → SYN-ACK → ACK. With CDN, this terminates at the PoP, not origin. Mumbai PoP ≈ 5 ms RTT vs. origin in `us-east-1` ≈ 200 ms RTT. Moving TCP termination closer to the user is the primary latency benefit of CDN.
- **Stateful firewall at L4:** tracks connection state table (SYN_SENT → ESTABLISHED → TIME_WAIT). Allows return traffic for established connections automatically. AWS Security Groups work this way — you only need an inbound allow rule; replies are tracked statefully.
- **L4 load balancer:** routes TCP connections by IP/port without reading HTTP content. Faster than L7 LB but cannot route by URL path or headers.
- **VPN at L4:** WireGuard encapsulates your TCP segments inside UDP datagrams addressed to the VPN server. ISP sees only UDP traffic to the VPN server's IP — destination and content are hidden.
- **QUIC (HTTP/3):** runs over UDP. Reimplements reliability + multiplexing in userspace. Eliminates TCP head-of-line blocking entirely. TLS 1.3 is mandatory and integrated — achieves 0-RTT connection establishment for known servers.

---

## Layer 3 — Network

| Field | Detail |
|-------|--------|
| **PDU** | Packet (IP datagram) |
| **Protocols** | `IPv4` `IPv6` · `ICMP` (ping, traceroute) · `BGP` (inter-AS routing) · `OSPF` (intra-AS routing) · `IPSec` (VPN at L3) · `ARP` (IP→MAC mapping) |
| **Devices / software** | **Router** (home router, ISP core router, BGP border router) · **L3 stateless firewall** · **VPN client/server** (tun0 virtual NIC) · ISP backbone · CDN anycast |

### Message contents

**IPv4 packet header:**
```
Version:         4
IHL:             20 bytes
DSCP/ECN:        (QoS markings)
Total length:    1500 bytes (MTU)
TTL:             64  ← decremented at each hop; 0 = drop + ICMP Time Exceeded
Protocol:        6 (TCP) | 17 (UDP) | 1 (ICMP)
Header checksum: 0x4b2f
Source IP:       203.0.113.5        ← your public IP (or VPN server IP)
Destination IP:  104.21.44.8        ← CDN PoP IP (or origin IP if no CDN)
Payload:         [TCP segment]
```

**ICMP echo (ping):**
```
Type: 8 (echo request) | 0 (echo reply)
Code: 0
Identifier: 1234
Sequence:   1
Data:       [timestamp + padding]
```

**BGP UPDATE message:**
```
WITHDRAWN routes: (none)
Path attributes:
  ORIGIN: IGP
  AS_PATH: 13335 (Cloudflare AS)
  NEXT_HOP: 192.0.2.1
NLRI: 104.16.0.0/12
```

### What happens here

- **Router:** reads destination IP, looks up forwarding table (longest prefix match), forwards to next-hop interface. Decrements TTL. Your home router does NAT — maps private `192.168.x.x:port` pairs to your one public IP using a port translation table (conntrack).
- **VPN at L3:** WireGuard creates virtual NIC `tun0`. OS routing table sends all packets through it. Original IP packet becomes payload of an outer UDP packet addressed to VPN server. At VPN server: outer UDP stripped, inner packet forwarded from VPN server's IP. Destination sees VPN server IP, not yours.
- **CDN anycast at L3:** Cloudflare announces the same IP prefix (e.g. `104.16.0.0/12`) from all 300+ PoPs via BGP. BGP routing naturally delivers your packets to the geographically nearest PoP. No DNS trick needed for routing — anycast handles it purely at L3.
- **ISP transit:** BGP routes packets between autonomous systems (AS). Your ISP peers with Cloudflare's AS directly or via an internet exchange point (IXP). ISP can see destination IP and protocol — not content over HTTPS.
- **traceroute** exploits TTL: sends packets with TTL=1, 2, 3… Each router that drops a packet sends back ICMP Time Exceeded with its own IP — reveals the hop-by-hop path.

---

## Layer 2 — Data Link

| Field | Detail |
|-------|--------|
| **PDU** | Frame (Ethernet / Wi-Fi frame) |
| **Protocols** | `Ethernet II` (IEEE 802.3) · `Wi-Fi` (IEEE 802.11) · `PPP` · `ARP` (IP→MAC) · `STP` (spanning tree) · `VLAN 802.1Q` |
| **Devices / software** | **L2 switch** (forwards by MAC address) · Wi-Fi access point · Network interface card (NIC) · Bridge |

> Your home "router" is actually L3 router + L2 switch + L2 Wi-Fi AP in one box.

### Message contents

**Ethernet II frame:**
```
Preamble:        7 bytes  (10101010 10101010 … — clock sync)
Start delimiter: 1 byte   (10101011)
Destination MAC: ff:ff:ff:ff:ff:ff  (broadcast ARP) or 00:1A:2B:3C:4D:5E (unicast)
Source MAC:      your NIC's MAC address
EtherType:       0x0800 (IPv4) | 0x86DD (IPv6) | 0x0806 (ARP)
Payload:         [IP packet — up to 1500 bytes MTU]
FCS:             4-byte CRC checksum
```

**ARP request (broadcast):**
```
"Who has 192.168.1.1? Tell 192.168.1.50"
Sender MAC: aa:bb:cc:dd:ee:ff
Sender IP:  192.168.1.50
Target MAC: 00:00:00:00:00:00  (unknown)
Target IP:  192.168.1.1
```

**ARP reply (unicast):**
```
"192.168.1.1 is at 11:22:33:44:55:66"
```

**802.1Q VLAN tag (inserted after source MAC):**
```
TPID:    0x8100
PCP:     3 bits (priority)
DEI:     1 bit
VLAN ID: 12 bits (0–4095)
```

### What happens here

- **L2 switch:** learns source MAC from every incoming frame, builds a MAC address table keyed by port. Forwards unicast frames only to the correct port. Floods unknown unicast and broadcasts to all ports. Does not read IP addresses — purely MAC-based.
- **ARP:** before your OS can send a frame, it needs the MAC of the next hop (your gateway). ARP broadcasts the question, gateway replies with its MAC. OS caches in ARP table (`arp -a` / `ip neigh`).
- **Wi-Fi (802.11):** adds wireless-specific headers (BSSID, frame control, sequence number, retry bit). Access point bridges Wi-Fi ↔ Ethernet frames transparently.
- **VLAN (802.1Q):** 4-byte tag inserted into the Ethernet frame. Corp switches isolate HR, Dev, and Guest traffic on the same physical cables by VLAN ID. Frames only reach ports in the same VLAN.
- **STP (Spanning Tree Protocol):** prevents L2 loops in switched networks by blocking redundant ports. A loop without STP would broadcast-storm the entire network.

---

## Layer 1 — Physical

| Field | Detail |
|-------|--------|
| **PDU** | Bit (raw signal / symbol) |
| **Protocols / standards** | `Ethernet` (1000BASE-T, 10GBASE-T) · `Wi-Fi 802.11ax` (Wi-Fi 6) · `Fibre optic` (SMF, MMF) · `DSL` `DOCSIS` (cable) · `5G NR` `LTE` |
| **Devices** | NIC · Ethernet cable (Cat5e / Cat6 / Cat6A) · Wi-Fi radio + antenna · Fibre optic cable + SFP transceiver · Hub (repeats bits to all ports — dumb) · Repeater · ISP fibre infrastructure · Submarine cables |

### Signal representations

**Ethernet (copper):**
```
1000BASE-T:  4 twisted pairs, PAM-5 encoding (5 voltage levels: −2, −1, 0, +1, +2 V)
             Full duplex, 250 Mbps per pair × 4 = 1 Gbps
10GBASE-T:   Cat6A or better, PAM-16, up to 100 m
```

**Wi-Fi:**
```
2.4 GHz band:  802.11n — OFDM, up to 600 Mbps
5 GHz band:    802.11ac — OFDM, 256-QAM, up to 3.5 Gbps
6 GHz band:    802.11ax (Wi-Fi 6E) — OFDM, 1024-QAM, up to 9.6 Gbps
Modulation:    Higher QAM order = more bits per symbol, but more noise-sensitive
```

**Fibre optic:**
```
Single-mode (SMF):  9 µm core, laser light — up to 100 km, used for ISP backbone
Multi-mode (MMF):   50/62.5 µm core, LED/VCSEL — up to 2 km, used in data centres
100G / 400G DWDM:   Multiple wavelengths per fibre (dense wave division multiplexing)
```

**No addressing at this layer — just bits.**

### What happens here

- **Ethernet:** bits encoded as voltage differentials on twisted-pair copper. Twisting the pairs cancels electromagnetic interference. Cat6A achieves 10 Gbps up to 100 m.
- **Wi-Fi:** bits encoded as radio wave modulations. OFDM splits the channel into many narrow subcarriers to handle multipath interference (signal bouncing off walls). MIMO uses multiple antennas to send parallel streams.
- **Fibre optic:** bits as light pulses (on/off or phase-modulated). Immune to electromagnetic interference. Used by ISP backbone and all data centre interconnects.
- **Submarine cables:** a handful of fibre cables on the ocean floor carry all transoceanic internet traffic. A single cut (ship anchor, earthquake) can take down connectivity for entire countries.
- **Your home connection at L1:** DSL (phone line copper, up to ~100 Mbps), DOCSIS (coax cable, up to ~1 Gbps), or FTTH — fibre to your premises (up to 10 Gbps symmetric).

---

## Quick-reference: which network component lives at which layer

| Component | OSI Layer | Reads IP? | Reads HTTP? | Caches? | Encrypts? |
|-----------|-----------|-----------|-------------|---------|-----------|
| **Hub** | L1 | No | No | No | No |
| **L2 switch** | L2 | No | No | No | No |
| **Wi-Fi AP** | L2 | No | No | No | No |
| **Router** | L3 | Yes (routing) | No | No | No |
| **L3 stateless firewall** | L3 | IP + port | No | No | No |
| **L4 stateful firewall** | L4 | IP + port + state | No | No | No |
| **VPN (WireGuard)** | L3 (tunnel in L4 UDP) | Yes | No | No | Yes (tunnel) |
| **Load balancer L4** | L4 | IP + port | No | No | Optional |
| **Load balancer L7** | L7 | Yes | Yes (HTTP headers) | No | Optional |
| **Forward proxy** | L7 | Yes | Yes + body (MITM) | Yes | Re-encrypts |
| **CDN / reverse proxy** | L3 + L7 | Yes (anycast) | Yes | Yes | Yes (TLS termination) |
| **WAF** | L7 | Yes | Yes + body | No | No (reads decrypted) |
| **API gateway** | L7 | Yes | Yes + JWT/tokens | Optional | No |
| **DNS resolver** | L7 | No (name only) | No | Yes (TTL) | Optional (DoH) |
| **ISP** | L1–L3 | Yes | No (DPI possible) | No | No |
| **Origin server** | L7 | Yes | Yes (full request) | No (app-level) | No (handled by LB) |

---

## DNS resolution order (before any TCP connection)

```
1. Browser DNS cache         (in-process, Chrome: chrome://net-internals/#dns)
2. OS DNS cache              (Windows DNS Client service / nscd on Linux)
3. hosts file                (wins over everything if entry exists)
   Windows: C:\Windows\System32\drivers\etc\hosts
   Linux:   /etc/hosts
4. Router / local DNS        (192.168.1.1 or corp DNS server)
5. Recursive resolver        (8.8.8.8, 1.1.1.1, or ISP resolver)
6. Authoritative nameserver  (Route53, Cloudflare NS — source of truth)
   └─ With CDN: returns CDN anycast IP, not origin IP
```

---

## TCP/IP vs OSI model mapping

| OSI Layer | TCP/IP Layer | What it maps to |
|-----------|-------------|-----------------|
| L7 Application | Application | HTTP, DNS, SMTP, FTP |
| L6 Presentation | Application | TLS, encoding, compression |
| L5 Session | Application | TLS session, HTTP/2 streams |
| L4 Transport | Transport | TCP, UDP, QUIC |
| L3 Network | Internet | IPv4, IPv6, ICMP, BGP |
| L2 Data Link | Network Access | Ethernet, Wi-Fi, ARP |
| L1 Physical | Network Access | Cables, radio, optics |

TCP/IP collapses OSI L5/L6/L7 into one Application layer and L1/L2 into one Network Access layer. In practice, engineers mostly talk TCP/IP but use OSI layer numbers as shorthand ("that's an L7 problem").


# OSI Model — Complete Reference

| Layer | Name | PDU | Protocols | Devices / Software | Network Components | Message / Frame Contents | Direction | Caches? | Encrypts? | Reads HTTP body? |
|-------|------|-----|-----------|-------------------|-------------------|--------------------------|-----------|---------|-----------|-----------------|
| **L7** | Application | Message | `HTTP/1.1` `HTTP/2` `HTTP/3` `HTTPS` `DNS` `SMTP` `FTP` `WebSocket` `gRPC` `SSH` | Browser, curl, Postman | **Forward proxy** (Squid, Zscaler) · **CDN / reverse proxy** (Cloudflare, nginx) · **API gateway** (Kong, AWS GW) · **WAF** (Cloudflare WAF) | `GET /api/users HTTP/1.1` · `Host:` `Authorization:` `Cache-Control:` `CF-Cache-Status: HIT` · Response: `200 OK` + JSON body | Both | Yes (CDN, proxy) | No (reads decrypted) | Yes |
| **L6** | Presentation | Message (encrypted) | `TLS 1.3` `TLS 1.2` `SSL` (deprecated) · `gzip` `JPEG` `PNG` `Base64` | OS TLS stack (SChannel / OpenSSL / BoringSSL) · Browser TLS engine · Certificate Authority (Let's Encrypt) | TLS termination at CDN PoP or load balancer | `ClientHello` (cipher suites, ECDHE key share, SNI) · `ServerHello` + X.509 cert · `Finished` (encrypted) · Session ticket (resumption) | Both | No | Yes (TLS) | No (encrypts/decrypts) |
| **L5** | Session | Message (session) | `TLS session resumption` · `HTTP/2 streams` · `WebSocket` · `RPC sessions` | Browser session manager · Server session store (Redis) · Load balancer sticky sessions | Load balancer sticky sessions (cookie / IP hash) | HTTP/2 HEADERS frame: `Stream-ID: 1` `:method GET` · WebSocket: `Upgrade: websocket` `Sec-WebSocket-Key:` · TLS session ticket (opaque blob) | Both | No | No | No |
| **L4** | Transport | Segment (TCP) / Datagram (UDP) | `TCP` `UDP` `QUIC` (HTTP/3) `SCTP` | OS TCP/UDP stack | **Stateful firewall** (AWS Security Group, iptables conntrack) · **L4 load balancer** (AWS NLB, HAProxy TCP mode) · **VPN** (WireGuard UDP 51820) | TCP segment: `src-port` `dst-port: 443` `seq` `ack` `flags: SYN/ACK/FIN/RST` `window` `checksum` · Handshake: `SYN` → `SYN-ACK` → `ACK` · Teardown: `FIN` → `FIN-ACK` → `FIN` → `ACK` | Both | No | No (VPN wraps here) | No |
| **L3** | Network | Packet (IP datagram) | `IPv4` `IPv6` · `ICMP` (ping, traceroute) · `BGP` (inter-AS) · `OSPF` (intra-AS) · `IPSec` · `ARP` (IP→MAC) | **Router** (home router, ISP core, BGP border) · **L3 stateless firewall** · **VPN client/server** (tun0 virtual NIC) | ISP backbone (BGP routing) · CDN anycast (same IP prefix announced from all PoPs) · VPN exit node | IPv4 header: `version` `TTL: 64` `protocol: 6(TCP)/17(UDP)/1(ICMP)` `src-IP` `dst-IP` · ICMP: `type: 8` (echo req) / `type: 0` (reply) · BGP UPDATE: `AS_PATH` `NLRI prefix` | Both | No | No | No |
| **L2** | Data Link | Frame (Ethernet / Wi-Fi) | `Ethernet II` (802.3) · `Wi-Fi` (802.11) · `PPP` · `ARP` · `STP` · `VLAN 802.1Q` | **L2 switch** (forwards by MAC) · Wi-Fi access point · NIC · Bridge | Home router = L3 router + L2 switch + L2 Wi-Fi AP in one box | Ethernet frame: `preamble (7B)` `dst-MAC` `src-MAC` `EtherType: 0x0800(IPv4)/0x86DD(IPv6)/0x0806(ARP)` `payload (≤1500B)` `FCS CRC (4B)` · ARP: "Who has 192.168.1.1? Tell 192.168.1.50" · 802.1Q VLAN tag: `TPID 0x8100` `VLAN-ID (12bit)` | Both | No | No | No |
| **L1** | Physical | Bit (raw signal) | `Ethernet` (1000BASE-T, 10GBASE-T) · `Wi-Fi 802.11ax` · `Fibre optic` (SMF/MMF) · `DSL` `DOCSIS` · `5G NR` `LTE` | NIC · Ethernet cable (Cat5e/Cat6/Cat6A) · Wi-Fi radio + antenna · Fibre + SFP transceiver · Hub (repeats to all ports) · Repeater · Submarine cables | ISP last-mile (DSL / DOCSIS / FTTH) · ISP backbone (100G/400G fibre) · Submarine cables (transoceanic) | Ethernet: voltage on copper (PAM-5, ±2V) · Wi-Fi: radio waves (OFDM, 256-QAM / 1024-QAM) · Fibre: light pulses (SMF up to 100 km) · No addressing — just bits | Both | No | No | No |