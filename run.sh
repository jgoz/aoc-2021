#!/usr/bin/env bash

DAY=day$(printf "%02d" $1)
PART="${2:-1}"

cargo run ${@:3} --bin $DAY $PART < $DAY/input
