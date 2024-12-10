#!/usr/bin/env bash

files=(
    Cargo.toml
    Makefile
    src/helpers.rs
    src/output.rs
    update-template.sh
)

for file in "${files[@]}" ; do
    cp --verbose "${file}" ../../template/"${file}"
done
