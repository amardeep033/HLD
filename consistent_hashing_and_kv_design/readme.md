# Key-Value Store Design & Consistent Hashing

Study notes covering distributed KV store design — sharding, consistent hashing, replication, consistency, failure handling, and storage internals.

---

## Table of Contents

1. [Distribution & Sharding](#1-distribution--sharding)
2. [Consistent Hashing](#2-consistent-hashing)
3. [CAP Theorem](#3-cap-theorem)
4. [Replication](#4-replication)
5. [Conflict Resolution](#5-conflict-resolution)
6. [Failure Detection & Recovery](#6-failure-detection--recovery)
7. [Node Architecture](#7-node-architecture)
8. [Write Path — LSM Tree](#8-write-path--lsm-tree)
9. [Read Path](#9-read-path)

---

## 1. Distribution & Sharding

A single server cannot hold all data when the dataset is large — keys must be distributed across multiple servers.

**Two main approaches:**

| Approach | How it works | Problem |
|---|---|---|
| Range-based | Split key space into ranges, assign each range to a server | Hotspots when data is skewed (e.g. keys starting with `A` vs `S`) |
| Hash-based | Hash the key, assign to server based on hash output | Modulo hashing requires remapping almost all keys on topology change |

**Modulo hashing problem:** `server = hash(key) % N` — adding or removing one server changes `N`, causing nearly all keys to remap.

---

## 2. Consistent Hashing

Solves the modulo hashing problem by limiting reshuffling to only the keys in the affected arc.

### Core Idea

- Both **servers** and **keys** are mapped to positions on the same ring using the same hash function.
- Keys are hashed to ring positions at lookup time — positions are **not stored persistently**.
- Server positions are stored in the routing layer.
- Each key is owned by the **first server found clockwise** from its position on the ring.

### Adding / Removing a Server

- Only keys in the arc between the new/removed server and its predecessor are reshuffled — a small fraction of total keys.
- Reshuffling can be **lazy** (acceptable for cache/temporary data) or **eager** (required for permanent data with consistency guarantees).

### Hotspot Problem

Servers may land unevenly on the ring, making some responsible for a larger arc than others.

### Virtual Nodes

- Each physical server maps to **multiple positions** on the ring.
- More powerful servers get more virtual nodes — proportional load assignment.
- Tradeoffs of virtual nodes:

| Benefit | Cost |
|---|---|
| More even load distribution | More memory to track ring positions |
| Graceful scaling | More complex rebalancing logic |

> **Note:** Virtual nodes do **not** increase total keys reshuffled — each virtual node covers a smaller arc, so aggregate reshuffling is similar to without virtual nodes.

---

## 3. CAP Theorem

A distributed system cannot simultaneously guarantee all three: **Consistency**, **Availability**, and **Partition Tolerance**.

| Type | Guarantee | Behaviour during partition |
|---|---|---|
| CA | Consistent + Available | Not realistically achievable — network partitions are inevitable |
| **CP** | Consistent + Partition-tolerant | Rejects requests when latest data cannot be guaranteed |
| **AP** | Available + Partition-tolerant | Accepts requests even if stale data may be returned |

> **CP does not mean synchronous replication on every write** — it means requests are rejected when the system cannot guarantee the latest data.

### Quorum vs CP/AP

Quorum settings (`W`, `R`, `N`) control the **consistency level** of reads and writes. They do **not** determine whether a system is CP or AP.

- **CP vs AP is determined by:** whether the system rejects or accepts requests when a node is unreachable.
- **Quorum determines:** how many replicas must acknowledge a read/write for it to succeed.

---

## 4. Replication

Replication provides **fault tolerance**, **availability**, and **backup**.

With consistent hashing and replication, each key is stored on **N consecutive clockwise servers** on the ring.

Each server acts as:
- **Primary** for some keys
- **Replica** for other keys

### Quorum Read / Write

```
W = write quorum (min replicas that must acknowledge a write)
R = read quorum  (min replicas that must acknowledge a read)
N = total replicas per key
```

| Configuration | Condition | Consistency |
|---|---|---|
| Strong (CP) | `W + R > N` | At least one replica with the latest write is always included in a read |
| Eventual (AP) | `W + R ≤ N` | Reads may return stale data; replicas converge over time |

**Common strong consistency config:** `N=3, W=2, R=2`

---

## 5. Conflict Resolution

Concurrent writes to the same key on different replicas can cause conflicts.

### Versioning

Each value carries a version number. On merge, the higher version wins.

### Vector Clocks

Track per-server version counters to detect causality and resolve diverged branches.

```
D([S1, v1], [S2, v2], ..., [Sn, vn])
```

- `D` = data item
- `Si` = server ID
- `vi` = version counter on that server

Allows the system to determine if one version is a descendant of another, or if two versions are concurrent (conflict requiring resolution).

### Write Acceptance Models

| Model | Who accepts writes | Tradeoff |
|---|---|---|
| Leader-based | Only primary replica | Simpler, but primary is a bottleneck |
| Leaderless | Any replica | Higher availability, requires conflict resolution |

---

## 6. Failure Detection & Recovery

### Detection — Gossip Protocol

- Each node periodically sends **heartbeat messages** to a random subset of peers.
- If a node stops receiving heartbeats from another node, it queries other nodes about that node's status.
- If a **majority confirm** the node is unresponsive, it is marked as failed.

### Temporary Failure

Handled with **sloppy quorum + hinted handoff** (the two work together):

1. **Sloppy quorum:** Route writes/reads to the first `W`/`R` *healthy* nodes on the ring, skipping the offline node.
2. **Hinted handoff:** The substitute node stores the data with a *hint* identifying the intended owner. Once the failed node recovers, the substitute transfers the data back.

### Permanent Failure

Handled with **anti-entropy using Merkle trees:**

- A Merkle tree is a hash tree where each leaf is a hash of a data block, and each internal node is a hash of its children.
- Nodes periodically compare their Merkle tree roots. A mismatch means diverged data — the tree is traversed to find and sync only the differing blocks.
- Much more efficient than comparing all data directly.

### Disaster Recovery

**Cross-datacenter replication** handles full datacenter failures.

---

## 7. Node Architecture

Each node in a KV store is responsible for:

- Accepting client reads and writes
- Storing primary and replica data
- Replicating data to peer nodes
- Detecting failures and participating in recovery
- Rebalancing data when the ring topology changes

---

## 8. Write Path — LSM Tree

Writes use an **LSM (Log-Structured Merge) tree** for durability and performance:

```
Write request
    │
    ▼
1. Commit log (disk)     ← append-only; ensures durability on crash
    │
    ▼
2. Memtable (RAM)        ← in-memory sorted buffer; fast writes
    │
    │  (when memtable exceeds threshold)
    ▼
3. SSTable (disk)        ← immutable sorted file flushed from memtable
```

> **Terminology note:** The in-memory buffer is called a **memtable** — not to be confused with Memcached, which is a separate distributed caching system.

---

## 9. Read Path

```
Read request for key K
    │
    ▼
1. Check memtable        → return if found
    │
    ▼
2. Check SSTables        → use bloom filter to skip files that
   (disk, newest first)    definitely don't contain K
    │
    ▼
3. Not found             → return null / key-not-found error
```

### Bloom Filter

A probabilistic data structure that answers: *"does this SSTable definitely NOT contain key K?"*

- **No false negatives** — if the filter says absent, the key is definitely not in that file.
- **Possible false positives** — if the filter says present, the file still needs to be checked.
- Avoids unnecessary disk reads, significantly speeding up reads for missing keys.

---

## Quick Reference

| Concept | Key formula / rule |
|---|---|
| Strong consistency | `W + R > N` |
| Eventual consistency | `W + R ≤ N` |
| Common strong config | `N=3, W=2, R=2` |
| Key ownership | First server clockwise on ring |
| Temp failure | Sloppy quorum + hinted handoff |
| Perm failure | Anti-entropy with Merkle trees |
| Write durability order | Commit log → memtable → SSTable |