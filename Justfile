default:
    just --list

# remove all BIP content directories, ignoring top level 'web/content' itself
clean:
    find web/content -maxdepth 1 -type d | grep --invert-match '^web/content$' | xargs -I{} rm -rf {}
