# CI/CD Pipeline Stages

- Continuous Integration (CI) focuses on the early stages of development, while continuous delivery/deployment (CD) encompasses the later stages of releasing and operating the application. Each stage has specific tools that facilitate the respective tasks and processes.

- Continuous Delivery (CD) ensures that code changes are automatically built, tested, and prepared for release to production. Continuous Deployment (CD) takes this a step further by automatically deploying every change that passes the tests to production without manual intervention.

- Continuous Deployment is a subset of continuous delivery, where every change that passes automated tests is automatically deployed to production. Continuous delivery ensures that code changes are automatically built, tested, and prepared for release, but the actual deployment to production may require manual approval.

| Stage | Phase | Purpose |
|-------|-------|---------|
| **1. Plan** | CI | Define requirements, tasks, and documentation before coding |
| **2. Code** | CI | Write and version-control source code |
| **3. Build** | CI | Compile, package, and bundle the application |
| **4. Test** | CI | Validate functionality through automated tests |
| **5. Release** | CI/CD | Automate the pipeline trigger and artifact creation |
| **6. Deploy** | CD | Package and deliver the application to target environments |
| **7. Operate** | CD | Run, configure, and scale the application in production |
| **8. Monitor** | CD | Track performance, errors, and system health |

# CI/CD Pipeline Tools Reference

| # | Stage | Phase | Tool Name | Category | Description |
|---|-------|-------|-----------|----------|-------------|
| **1** | **Plan** | CI | | **Project Management** | |
| 1.1 | Plan | CI | Jira | Project Management | Issue tracking, sprint planning, and backlog management |
| 1.2 | Plan | CI | Confluence | Documentation | Team wiki and collaborative documentation platform |
| **2** | **Code** | CI | | **Source Control** | |
| 2.1 | Code | CI | GitHub | Source Control | Cloud-based Git repository hosting and collaboration |
| 2.2 | Code | CI | GitLab | Source Control | DevOps platform with built-in CI/CD and Git repository |
| 2.3 | Code | CI | Bitbucket | Source Control | Git hosting with native Jira and Atlassian integration |
| 2.4 | Code | CI | Azure Repos | Source Control | Git repositories integrated into Azure DevOps |
| **3** | **Build** | CI | | **Build Tools** | |
| 3.1 | Build | CI | Gradle | Build Tool | Flexible build automation for JVM-based projects |
| 3.2 | Build | CI | Bazel | Build Tool | Fast, multi-language build system by Google |
| 3.3 | Build | CI | webpack | Build Tool | JavaScript module bundler for web applications |
| 3.4 | Build | CI | Maven | Build Tool | Java project build and dependency management tool |
| **4** | **Test** | CI | | **Testing Frameworks** | |
| 4.1 | Test | CI | Jest | Unit Testing | JavaScript unit and integration testing framework |
| 4.2 | Test | CI | Playwright | E2E Testing | End-to-end browser automation and testing by Microsoft |
| 4.3 | Test | CI | JUnit | Unit Testing | Standard unit testing framework for Java |
| 4.4 | Test | CI | Selenium | E2E Testing | Browser automation framework for web app testing |
| **5** | **Release** | CI/CD | | **CI/CD Automation** | |
| 5.1 | Release | CI/CD | Jenkins | CI/CD Automation | Open-source automation server for building pipelines |
| 5.2 | Release | CI/CD | GitHub Actions | CI/CD Automation | Native CI/CD workflows integrated into GitHub |
| 5.3 | Release | CI/CD | GitLab CI | CI/CD Automation | Built-in CI/CD pipelines in GitLab |
| 5.4 | Release | CI/CD | JFrog Artifactory | Artifact Registry | Universal artifact repository and binary management |
| **6** | **Deploy** | CD | | **Deployment & Containerization** | |
| 6.1 | Deploy | CD | Docker | Containerization | Platform for packaging apps into portable containers |
| 6.2 | Deploy | CD | Argo CD | GitOps / CD | Kubernetes-native GitOps continuous delivery tool |
| 6.3 | Deploy | CD | Helm | Package Manager | Kubernetes application package manager (charts) |
| **7** | **Operate** | CD | | **Infrastructure & Orchestration** | |
| 7.1 | Operate | CD | Kubernetes | Container Orchestration | Automated deployment, scaling, and management of containers |
| 7.2 | Operate | CD | AWS Lambda | Serverless | Serverless compute service on Amazon Web Services |
| 7.3 | Operate | CD | Terraform | Infrastructure as Code | Declarative tool for provisioning cloud infrastructure |
| 7.4 | Operate | CD | Ansible | Configuration Management | Agentless automation for server configuration and deployments |
| 7.5 | Operate | CD | Puppet | Configuration Management | Infrastructure automation and configuration enforcement |
| 7.6 | Operate | CD | Chef | Configuration Management | Ruby-based infrastructure automation framework |
| **8** | **Monitor** | CD | | **Monitoring & Observability** | |
| 8.1 | Monitor | CD | Datadog | Monitoring & Observability | Full-stack monitoring, logging, and analytics platform |
| 8.2 | Monitor | CD | Prometheus | Metrics & Alerting | Open-source metrics collection and alerting system |
| 8.3 | Monitor | CD | Grafana | Visualization | Dashboard and visualization layer for metrics and logs |
| 8.4 | Monitor | CD | ELK Stack | Log Management | Elasticsearch + Logstash + Kibana for log aggregation |
| 8.5 | Monitor | CD | OpenTelemetry | Observability Framework | Vendor-neutral standard for traces, metrics, and logs |