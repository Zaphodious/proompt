#!/bin/bash
cargo watch -x "run -- \
    -o -t indev 797979 903030 'yo man how goes' \
    -w $(tput cols) \
    -g '$(git status --porcelain=v2 --branch 2>&1)' \
    --seperators ' ' ' ' \
    --git-s committed 4422ff 000000 \
    --git-s staged 4499ff 000000 \
    --git-s unstaged ff3399 000000 \
     --git-s all ffffff 000000 ' Git! @e @i '\
    --break \
    -s faaa5a 07102e '$(pwd)' \
    -s f76c59 07102e '$(date +%H:%M:%M)' \
    -s f04181 07102e ' $USER' \
    "
# -i $EUID -c '󰣙' \
