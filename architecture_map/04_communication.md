# 04 · Communication — how two things talk

> tissues. once work leaves your process, it has to travel.
> prev ← [03](03_system_design.md) · next → [05 event driven](05_event_driven.md) · deep dive → [../networking](../networking/)

## map

```
two things need to talk
├── same process ── function call / interface (→ 09 rpc vs interface)
├── same machine, diff process (IPC) ── pipe | unix socket | shared memory | signal | local tcp
└── over network
    ├── 1. request / response ── caller asks, callee answers
    │   ├── REST ── resources + http verbs, usually json
    │   ├── GraphQL ── one endpoint, client picks exact fields
    │   ├── gRPC ── protobuf + http/2, codegen stubs, 4 modes (unary, server/client/bidi stream)
    │   ├── SOAP ── xml envelope + WSDL contract, WS-* standards, enterprise legacy
    │   ├── JSON-RPC / XML-RPC ── call method by name, minimal
    │   ├── tRPC ── typescript types shared client↔server, no schema file
    │   └── OData ── REST + query language in url (MS)
    ├── 2. server → client push (realtime)
    │   ├── short polling ── client asks every N sec (fake realtime)
    │   ├── long polling ── client asks, server holds until data/timeout
    │   ├── SSE ── one http response kept open, server→client only, text events, auto reconnect
    │   ├── WebSocket ── full duplex over 1 tcp conn after http upgrade (101)
    │   ├── Socket.IO ── lib on top of ws: rooms, acks, reconnect, fallback (NOT plain ws protocol)
    │   ├── SignalR ── .net equivalent of socket.io (ws → sse → long poll fallback)
    │   ├── WebRTC ── peer-to-peer audio/video/data (needs signaling server + STUN/TURN)
    │   └── WebTransport ── http/3 / quic based, newer ws alternative
    ├── 3. streaming (continuous flow)
    │   ├── client-facing ── SSE | gRPC streaming | http chunked | ws
    │   └── backend data streams ── kafka / kinesis → 05
    └── 4. async / event driven ── fire and continue → 05
        ├── via broker ── queue | pub/sub | log
        └── no broker ── webhook / callback (async over sync)
underneath (→ ../networking)
├── transport ── TCP (reliable, ordered) | UDP (fast, lossy) | QUIC (udp + reliability + tls, http/3)
├── app protocol ── HTTP/1.1 | HTTP/2 | HTTP/3 | AMQP (rabbit) | MQTT (iot) | STOMP | kafka wire protocol
└── serialization ── JSON | XML | Protobuf | Avro | Thrift | MessagePack | CBOR
```

## request/response (siblings)

| | REST | GraphQL | gRPC | SOAP |
|---|---|---|---|---|
| shape | resources + verbs | query language | remote procedures | xml envelopes |
| format | json (any) | json | protobuf (binary) | xml |
| transport | http/1.1+ | http (usually POST) | http/2 | http / smtp / anything |
| contract | OpenAPI (optional) | SDL schema (required) | .proto (required) | WSDL (required) |
| browser native | ✓ | ✓ | ✗ (needs grpc-web) | meh |
| sweet spot | public api, CRUD | many clients, varied screens | internal svc↔svc, speed | banks, legacy enterprise |
| pain | over/under fetch | caching, N+1, query cost | debugging, browser | verbose, heavy |

## push / realtime (siblings)

| | direction | connection | when |
|---|---|---|---|
| short polling | client pulls | new req each time | simple, low freq |
| long polling | client pulls, server delays | held req | ws not possible |
| SSE | server → client | 1 long http response | notifications, live feed, LLM token stream |
| WebSocket | both | 1 persistent tcp | chat, games, collab editing |
| Socket.IO / SignalR | both | ws + extras | want rooms/reconnect out of box |
| WebRTC | peer ↔ peer | udp mostly | video call, p2p |
| webhook | server → server | new http call per event | 3rd party notifies you (stripe, github) |

## who initiates / who waits

| pattern | initiator | caller waits? | coupling |
|---|---|---|---|
| request/response | caller | yes | caller knows callee |
| polling | client | per poll | client knows server |
| webhook / callback | callee calls back later | no (got 202) | both know each other |
| pub/sub / events | producer | no | producer doesn't know consumers |

## serialization (siblings)

| format | text/binary | schema | one-liner |
|---|---|---|---|
| JSON | text | optional (JSON Schema) | default for web |
| XML | text | XSD | SOAP, old enterprise |
| Protobuf | binary | required .proto | gRPC, compact, fast |
| Avro | binary | schema in registry | kafka, schema evolution |
| Thrift | binary | IDL | facebook's protobuf |
| MessagePack / CBOR | binary | none | "binary json" |

## http versions

| | one-liner |
|---|---|
| HTTP/1.1 | text, one request at a time per conn (head-of-line blocking) |
| HTTP/2 | binary, multiplexed streams on one tcp conn, header compression |
| HTTP/3 | over QUIC (udp), no tcp head-of-line blocking, faster handshake |

## decision

```
need the answer to continue?         → request/response (REST / gRPC / GraphQL)
server must tell client something?   → SSE (one-way) / WebSocket (two-way)
3rd party tells you later?           → webhook
don't need answer / many listeners?  → event driven → 05
```
