#!/bin/bash

cargo build --release

./target/release/server &
PID=$!
echo "Server running as pid $PID"

ENDPOINT_BASE="http://127.0.0.1:3000"
# ENDPOINT_BASE="http://host.docker.internal:3000"
sleep 2

echo ""
echo "Using ENDPOINT_BASE: $ENDPOINT_BASE"

echo ""
echo "Hyper<1.0"
echo "Using regular route"
./target/release/client0x "${ENDPOINT_BASE}/regular"

echo "Using stream route"
./target/release/client0x "${ENDPOINT_BASE}/stream"

echo ""
echo "Hyper>=1.0"
echo "Using regular route"
./target/release/client1x "${ENDPOINT_BASE}/regular"

echo "Using stream route"
./target/release/client1x "${ENDPOINT_BASE}/stream"

kill $PID
