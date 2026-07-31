docker compose up -d

docker exec -it kafka-redpanda rpk topic create orders

docker exec -it kafka-redpanda rpk topic list

cd rust
cargo run

curl -s localhost:8080/health

curl -s -X POST localhost:8080/orders -H "Content-Type: application/json" -d '{"user_id":"user_1","amount":1299}'

curl -s localhost:8080/orders/<order_id>
