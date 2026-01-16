#!/usr/bin/env bash

find bips -maxdepth 1 -type f -name 'bip-*' \
    | cargo run -- generate
