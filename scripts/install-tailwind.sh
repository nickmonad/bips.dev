#!/usr/bin/bash

curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.1/tailwindcss-linux-x64
chmod +x tailwindcss*

mkdir bin
mv tailwindcss* bin/tailwindcss
