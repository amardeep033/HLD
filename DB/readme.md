# Database Cheat Sheet — HLD

## 1. SQL Fundamentals

### 1.1 SQL Types

| Type | Purpose | Examples |
|---|---|---|
| DDL | Define schema | `CREATE`, `ALTER`, `DROP`, `TRUNCATE` |
| DML | Modify data | `INSERT`, `UPDATE`, `DELETE` |
| DQL | Query data | `SELECT` |
| DCL | Access control | `GRANT`, `REVOKE` |
| TCL | Transaction control | `COMMIT`, `ROLLBACK`, `SAVEPOINT` |

### 1.2 ER Diagram

| Term | Meaning |
|---|---|
| Entity | Table/object, e.g. `User`, `Order` |
| Attribute | Column/property |
| Primary Key | Unique row identifier |
| Foreign Key | Reference to another table |
| Cardinality | `1:1`, `1:N`, `M:N` |
| Relationship | Association between entities |
| Junction Table | Resolves many-to-many relationship |

### 1.3 Normalisation

| Form | Rule | Fixes |
|---|---|---|
| 1NF | Atomic columns, no repeating groups | Repeated/multi-value fields |
| 2NF | 1NF + no partial dependency on composite key | Partial dependency |
| 3NF | 2NF + no transitive dependency | Derived/indirect dependency |
| BCNF | Every determinant is a candidate key | Stronger 3NF anomalies |
| Denormalisation | Add redundancy intentionally | Read performance / fewer joins |

### 1.4 SQL Design Topics

| Topic | Quick Reference |
|---|---|
| Sharding | Split rows across machines/databases |
| Replication | Copy same data to multiple nodes |
| Partitioning | Split one logical table into smaller physical parts |
| Indexing | Extra data structure for faster reads |
| Transactions | Group operations atomically |
| Constraints | `PRIMARY KEY`, `FOREIGN KEY`, `UNIQUE`, `CHECK`, `NOT NULL` |

## 2. SQL vs NoSQL

### 2.1 Comparison

| Area | SQL | NoSQL |
|---|---|---|
| Model | Relational tables | Document, key-value, column-family, graph |
| Schema | Fixed / strongly structured | Flexible / evolving |
| Query | SQL joins and aggregations | API/query model depends on DB |
| Transactions | Strong ACID support | Often limited or tunable |
| Scaling | Vertical + read replicas + sharding | Horizontal-first in many systems |
| Consistency | Strong by default | Tunable/eventual common |
| Best For | OLTP, financial, relational data | High scale, flexible schema, massive writes |

### 2.2 Common NoSQL Types

| Type | Examples | Use Case |
|---|---|---|
| Key-value | Redis, DynamoDB | Cache, session, cart, counters |
| Document | MongoDB, Couchbase | JSON-like app data, catalog, profile |
| Column-family | Cassandra, HBase | High write scale, time-series, events |
| Graph | Neo4j, Neptune | Relationships, recommendations, fraud graph |
| Search | Elasticsearch, OpenSearch | Full-text search, logs, analytics search |

### 2.3 SQL vs NoSQL Topics

| Topic | SQL Takeaway | NoSQL Takeaway |
|---|---|---|
| Replication + sharding | Available, often more manual/complex | Often core design feature |
| Indexing | B-tree/hash/full-text/GIN/GiST | Engine-specific secondary indexes |
| Transactions | Mature ACID | Varies by database and scope |
| Partitioning | Table-level performance/manageability | Often distribution primitive |
| Read replicas | Common for read scaling | Common, sometimes eventual |
| ACID vs BASE | ACID default expectation | BASE/eventual common trade-off |
| Query optimization | Optimizer + `EXPLAIN` | Query model/access-pattern driven |
| OLTP vs OLAP | Both possible, separate systems at scale | Specialized engines common |
| Sharding strategies | Range/hash/directory/custom | Often built into cluster model |

## 3. Scaling And Distribution

### 3.1 Replication

| Type | Write Path | Read Path | Trade-off |
|---|---|---|---|
| Primary-replica | Writes to primary | Reads from replicas | Replica lag |
| Multi-primary | Writes to multiple primaries | Reads local/nearest | Conflict resolution |
| Synchronous | Wait for replica ack | Stronger durability | Higher write latency |
| Asynchronous | Primary returns first | Lower latency | Data loss/lag risk |
| Quorum | Need `W` writes and `R` reads | Tunable consistency | Complexity |

### 3.2 Read Replicas

| Use | Notes |
|---|---|
| Read scaling | Route read-heavy traffic to replicas |
| Analytics isolation | Keep reporting queries away from primary |
| Disaster recovery | Promote replica if primary fails |
| Risk | Replication lag / stale reads |

### 3.3 Partitioning

| Strategy | How It Splits | Use Case |
|---|---|---|
| Range partitioning | By value range | Dates, IDs, ordered data |
| Hash partitioning | By hash of key | Even distribution |
| List partitioning | By explicit values | Region/status/category |
| Composite partitioning | Combination | Large multi-dimensional datasets |
| Time partitioning | By day/month/year | Logs, metrics, events |

### 3.4 Sharding

| Strategy | Routing Key | Pros | Cons |
|---|---|---|---|
| Hash-based | `hash(user_id)` | Even spread | Hard range queries |
| Range-based | Key ranges | Efficient range scans | Hot shards |
| Directory-based | Lookup service | Flexible placement | Extra routing dependency |
| Geo-based | Region/country | Low latency by region | Cross-region queries |
| Tenant-based | `tenant_id` | Good SaaS isolation | Large tenant imbalance |

### 3.5 Sharding vs Partitioning vs Replication

| Concept | Main Goal | Data Copy? | Across Machines? |
|---|---|---|---|
| Partitioning | Split large table | No | Optional |
| Sharding | Horizontal scale | No | Yes |
| Replication | Availability/read scale | Yes | Usually yes |

## 4. Indexing And Query Optimization

### 4.1 Index Types

| Index | Best For |
|---|---|
| B-tree | Equality + range queries |
| Hash | Equality lookup |
| Composite | Multi-column filters/sorts |
| Unique | Enforce uniqueness |
| Partial | Index subset of rows |
| Covering | Query answered from index only |
| Full-text | Text search |
| GIN/GiST | Arrays, JSONB, geospatial, full-text |

### 4.2 Index Trade-offs

| Helps | Hurts |
|---|---|
| Faster reads | Slower writes |
| Faster filters/sorts | Extra disk/memory |
| Enforces uniqueness | Maintenance overhead |
| Better joins | Bad indexes can be ignored |

### 4.3 Query Optimization Checklist

| Check | Tool/Idea |
|---|---|
| Execution plan | `EXPLAIN`, `EXPLAIN ANALYZE` |
| Missing index | Add index on filter/join/sort columns |
| Too many rows | Pagination / better predicate |
| `SELECT *` | Select only needed columns |
| N+1 queries | Join/fetch join/batching |
| Expensive sort | Composite index matching `WHERE + ORDER BY` |
| Hot query | Cache / materialized view / denormalise |

## 5. Transactions And Consistency

### 5.1 ACID vs BASE

| Model | Meaning | Keywords |
|---|---|---|
| ACID | Strong transactional guarantees | Atomicity, Consistency, Isolation, Durability |
| BASE | Availability-oriented eventual consistency | Basically Available, Soft state, Eventual consistency |

### 5.2 Isolation Levels

| Level | Dirty Read | Non-repeatable Read | Phantom Read |
|---|---|---|---|
| Read Uncommitted | Possible | Possible | Possible |
| Read Committed | Prevented | Possible | Possible |
| Repeatable Read | Prevented | Prevented | Possible/DB-dependent |
| Serializable | Prevented | Prevented | Prevented |

### 5.3 DB Problems / Anomalies

| Problem | Meaning |
|---|---|
| Dirty read | Read uncommitted data from another transaction |
| Non-repeatable read | Same row read twice gives different values |
| Phantom read | Same query returns different row set |
| Lost update | Concurrent writes overwrite each other |
| Write skew | Two transactions read same state, write different rows, violate invariant |
| Deadlock | Transactions wait on each other's locks |
| Lock contention | Many transactions blocked on same resource |
| Replica lag | Replica is behind primary |
| Hot partition/shard | One partition receives disproportionate traffic |
| Split brain | Multiple nodes think they are primary |

### 5.4 Locking And Concurrency Control

| Technique | Use Case |
|---|---|
| Pessimistic lock | Prevent concurrent update upfront |
| Optimistic lock | Version check on commit/update |
| Row lock | Lock specific rows |
| Table lock | Coarse lock for bulk/schema operations |
| MVCC | Readers do not block writers via versions/snapshots |
| Compare-and-swap | Conditional update using version/value |

## 6. Schema And Analytics Models

### 6.1 OLTP vs OLAP

| Area | OLTP | OLAP |
|---|---|---|
| Goal | Run business transactions | Analyze large datasets |
| Queries | Short, frequent, indexed | Long, aggregate-heavy |
| Writes | Frequent small writes | Batch/stream ingestion |
| Schema | Normalized common | Star/snowflake common |
| Examples | Orders, payments, users | Dashboards, BI, reports |

### 6.2 Schema Types

| Schema | Shape | Use Case |
|---|---|---|
| Normalized schema | Many related tables | OLTP correctness, minimal duplication |
| Denormalized schema | Duplicated/read-optimized fields | Read-heavy services |
| Star schema | Fact table + dimension tables | OLAP dashboards |
| Snowflake schema | Normalized dimensions | Complex analytics dimensions |
| Wide table | Many columns, prejoined data | Fast analytical scans / NoSQL-style reads |
| EAV | Entity-attribute-value | Highly dynamic attributes; use carefully |

### 6.3 Star Schema Terms

| Term | Meaning | Example |
|---|---|---|
| Fact table | Measurable event | `sales_fact` |
| Dimension table | Descriptive context | `date_dim`, `product_dim`, `store_dim` |
| Measure | Numeric metric | revenue, quantity, count |
| Grain | One row meaning | one sale line item |
| Surrogate key | Warehouse-generated key | `product_key` |

## 7. Quick Decision Tables

### 7.1 Choose Storage

| Requirement | Common Choice |
|---|---|
| Strong consistency + joins | SQL |
| Flexible JSON documents | Document DB |
| Ultra-fast key lookup | Key-value store |
| High write time-series/events | Column-family / time-series DB |
| Relationship traversal | Graph DB |
| Full-text search | Search engine |
| Analytics/reporting | OLAP warehouse |

### 7.2 Choose Scaling Technique

| Problem | Technique |
|---|---|
| Read load high | Read replicas / cache |
| Write load high | Sharding / partitioning |
| Table too large | Partitioning / archiving |
| Global latency | Geo-replication / regional shards |
| Hot key/shard | Better shard key / key splitting / caching |
| Slow query | Indexing / query optimization / denormalization |
