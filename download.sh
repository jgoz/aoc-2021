#!/usr/bin/env bash

DIR=day$(printf "%02d" $1)

echo "Downloading day $1 input into $DIR/input"

$HOME/go/bin/aocdl -day $1 -output "$DIR/input" ${@:2}