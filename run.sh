#!/bin/bash

cargo build --release
cd target/release

./server &
PID=$!
echo "Server running as pid $PID"

sleep 2

echo ""
echo "Reqwest (hyper<1)"
echo "Using regular route"
./reqwest_client http://127.0.0.1:3000/regular

echo "Using stream route"
./reqwest_client http://127.0.0.1:3000/stream

echo ""
echo "Hyper (hyper>=1)"
echo "Using regular route"
./hyper1_client http://127.0.0.1:3000/regular

echo "Using stream route"
./hyper1_client http://127.0.0.1:3000/stream

kill $PID
