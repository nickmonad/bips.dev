#!env bash
set -e

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

find bips -type f -name 'bip*.mediawiki' -maxdepth 1 \
    | cargo run --release

find bips -type d -name 'bip-*' -maxdepth 1 \
    | xargs -I{} bash -c 'move_static "{}"'
