curl -s localhost:8080/health

curl -si -X POST localhost:8080/test -H "Content-Type: application/json" -d '{"value": 9}'

curl -si -X POST localhost:8080/test -H "Content-Type: application/json" -d '{"value": 10}'

curl -si -X POST localhost:8080/test -H "Content-Type: application/json" -d '{"value": 11}'

curl -s localhost:8080/metrics