#!/bin/bash

set -e

cat web/content/_index.md | pbcopy
rm -rf web/content
mkdir web/content
pbpaste > web/content/_index.md
