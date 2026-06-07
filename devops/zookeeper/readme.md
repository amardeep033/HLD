# ClickHouse Cluster Notes

This workspace runs a small ClickHouse cluster with Docker Compose and one ZooKeeper container.

The main idea is:

- Docker Compose creates the network, starts the containers, and mounts the right config into each container.
- ZooKeeper acts as the shared coordination service for replicated ClickHouse metadata.
- ClickHouse stores the actual table data on each node's local disk.

## What is running here

`docker-compose.yml` starts five containers:

- `zookeeper`
- `clickhouse-node1`
- `clickhouse-node2`
- `clickhouse-node3`
- `clickhouse-node4`

The ClickHouse layout is:

- Shard 1: `clickhouse-node1`, `clickhouse-node2`
- Shard 2: `clickhouse-node3`, `clickhouse-node4`
- One replica pair per shard

This comes from `configs/metrika.xml` and the per-node macro files under `configs/macros/`.

## Docker Compose role

Docker Compose is the orchestration layer for this local cluster.

In this project it is responsible for:

- Starting all containers on the same Docker network so container hostnames resolve to each other.
- Mapping host ports to container ports.
- Mounting config files into each ClickHouse container.
- Mounting persistent data directories from the host into the containers.
- Starting ClickHouse containers after ZooKeeper is started with `depends_on`.

Important detail: `depends_on` controls startup order, not application readiness. That means Docker Compose can start ZooKeeper first, but ClickHouse may still try to connect before ZooKeeper is fully ready. In small local setups this often works anyway, but it is not a strong health guarantee.

## ZooKeeper role

ZooKeeper is not storing your ClickHouse table data.

Its job here is coordination. In a replicated ClickHouse setup, ZooKeeper is typically used to keep track of things like:

- replica registration
- replication queue metadata
- distributed DDL task coordination
- leader and replica state for replicated engines

In this workspace, each ClickHouse node includes:

- a ZooKeeper reference via `<zookeeper incl="zookeeper" />`
- distributed DDL coordination via `<distributed_ddl><path>/clickhouse/task_queue/ddl</path></distributed_ddl>`

That means ZooKeeper is the shared place where the nodes agree on replicated and cluster-wide coordination state.

## How the ClickHouse nodes are wired

Each ClickHouse container mounts these important files:

- `configs/nodeX/config.xml` as the main server config
- `configs/nodeX/users.xml` for users and auth
- `configs/metrika.xml` for shared cluster and ZooKeeper definitions
- `configs/macros/macros-nodeX.xml` for node-specific shard and replica identity

### Shared cluster definition

`configs/metrika.xml` defines:

- cluster name: `my_cluster`
- shard membership
- replica membership
- ZooKeeper host: `zookeeper:2181`

This is the file that tells ClickHouse how all nodes relate to each other.

### Per-node identity

Each file in `configs/macros/` gives a node its local identity.

Examples:

- node1: shard `1`, replica `replica_1`
- node2: shard `1`, replica `replica_2`
- node3: shard `2`, replica `replica_1`
- node4: shard `2`, replica `replica_2`

These macros are usually referenced by replicated table engines so the same table definition can be reused across nodes while each node still knows its own shard and replica name.

## Data flow: who stores what

There are two different kinds of state in this setup.

### 1. ClickHouse data files

Actual table parts and server data live in the per-node data directories mounted to:

- `./node1-data -> /var/lib/clickhouse`
- `./node2-data -> /var/lib/clickhouse`
- `./node3-data -> /var/lib/clickhouse`
- `./node4-data -> /var/lib/clickhouse`

If you remove one node's data directory, that node loses its local stored data.

### 2. ZooKeeper coordination state

ZooKeeper stores its own coordination metadata under:

- `./zookeeper-data -> /data`

If you remove ZooKeeper state, the coordination metadata for replicated objects can be lost or reset, depending on what was created and how recovery is handled.

## Ports exposed to your host

Each ClickHouse node exposes different host ports so you can connect to them individually.

### Node 1

- HTTP: `8123`
- native TCP: `9000`
- interserver HTTP: `9009`

### Node 2

- HTTP: `8124`
- native TCP: `9001`
- interserver HTTP: `9010`

### Node 3

- HTTP: `8125`
- native TCP: `9002`
- interserver HTTP: `9011`

### Node 4

- HTTP: `8126`
- native TCP: `9003`
- interserver HTTP: `9012`

### ZooKeeper

- client port: `2181`

## What happens when you run Docker Compose

When you run `docker compose up -d`, the rough sequence is:

1. Docker creates the containers and network.
2. ZooKeeper starts and exposes port `2181`.
3. ClickHouse containers start.
4. Each ClickHouse node reads its mounted config.
5. Each node learns:
   - how to identify itself from macros
   - how to find ZooKeeper
   - how the cluster is laid out
6. If you create replicated tables, the nodes use ZooKeeper to coordinate those replicas.
7. Table data itself is still written to each node's own local ClickHouse storage.

## Why both Docker Compose and ZooKeeper are needed

These two tools solve different problems.

### Docker Compose solves container lifecycle and local infrastructure wiring

It answers questions like:

- Which containers should exist?
- Which image should each container run?
- Which ports should be exposed?
- Which config files should be mounted?
- Which local directories should persist data?

### ZooKeeper solves distributed coordination inside the database cluster

It answers questions like:

- Which replicas belong to a replicated table?
- What replication work is pending?
- How do nodes share DDL tasks?
- Which node identity is attached to a replicated object path?

Compose gets the services running. ZooKeeper helps the distributed database behave like a coordinated cluster.

## Useful files in this workspace

- `docker-compose.yml`: service definitions and volume/port mappings
- `configs/metrika.xml`: cluster topology and ZooKeeper endpoint
- `configs/node1/config.xml` through `configs/node4/config.xml`: node server settings
- `configs/node1/users.xml` through `configs/node4/users.xml`: user accounts
- `configs/macros/`: shard and replica identity per node
- `zookeeper-data/`: persisted ZooKeeper state

Note that the root-level `config.xml` in this workspace looks like a ClickHouse client config, not the server config used by the containers in `docker-compose.yml`.

## Authentication currently configured

The mounted `users.xml` files define at least these users:

- `default` with empty password
- `admin` with password `admin@123`
- `interserver` with password `interserver_password`

`interserver` is used for internal ClickHouse node-to-node communication. The `admin` user is also referenced in the remote server definitions.

## Generic mental model

If you are trying to build intuition, this is the simplest way to think about it:

- Docker Compose is the local infrastructure manager.
- ClickHouse is the database engine running on each node.
- ZooKeeper is the shared coordinator for replicated cluster behavior.
- Local node disks hold the actual data.
- Shared metadata in ZooKeeper helps replicas stay in sync.

## Practical caveats

- This setup uses a single ZooKeeper container, so it is fine for learning and local development but not a highly available production coordination layer.
- Because the data directories are bind mounts, deleting local folders affects persisted state.
- The cluster topology is static here. If you add nodes, you need to update the shared cluster config and likely add more macro and service definitions.

## Common commands

Start everything:

```bash
docker compose up -d
```

Stop everything:

```bash
docker compose down
```

See running containers:

```bash
docker compose ps
```

Follow logs for one node:

```bash
docker compose logs -f clickhouse-node1
```

Connect to node1 with the ClickHouse client inside the container:

```bash
docker exec -it clickhouse-node1 clickhouse-client -u admin --password admin@123
```

If you want, the next useful step would be to add a second section with example replicated table DDL and show exactly where ZooKeeper becomes visible in practice.