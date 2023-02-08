#!env bash
set -e
rustup default stable

bip_number() {
    if [[ $1 =~ ^bip-([0-9]+).*$ ]]
    then
        echo "$(echo -e "${BASH_REMATCH[1]}" | sed -e 's/^[0]*//')"
    else
        echo "$1 did not match pattern!"
        exit 1;
    fi
}

move_static() {
    path=$1
    base=$(basename $path)
    bip=$(bip_number $base)

    # create a directory to co-locate static assets for a BIP page.
    # note, we need a dummy index.md to force zola to create a "servable" directory for these assets.
    mkdir -p web/content/$bip/$base
    echo -e "+++\n+++" > web/content/$bip/$base/index.md
    cp -R $path/* web/content/$bip/$base/
}

export -f bip_number
export -f move_static

find bips -maxdepth 1 -type f -name 'bip*.mediawiki' \
    | cargo run --release

find bips -maxdepth 1 -type d -name 'bip-*' \
    | xargs -I{} bash -c 'move_static "{}"'

# replace every bip.mediawiki link with the absolute path equivalent
# stored in a `.md.rg` file
find web/content -type f -name 'index.md' \
    | xargs -I{} bash -c "rg --passthru '\(bip-[0]+(\d+).mediawiki.+?\)' -r '(/\$1)' {} > {}.rg && mv {}.rg {} && rm {}.rg"
