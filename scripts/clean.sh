#!/usr/bin/env bash

find web/content -maxdepth 1 -type d | grep --invert-match '^web/content$' | xargs -I{} rm -rf {}
