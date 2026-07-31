# AWS Cloud Cheat Sheet — HLD / SDE2

## 1. Cloud Basics

### 1.1 Service Models

| Model | You Manage | Provider Manages | AWS Examples |
|---|---|---|---|
| IaaS | OS, runtime, app, data | Physical infra, virtualization | EC2, EBS, VPC |
| PaaS | App, data | OS, runtime, scaling platform | Elastic Beanstalk, App Runner |
| CaaS | Containers, app, data | Cluster/control plane varies | ECS, EKS, Fargate |
| FaaS / Serverless | Function code, config | Servers, scaling, runtime ops | Lambda |
| SaaS | Usage/config | Full application | Amazon QuickSight, managed AWS consoles/tools |

### 1.2 AWS Global Infrastructure

| Term | Meaning |
|---|---|
| Region | Geographic AWS location, e.g. `us-east-1` |
| Availability Zone | Isolated data center group inside a region |
| Edge Location | CDN/DNS/security edge location |
| Multi-AZ | High availability within one region |
| Multi-Region | Disaster recovery / global latency strategy |
| Local Zone | AWS infra closer to specific metro areas |

### 1.3 AWS Service Categories

| Category | Core Services |
|---|---|
| Compute | EC2, Auto Scaling, Lambda, ECS, EKS, Fargate |
| Storage | S3, EBS, EFS, Glacier / S3 Glacier |
| Database | RDS, Aurora, DynamoDB, ElastiCache, Redshift |
| Networking | VPC, Subnet, Route Table, NAT Gateway, ALB/NLB, Route 53, CloudFront |
| Security | IAM, KMS, Secrets Manager, ACM, WAF, Shield |
| Messaging | SQS, SNS, EventBridge, MSK/Kinesis |
| Observability | CloudWatch, CloudTrail, X-Ray, Config |
| Deployment | CodeBuild, CodeDeploy, CodePipeline, ECR |

## 2. Storage Types

### 2.1 Storage Comparison

| Type | AWS Service | Best For | Access Pattern |
|---|---|---|---|
| Object storage | S3 | Images, videos, backups, logs, static files | HTTP/object key |
| Block storage | EBS | EC2 disks, databases, low-latency volumes | Mounted block device |
| File storage | EFS | Shared filesystem across EC2/tasks | NFS mount |
| Archive storage | S3 Glacier | Long-term backup/compliance | Rare retrieval |
| In-memory storage | ElastiCache | Cache/session/hot data | Redis/Memcached protocol |
| Data warehouse storage | Redshift | Analytics/OLAP | Columnar warehouse |

### 2.2 Storage Decision Table

| Requirement | Choose |
|---|---|
| Store user uploads/static assets | S3 |
| Disk for one EC2 instance | EBS |
| Shared filesystem across instances | EFS |
| Cheap long-term archive | S3 Glacier |
| Cache hot reads | ElastiCache Redis |
| Analytical queries over huge data | Redshift / S3 + Athena |

## 3. S3 Bucket

### 3.1 S3 Basics

| Concept | Meaning |
|---|---|
| Bucket | Top-level container for objects |
| Object | File/data stored in S3 |
| Key | Object path/name inside bucket |
| Versioning | Keep multiple versions of same key |
| Metadata | Key-value info attached to object |
| Prefix | Key path prefix used for organization/filtering |
| Event notification | Trigger Lambda/SQS/SNS/EventBridge on object event |

### 3.2 S3 Storage Classes

| Class | Use Case |
|---|---|
| Standard | Frequently accessed data |
| Intelligent-Tiering | Unknown/changing access patterns |
| Standard-IA | Infrequent access, quick retrieval |
| One Zone-IA | Infrequent access, lower cost, single AZ |
| Glacier Instant Retrieval | Archive with millisecond access |
| Glacier Flexible Retrieval | Archive, minutes-hours retrieval |
| Glacier Deep Archive | Cheapest long-term archive |

### 3.3 S3 Security

| Feature | Use |
|---|---|
| Bucket policy | Resource-based bucket access rules |
| IAM policy | Principal-based access rules |
| Block Public Access | Prevent accidental public exposure |
| ACL | Legacy object/bucket permissions; avoid unless needed |
| SSE-S3 | Server-side encryption with S3-managed keys |
| SSE-KMS | Server-side encryption with KMS keys |
| Pre-signed URL | Temporary upload/download access |
| VPC endpoint | Private VPC access to S3 |

### 3.4 S3 Common Design Patterns

| Pattern | AWS Pieces |
|---|---|
| Static website/media hosting | S3 + CloudFront + Route 53 + ACM |
| User file upload | Pre-signed URL + S3 + backend metadata DB |
| Image/video processing | S3 event + Lambda/ECS worker |
| Data lake | S3 + Glue Catalog + Athena/EMR/Redshift Spectrum |
| Backup/archive | S3 lifecycle policy + Glacier classes |
| Cross-region DR | Versioning + cross-region replication |

### 3.5 S3 Performance / Cost

| Concern | Cheat Sheet |
|---|---|
| Large upload | Multipart upload |
| Large download | CloudFront / byte-range requests |
| Cost control | Lifecycle policy, storage classes, delete old versions |
| Data protection | Versioning + MFA delete/Object Lock where needed |
| Access audit | CloudTrail data events / S3 access logs |

## 4. Compute

### 4.1 Compute Options

| Service | Use Case | Notes |
|---|---|---|
| EC2 | Full VM control | You manage OS/patching/runtime |
| Auto Scaling Group | Horizontal EC2 scaling | Health checks + desired capacity |
| Lambda | Event-driven functions | Short-lived, serverless, auto-scale |
| ECS | Container orchestration | AWS-native container service |
| EKS | Managed Kubernetes | Kubernetes control plane managed by AWS |
| Fargate | Serverless containers | No EC2 node management |
| App Runner | Simple container/web app hosting | Less infra control |

### 4.2 Compute Decision Table

| Requirement | Choose |
|---|---|
| Need full OS control | EC2 |
| Existing Docker app, simple AWS orchestration | ECS |
| Kubernetes ecosystem required | EKS |
| No server/node management for containers | Fargate |
| Event-driven background task | Lambda |
| Simple web service deployment | App Runner / ECS Fargate |

### 4.3 EC2 Essentials

| Concept | Meaning |
|---|---|
| AMI | Machine image |
| Instance type | CPU/memory/network shape |
| Security group | Instance-level virtual firewall |
| Key pair | SSH access credential |
| User data | Startup script |
| EBS volume | Persistent block disk |
| Elastic IP | Static public IP |
| Placement group | Control instance placement for latency/HA |

### 4.4 Lambda Essentials

| Concept | Meaning |
|---|---|
| Handler | Function entry point |
| Trigger | Event source: API Gateway, S3, SQS, EventBridge |
| Cold start | Startup delay for new execution environment |
| Timeout | Max execution duration |
| Memory setting | Also controls CPU allocation |
| Concurrency | Parallel executions |
| DLQ / destination | Failed async event handling |

## 5. Networking

### 5.1 VPC Building Blocks

| Component | Meaning |
|---|---|
| VPC | Isolated virtual network |
| CIDR | IP range, e.g. `10.0.0.0/16` |
| Public subnet | Route to internet gateway |
| Private subnet | No direct inbound internet route |
| Route table | Routing rules for subnet |
| Internet Gateway | Public internet access for VPC |
| NAT Gateway | Private subnet outbound internet |
| Security Group | Stateful instance/service firewall |
| NACL | Stateless subnet firewall |
| VPC Endpoint | Private access to AWS service |

### 5.2 Load Balancers

| Type | Layer | Use Case |
|---|---|---|
| ALB | L7 HTTP/HTTPS | Web apps, path/host routing |
| NLB | L4 TCP/UDP/TLS | Very high throughput, static IP, low latency |
| Gateway LB | L3/L4 appliance routing | Firewalls/inspection appliances |

### 5.3 DNS / CDN

| Service | Use |
|---|---|
| Route 53 | DNS, health checks, routing policies |
| CloudFront | CDN edge cache |
| ACM | TLS certificates |
| WAF | Layer 7 web protection |
| Shield | DDoS protection |

### 5.4 Common Network Layout

| Layer | Placement |
|---|---|
| ALB | Public subnets |
| App servers/tasks | Private subnets |
| Database | Private subnets |
| NAT Gateway | Public subnet for private outbound access |
| Bastion / SSM | Admin access path |

## 6. IAM And Security

### 6.1 IAM Basics

| Term | Meaning |
|---|---|
| User | Long-term human/service identity; avoid for apps when possible |
| Group | Collection of users |
| Role | Assumable identity with temporary credentials |
| Policy | JSON permission document |
| Principal | Who gets access |
| Action | What operation is allowed |
| Resource | Which AWS resource |
| Condition | When/how access applies |

### 6.2 Security Cheat Sheet

| Need | AWS Feature |
|---|---|
| Least privilege | IAM scoped policies |
| App credentials on AWS | IAM roles, not hardcoded keys |
| Secret storage | Secrets Manager / SSM Parameter Store |
| Encryption keys | KMS |
| TLS certificates | ACM |
| Audit API calls | CloudTrail |
| Config drift/compliance | AWS Config |
| Network isolation | VPC + private subnets + security groups |
| Web attack protection | WAF |

## 7. Databases And Caching

### 7.1 Database Services

| Service | Type | Use Case |
|---|---|---|
| RDS | Managed relational DB | PostgreSQL/MySQL/MariaDB/SQL Server/Oracle |
| Aurora | Cloud-native relational DB | High availability relational workloads |
| DynamoDB | NoSQL key-value/document | Low-latency high-scale access |
| ElastiCache Redis | In-memory cache | Cache, sessions, locks, rate limits |
| Redshift | Data warehouse | OLAP/analytics |
| OpenSearch | Search/analytics | Full-text search, logs |
| Neptune | Graph DB | Relationship-heavy data |

### 7.2 RDS / Aurora Essentials

| Feature | Use |
|---|---|
| Multi-AZ | High availability failover |
| Read replica | Read scaling / reporting |
| Backup | Point-in-time recovery |
| Parameter group | DB engine settings |
| Subnet group | Private DB placement |
| Security group | DB network access |
| Connection pool | Avoid too many DB connections |

### 7.3 DynamoDB Essentials

| Concept | Meaning |
|---|---|
| Partition key | Primary distribution key |
| Sort key | Optional range key inside partition |
| GSI | Global secondary index |
| LSI | Local secondary index |
| RCU/WCU | Read/write capacity units |
| On-demand | Pay per request capacity mode |
| TTL | Automatic item expiry |
| Streams | Change events from table |
| Hot partition | Bad key distribution bottleneck |

### 7.4 Cache Patterns

| Pattern | AWS Service |
|---|---|
| Cache-aside | App + ElastiCache Redis |
| Session store | ElastiCache Redis / DynamoDB |
| Rate limiting | Redis counters / API Gateway throttling |
| CDN cache | CloudFront |
| DB read scaling | Read replicas + cache |

## 8. Messaging And Eventing

### 8.1 Messaging Services

| Service | Type | Use Case |
|---|---|---|
| SQS Standard | Queue | Decouple services, high throughput, at-least-once |
| SQS FIFO | Ordered queue | Ordered/deduplicated processing |
| SNS | Pub-sub fanout | Push event to many subscribers |
| EventBridge | Event bus | SaaS/app events, routing rules |
| Kinesis | Stream | Real-time ordered event streams |
| MSK | Managed Kafka | Kafka workloads |

### 8.2 Queue Concepts

| Concept | Meaning |
|---|---|
| Visibility timeout | Time message hidden after consumer receives it |
| DLQ | Stores failed messages after retries |
| At-least-once | Consumer may see duplicate messages |
| Idempotency | Safe duplicate processing |
| Back pressure | Queue absorbs producer spikes |
| Fanout | One event to many consumers |

## 9. Scaling, Availability, Reliability

### 9.1 Scaling Techniques

| Problem | AWS Technique |
|---|---|
| More web traffic | ALB + Auto Scaling Group / ECS service scaling |
| More reads | CloudFront, ElastiCache, RDS read replicas |
| More writes | Queue buffering, sharding, DynamoDB partition design |
| Spiky workload | SQS + workers / Lambda |
| Global latency | CloudFront, Route 53 latency routing, multi-region |
| DB bottleneck | Indexing, read replicas, cache, partitioning |

### 9.2 Availability Patterns

| Pattern | AWS Pieces |
|---|---|
| Multi-AZ app | ALB + app instances/tasks in multiple AZs |
| Multi-AZ DB | RDS/Aurora Multi-AZ |
| Backup/restore | S3 backups, RDS snapshots |
| Warm standby | Smaller running stack in second region |
| Active-active | Multi-region live traffic |
| Graceful degradation | Cache, queue, fallback response |

### 9.3 DR Metrics

| Metric | Meaning |
|---|---|
| RTO | Max acceptable recovery time |
| RPO | Max acceptable data loss window |
| Backup/restore | Cheapest, slowest DR |
| Pilot light | Minimal core infra always running |
| Warm standby | Scaled-down full environment |
| Active-active | Fastest, most expensive |

## 10. Observability And Operations

### 10.1 Observability Services

| Service | Use |
|---|---|
| CloudWatch Metrics | Metrics and alarms |
| CloudWatch Logs | Application/system logs |
| CloudWatch Alarms | Alert on thresholds |
| X-Ray | Distributed tracing |
| CloudTrail | AWS API audit log |
| AWS Config | Resource config history/compliance |
| VPC Flow Logs | Network traffic metadata |

### 10.2 Operational Checks

| Symptom | Check |
|---|---|
| High latency | ALB target response time, app metrics, DB metrics |
| 5xx errors | ALB logs, app logs, deployment changes |
| DB slow | CPU, connections, locks, slow query log, indexes |
| Queue growing | Consumer errors, worker count, downstream bottleneck |
| Lambda errors | Timeout, memory, concurrency throttle, DLQ |
| S3 access denied | IAM policy, bucket policy, block public access, KMS policy |
| EC2 unreachable | Security group, NACL, route table, instance health |

## 11. Common SDE2 Design Choices

### 11.1 Service Selection

| Requirement | AWS Choice |
|---|---|
| Public REST API | ALB/API Gateway + ECS/Lambda |
| Background jobs | SQS + ECS/Lambda workers |
| Static frontend/assets | S3 + CloudFront |
| File uploads | Pre-signed S3 URLs |
| Relational transactional data | RDS/Aurora PostgreSQL/MySQL |
| High-scale key-value access | DynamoDB |
| Cache | ElastiCache Redis |
| Full-text search | OpenSearch |
| Analytics | S3 data lake + Athena / Redshift |

### 11.2 Common Architectures

| System | AWS Building Blocks |
|---|---|
| Web app | Route 53 + CloudFront/ALB + ECS/EC2 + RDS + ElastiCache |
| Serverless API | API Gateway + Lambda + DynamoDB/RDS Proxy + CloudWatch |
| Async pipeline | Producer + SQS/SNS/EventBridge + workers + DLQ |
| Media upload | Client pre-signed URL + S3 + Lambda processor + CloudFront |
| Data lake | S3 + Glue + Athena + Redshift |
| High-read service | ALB + app + Redis + read replicas + CloudFront where applicable |

### 11.3 Interview Keywords

| Topic | Must Mention |
|---|---|
| Security | IAM least privilege, private subnets, SGs, encryption, secrets manager |
| Availability | Multi-AZ, health checks, autoscaling, backups |
| Scalability | Load balancer, cache, queue, replicas, partitioning/sharding |
| Reliability | Retry with backoff, DLQ, idempotency, circuit breaker |
| Cost | Right sizing, storage classes, autoscaling, lifecycle policies |
| Observability | Metrics, logs, traces, alarms, audit logs |
