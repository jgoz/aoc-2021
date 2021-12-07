#!/usr/bin/env bash

DAY_DD=day$(printf "%02d" $1)
DAY_D=day$1

mkdir -p $DAY_DD/src
sed "s/dayDD/${DAY_DD}/g" ./.template/Cargo.toml > $DAY_DD/Cargo.toml
sed -e "s/dayDD/${DAY_DD}/g" -e "s/dayD/${DAY_D}/g" ./.template/src/main.rs > $DAY_DD/src/main.rs
sed -i.bak "s/^]/    \"${DAY_DD}\",\\n]/" ./Cargo.toml
rm ./Cargo.toml.bak
