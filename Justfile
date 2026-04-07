default: install build

nix:
    nix-shell --run fish

install-tailwind:
    curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.1/tailwindcss-linux-x64
    chmod +x tailwindcss*
    mkdir -p bin
    mv tailwindcss* bin/tailwindcss

install-pagefind:
    cargo install pagefind@1.4.0

install: install-tailwind install-pagefind

bips:
    git submodule init && git submodule update --recursive --remote

tailwind:
    ./bin/tailwindcss -c web/tailwind.config.js -i web/static/style.tailwind.css -o web/static/style.css --minify

pagefind:
    pagefind --site web/public

static:
    ./scripts/static.sh

generate:
    find bips -maxdepth 1 -type f -name 'bip-*' \
        | cargo run -- generate

[working-directory: 'web']
zola:
    zola build

build: bips tailwind static generate zola pagefind
