#!/bin/bash

git config --global --add safe.directory $PWD

make bips
make build
