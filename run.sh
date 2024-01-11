#!/bin/bash

cargo build --release

./target/release/server &
PID=$!
echo "Server running as pid $PID"

sleep 2

echo ""
echo "Hyper<1.0"
echo "Using regular route"
./target/release/client0x http://127.0.0.1:3000/regular

echo "Using stream route"
./target/release/client0x http://127.0.0.1:3000/stream

echo ""
echo "Hyper>=1.0"
echo "Using regular route"
./target/release/client1x http://127.0.0.1:3000/regular

echo "Using stream route"
./target/release/client1x http://127.0.0.1:3000/stream

kill $PID
