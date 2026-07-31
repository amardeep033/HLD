# 02 · Execution — sync / async / blocking / where work runs

> molecules. how one piece of work gets executed.
> prev ← [01](01_foundations.md) · next → [03 system design](03_system_design.md)
> full notes → `Java-Playground/2_spring_boot/06_async_processing_executors/readme2.md`

## map

```
execution
├── nature of work ── cpu-bound (computing) | io-bound (waiting on disk/net/db) | memory-bound (memcpy)
├── 3 independent questions  ← most confusion = mixing these
│   ├── Q1 where does it run? ── same thread | other thread (same process) | other process | other service
│   ├── Q2 does caller wait? ─── sync (wait for result) | async (continue, result later)
│   └── Q3 waiting thread? ───── blocking (parked by OS) | non-blocking (free for other work)
├── concurrency vs parallelism ── dealing with many things | doing many things at the same instant
├── concurrency models
│   ├── thread per request ── classic tomcat, simple, 1 thread parked per waiting request
│   ├── thread pool / executor ── reuse N threads, queue tasks
│   ├── event loop ── 1 thread, many tasks, never block it (node, netty, nginx, redis)
│   ├── async/await + runtime ── state machines on few threads (tokio, .net, asyncio)
│   ├── virtual / green threads ── looks blocking, runtime parks cheaply (java 21 loom, goroutines)
│   ├── actor ── state + mailbox, no shared memory (akka, erlang/elixir, orleans)
│   ├── CSP ── share memory by communicating over channels (go)
│   └── reactive streams ── push pipeline + backpressure (Reactor/WebFlux, RxJava)
├── os io models
│   ├── blocking io ── read() parks thread
│   ├── non-blocking io ── read() returns EAGAIN, you poll
│   ├── io multiplexing ── one thread watches many fds: select | poll | epoll (linux) | kqueue (bsd/mac)
│   ├── async io ── kernel does the whole op: io_uring (linux) | IOCP (windows)
│   └── reactor ("ready, you read") vs proactor ("done, data in your buffer")
├── result handle evolution ── callback → future/promise → async/await → stream/observable
└── shared-memory safety
    ├── primitives ── mutex | rwlock | semaphore | condition variable | atomic / CAS | latch / barrier
    └── bugs ── race condition | deadlock | livelock | starvation | priority inversion
```

## the 3 axes, one table

| scenario | where | caller | blocked thread |
|---|---|---|---|
| `read()` on main thread | same thread | sync | main |
| `await db.fetch()` (tokio, socket) | db server | async | none |
| `await tokio::fs::read()` | pool thread | async | pool thread (your worker free) |
| REST call, wait for response | other service | sync | your thread |
| publish to kafka, move on | broker | async | none |
| `read(O_NONBLOCK)` in loop | same thread | sync | none, but busy polling |
| `submit()` then `.get()` immediately | other thread | effectively sync | your thread |

## common combos

| combo | one-liner |
|---|---|
| sync + blocking + same thread | normal code |
| async + non-blocking + other thread/service | what people mean by "async" |
| async + blocking (other thread) | thread-per-request, `spawn_blocking` — fine unless it's the event loop thread |
| sync + non-blocking | poll loop, rare |

## where can async work go? (the ladder that leads to 04/05)

| where | mechanism | survives crash? | next |
|---|---|---|---|
| same thread, later | event loop / await | no | — |
| other thread, same process | executor, `@Async`, `tokio::spawn` | no | — |
| other process, same box | worker process, cron | partly | — |
| other service via broker | queue / topic / stream | yes (durable) | [05](05_event_driven.md) |
| other service direct | REST/gRPC, webhook callback | depends | [04](04_communication.md) |

## concurrency models (siblings)

| model | shared memory? | unit | example |
|---|---|---|---|
| threads + locks | yes | OS thread | java classic, C++ |
| event loop | single thread | callback / task | node, netty |
| async/await | usually no | future / task | tokio, C#, python |
| virtual threads | yes | lightweight thread | java loom, go |
| actor | no | actor + mailbox | akka, erlang |
| CSP | no | goroutine + channel | go |

## language cheat

| | fire on other thread | get result later | wait | async on same thread |
|---|---|---|---|---|
| Java | `executor.execute(r)` | `submit(c)` → `Future` / `CompletableFuture` | `.get()` / `.join()` | — (virtual threads instead) |
| Spring | `@Async` | returns `CompletableFuture` | `.get()` | WebFlux `Mono/Flux` |
| C# | `Task.Run` | `Task<T>` | `.Result` (bad) / `await` | `async/await` |
| Rust | `tokio::spawn` / `spawn_blocking` | `JoinHandle` | `.await` | `async fn` + `.await`, `join!` |
| JS | worker thread | `Promise` | `await` | event loop, `async/await` |
| Go | `go f()` | channel | `<-ch` / `WaitGroup` | goroutines (runtime handles) |

## one-liners

- threads ≠ async. async = caller flow doesn't wait. work can be on same thread later.
- blocking is per thread, not per system.
- cpu-bound → parallelism (more cores). io-bound → overlap waiting (async).
- 50ms cpu work inside an async task freezes every other task on that thread.
- epoll can't do regular files (always "ready") → tokio::fs uses a blocking pool; io_uring fixes it.
