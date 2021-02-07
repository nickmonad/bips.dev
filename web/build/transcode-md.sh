#!env bash
set -e

transcode() {
    path=$1
    base=$(basename $path)

    # isolate bip number from basename
    if [[ $base =~ ^bip-([0-9]+)\.mediawiki$ ]]
    then
        # trim leading zeros
        bip="$(echo -e "${BASH_REMATCH[1]}" | sed -e 's/^[0]*//')"
        echo "processing $path -> $bip"
        pandoc $path -f mediawiki -t commonmark_x -s -o web/content/$bip.md.pre
    else
        # log non-matching basename
        echo "$path does not match!" >&2
    fi

    # apply page data to markdown
    cargo run $path | cat - web/content/$bip.md.pre > web/content/$bip.md
    rm web/content/$bip.md.pre
}

export -f transcode

find bips -type f -name 'bip*.mediawiki' -maxdepth 1 \
    | xargs -I{} bash -c 'transcode "{}"'
