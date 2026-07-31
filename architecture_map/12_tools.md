# 12 · Tools — concrete products per concept

> the clothes. concepts are permanent, tools rotate. always ask "which concept does this tool implement?"
> prev ← [11](11_theories_laws.md) · back → [README](README.md)

## map

```
tools (concept → products)
├── observability ── standard: OpenTelemetry · metrics: Prometheus · logs: ELK/EFK, Loki · traces: Jaeger, Zipkin, Tempo · viz: Grafana, Kibana
├── cache ── Redis, Valkey, Memcached, Hazelcast · in-process: Caffeine, Guava, IMemoryCache · http: Varnish, CDN
├── messaging
│   ├── broker (queue/pubsub) ── RabbitMQ, ActiveMQ/Artemis, AWS SQS + SNS, Azure Service Bus, GCP Pub/Sub, NATS
│   ├── log/stream ── Kafka, Redpanda, Pulsar, AWS Kinesis, Azure Event Hubs, Redis Streams, NATS JetStream
│   ├── event router ── AWS EventBridge, Azure Event Grid
│   └── iot ── MQTT: Mosquitto, EMQX, HiveMQ
├── stream / batch processing ── Kafka Streams, Flink, Spark (Structured Streaming), ksqlDB, Beam · schedule: Airflow, cron
├── CDC ── Debezium, AWS DMS, Maxwell
├── workflow / saga orchestration ── Temporal, Camunda/Zeebe, AWS Step Functions, Azure Durable Functions, Conductor
├── in-process mediator / events ── MediatR (.NET), Spring ApplicationEvents, Axon (java cqrs/es)
├── databases → ../DB
│   ├── relational ── PostgreSQL, MySQL, SQL Server, Oracle, Aurora
│   ├── document ── MongoDB, Cosmos DB, Couchbase
│   ├── key-value ── DynamoDB, Redis, etcd
│   ├── wide column ── Cassandra, ScyllaDB, HBase, Bigtable
│   ├── graph ── Neo4j, Neptune
│   ├── search ── Elasticsearch, OpenSearch, Solr, Typesense
│   ├── time series ── InfluxDB, TimescaleDB, Prometheus TSDB
│   ├── vector ── pgvector, Pinecone, Qdrant, Weaviate, Milvus (→ ../AI)
│   └── OLAP / warehouse ── ClickHouse, Snowflake, BigQuery, Redshift, Druid
├── edge / proxy / gateway
│   ├── reverse proxy / LB ── nginx, HAProxy, Envoy, Traefik, Caddy, AWS ALB/NLB, Azure App Gateway / Front Door
│   ├── API gateway ── Kong, Apigee, AWS API Gateway, Azure APIM, Spring Cloud Gateway, YARP, Tyk
│   └── CDN / WAF ── Cloudflare, Akamai, CloudFront, Azure Front Door, Fastly
├── service discovery / coordination ── Consul, Eureka, etcd, ZooKeeper, k8s DNS
├── service mesh ── Istio, Linkerd, Consul Connect, Cilium
├── resilience libs ── Resilience4j, Polly (.NET), Hystrix (retired), Envoy policies, failsafe
├── containers & orchestration → ../devops ── Docker, Podman, Kubernetes, Helm, ECS, AKS/EKS/GKE, Nomad
├── IaC & config ── Terraform, OpenTofu, Pulumi, Bicep/ARM, CloudFormation, Ansible
├── CI/CD ── GitHub Actions, GitLab CI, Jenkins, Azure DevOps, ArgoCD / Flux (GitOps)
├── API tooling ── OpenAPI/Swagger, AsyncAPI, Postman, protobuf + buf, GraphQL (Apollo, Hot Chocolate)
├── identity & secrets → 08 ── Keycloak, Auth0, Okta, Entra ID, Cognito · Vault, Azure Key Vault, AWS Secrets Manager
├── frontend → 07 ── React/Angular/Vue · Next/Nuxt · Vite/webpack · Nx/Turborepo · Fluent UI/MUI
└── cloud → ../CLOUD ── AWS | Azure | GCP (each has its own name for every box above)
```

## deep-dive folders in this repo

| concept | folder |
|---|---|
| OS, memory, processes | [../OS](../OS/readme.md) |
| networking, OSI, tcp/http | [../networking](../networking/) |
| databases | [../DB](../DB/readme.md) |
| cache, LRU | [../cache](../cache/readme.md) |
| redis | [../redis](../redis/readme.md) |
| kafka | [../kafka](../kafka/readme.md) |
| rate limiter | [../rate_limiter](../rate_limiter/readme.md) |
| retry & circuit breaker | [../retry_and_circuit_breaker](../retry_and_circuit_breaker/) |
| consistent hashing, kv store | [../consistent_hashing_and_kv_design](../consistent_hashing_and_kv_design/readme.md) |
| observability / otel | [../observability](../observability/readme.md) |
| devops, docker, k8s | [../devops](../devops/readme.md) |
| cloud | [../CLOUD](../CLOUD/readme.md) |
| AI / LLM systems | [../AI](../AI/README.md) |

## same box, three clouds

| concept | AWS | Azure | GCP |
|---|---|---|---|
| queue | SQS | Service Bus queue / Storage Queue | Pub/Sub (pull) / Cloud Tasks |
| pub/sub | SNS | Service Bus topic | Pub/Sub |
| stream | Kinesis / MSK | Event Hubs | Pub/Sub / Managed Kafka |
| event router | EventBridge | Event Grid | Eventarc |
| workflow | Step Functions | Durable Functions / Logic Apps | Workflows |
| serverless fn | Lambda | Functions | Cloud Functions / Run |
| k8s | EKS | AKS | GKE |
| api gateway | API Gateway | APIM | API Gateway / Apigee |
| cdn | CloudFront | Front Door | Cloud CDN |
| cache | ElastiCache | Azure Cache for Redis | Memorystore |
| secrets | Secrets Manager | Key Vault | Secret Manager |
| identity | Cognito / IAM | Entra ID | Identity Platform / IAM |
| monitoring | CloudWatch / X-Ray | Azure Monitor / App Insights | Cloud Monitoring / Trace |

## messaging tools (siblings)

| | model | retention | ordering | one-liner |
|---|---|---|---|---|
| RabbitMQ | broker (queue + exchanges) | until consumed | per queue | flexible routing, AMQP |
| Kafka | log | time/size | per partition | high throughput event backbone |
| SQS | queue | ≤14 days | FIFO variant | managed, zero ops |
| SNS | pub/sub | none | FIFO variant | fan-out to SQS/lambda/http |
| Redis Pub/Sub | pub/sub | none | — | fire & forget, fast |
| Redis Streams | log | memory bound | per stream | kafka-lite |
| Pulsar | log + queue | tiered | per partition | multi-tenant, geo |
| NATS | pub/sub (+JetStream log) | optional | per subject | tiny, fast |
