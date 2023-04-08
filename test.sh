#!/bin/bash
cargo watch -x "run -- \
    -o -t indev 797979 903030 'yo man how goes' \
    -i $EUID -c '󰣙' \
    -g '$(git status --porcelain=v2 --branch 2>&1)' \
    --git-s all ffffff 000000 ' Git! @e @i '\
    --git-s committed 4422ff 000000 \
    --git-s staged 4499ff 000000 \
    --git-s unstaged ff3399 000000 \
    -s f04181 07102e ' $USER' \
    -s f76c59 07102e '$(date +%H:%M:%M)' \
    -s faaa5a 07102e '$(pwd)'"

