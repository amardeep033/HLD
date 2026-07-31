# Curl Commands

Health:

```bash
curl -s localhost:8080/health
```

Create a short URL:

```bash
curl -s -X POST localhost:8080/shorten \
  -H "Content-Type: application/json" \
  -d '{"long_url":"https://www.rust-lang.org/"}'
```

Resolve a short URL:

```bash
curl -s localhost:8080/r/<code>
```

Call it twice. The first response should say:

```json
"source":"database"
```

The second response should say:

```json
"source":"redis-cache"
```

Test rate limiting:

```bash
for i in 1 2 3 4 5 6; do
  curl -si localhost:8080/limited/user-123 | head -n 8
  echo
done
```

Inspect Redis keys:

```bash
redis-cli KEYS '*'
redis-cli TTL cache:url:<code>
redis-cli GET cache:url:<code>
redis-cli GET counter:redirects:<code>
redis-cli GET rate:user:user-123:fixed-window
```
