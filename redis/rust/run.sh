#!/usr/bin/env bash
set -euo pipefail

export REDIS_URL="${REDIS_URL:-redis://127.0.0.1/}"
cargo run
