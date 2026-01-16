#!/usr/bin/env bash

./bin/tailwindcss -c web/tailwind.config.js -i web/static/style.tailwind.css -o web/static/style.css --minify
