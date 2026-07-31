#!/bin/bash
set -e

docker compose -f ../docker-compose.yml up -d
docker exec kafka-redpanda rpk topic create orders 2>/dev/null || true

KAFKA_BOOTSTRAP_SERVERS=127.0.0.1:9092 cargo run --features kafka
