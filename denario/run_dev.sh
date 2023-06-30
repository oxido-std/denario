#!/bin/bash
docker compose up -d
cargo watch -q -c -w src/ -x run