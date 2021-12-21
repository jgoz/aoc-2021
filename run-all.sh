#!/usr/bin/env bash

cargo build --release
ls -a | grep ^day | while read day; do
    echo "Running $day"
    cargo run --release --bin $day 1 < $day/input
    cargo run --release --bin $day 2 < $day/input
done
