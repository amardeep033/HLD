# 01 · Foundations — hardware + code

> atoms. everything above runs on this.
> next → [02 execution](02_execution.md) · deep dive → [../OS](../OS/readme.md)

## map

```
foundations
├── hardware
│   ├── cpu ─────── cores, hw threads (hyperthreading), clock, registers, L1/L2/L3
│   ├── memory ──── registers > L1 > L2 > L3 > RAM > SSD > HDD > network   (fast+small → slow+big)
│   ├── storage ─── HDD | SSD (SATA) | NVMe | network/object storage (S3)
│   └── io ──────── disk io | network io (NIC) | DMA (device writes RAM, cpu free) | interrupt (device → "done")
├── os
│   ├── unit of execution ── process | thread | green thread / coroutine / fiber / virtual thread
│   ├── kernel space vs user space ── syscall = the door between them
│   ├── virtual memory ── page | page table | page fault | swap | page cache
│   └── scheduler ── time slice, context switch (costly), priority
├── memory layout (one process)
│   ├── text (code) | data (init globals) | bss (uninit globals) | heap ↑ ... ↓ stack
│   └── who frees heap ── manual (C) | GC (Java, Go, JS, C#) | ownership (Rust) | ref counting (Swift, Python)
├── how code runs
│   └── compiled (C, Rust, Go) | interpreted (Python, Ruby) | bytecode + VM + JIT (Java, C#) | transpiled (TS → JS)
├── paradigms
│   ├── imperative ("how")
│   │   ├── procedural / POP ── functions + shared data, top-down (C)
│   │   └── object oriented / OOP ── objects = data + behaviour (Java, C#)
│   ├── declarative ("what")
│   │   ├── functional ── pure fns, immutable, no side effects (Haskell, Elixir, bits of JS/Rust)
│   │   ├── logic ── facts + rules (Prolog)
│   │   └── query / markup / config ── SQL, HTML, Terraform
│   └── cross-cutting styles ── reactive (Rx streams) | event driven programming (callbacks, listeners) | aspect oriented (AOP, Spring @Transactional)
├── oop
│   ├── 4 pillars ── encapsulation | abstraction | inheritance | polymorphism
│   ├── composition vs inheritance ── has-a vs is-a (prefer has-a)
│   └── SOLID
└── design patterns (GoF = in-process, one codebase)
    ├── creational | structural | behavioural
    └── bigger patterns ── architecture → 09/10, distributed → 05/06
```

## memory hierarchy (rough latency)

| level | ~latency | one-liner |
|---|---|---|
| register | <1 ns | inside cpu, what it's computing right now |
| L1 / L2 / L3 | 1 / 4 / 10–40 ns | per-core → shared cpu cache |
| RAM | ~100 ns | process memory, gone on power off |
| NVMe / SSD | 10–100 µs | persistent, no moving parts |
| HDD seek | ~10 ms | spinning disk, sequential ok random bad |
| same-DC network round trip | ~0.5 ms | another service in same region |
| cross-continent round trip | ~150 ms | speed of light problem |

## process vs thread vs coroutine

| unit | memory | switch cost | one-liner |
|---|---|---|---|
| process | own address space | high | isolated, crash doesn't kill others |
| OS thread | shares process heap, own stack (~MB) | medium (kernel) | parallel on cores, shared memory → locks |
| green / virtual thread / goroutine | tiny stack, runtime-managed | low (user space) | millions possible, runtime maps M:N onto OS threads |
| coroutine / async task | state machine on heap (bytes) | very low | suspends at `await`, cooperative |

## stack vs heap

| | stack | heap |
|---|---|---|
| holds | local vars, call frames | objects, dynamic size data |
| alloc | move pointer (fast) | allocator / GC (slower) |
| lifetime | ends with function | until freed / GC'd |
| per | thread | process (shared) |

## paradigms (siblings)

| paradigm | unit | state | example |
|---|---|---|---|
| procedural (POP) | function | shared, mutable | C |
| OOP | object | inside object, mutable | Java, C# |
| functional | pure function | immutable | Haskell, Elixir |
| logic | rule | facts | Prolog |
| reactive | stream | flows through pipes | RxJS, Reactor |

## SOLID

| | one-liner |
|---|---|
| S — single responsibility | one reason to change |
| O — open/closed | extend without editing |
| L — liskov substitution | subclass usable wherever parent is |
| I — interface segregation | many small interfaces > one fat |
| D — dependency inversion | depend on abstractions, not concrete (→ DI / IoC container) |

## GoF patterns

**creational** — how objects get made

| pattern | one-liner |
|---|---|
| singleton | one instance globally |
| factory method | subclass decides which class to create |
| abstract factory | family of related objects |
| builder | step by step construction |
| prototype | clone an existing object |

**structural** — how objects are composed

| pattern | one-liner |
|---|---|
| adapter | convert one interface to another |
| bridge | split abstraction from implementation |
| composite | tree, treat leaf & group same |
| decorator | wrap to add behaviour |
| facade | simple front for a messy subsystem |
| flyweight | share common state to save memory |
| proxy | stand-in that controls access (lazy, remote, auth) |

**behavioural** — how objects talk

| pattern | one-liner |
|---|---|
| chain of responsibility | pass request along handlers (middleware) |
| command | request as an object (queue it, undo it) |
| iterator | walk a collection without knowing its guts |
| mediator | objects talk via a hub, not each other |
| memento | snapshot state for undo |
| observer | subject notifies subscribers on change |
| state | behaviour changes with internal state |
| strategy | swap algorithm at runtime |
| template method | skeleton in parent, steps in child |
| visitor | add operation without changing classes |
| interpreter | grammar → evaluator |

**non-GoF but everywhere** — DI / IoC · repository · unit of work · specification · null object · DTO · service locator (anti-ish)
