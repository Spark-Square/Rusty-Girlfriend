#!/usr/bin/env bash
set -e

# Step 1: Build frontend
echo "🔨 Building Yew frontend with Trunk..."
cd frontend
trunk build 
cd ..

# Step 2: Start Rocket backend
echo "🚀 Starting Rocket backend..."
cd backend 
cargo run &
PID=$!

sleep 2
firefox "http://127.0.0.1:8000/" &

echo "🚀 Rocket started with PID $PID"
echo "Press ENTER to stop..."

read

kill $PID
echo "🛑 Rocket stopped."
