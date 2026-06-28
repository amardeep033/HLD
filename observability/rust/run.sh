#!/bin/bash
set -e

fuser -k 8080/tcp 2>/dev/null

cargo build --release

rm log.txt
rm otel.log

./target/release/obsv > otel.log 2>&1 &