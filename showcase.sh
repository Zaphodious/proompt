#!/bin/bash

cargo build --release

./target/release/proompt -o -t showcase 797979 903030 \
    -i "$EUID" \
    -w $(tput cols) \
    -g "$(git status --porcelain=v2 --branch 2>&1)" \
    --git-s committed 4422ff 000000 \
    --git-s staged 4499ff 000000 \
    --git-s unstaged ff3399 000000 \
    -s fa7a5a 000000 "@bistromath" \
    -s faaa5a 07102e "$(pwd)" \
    --break \
    -s f76c59 07102e "$(date +%H:%M:%M)" \
    -s f04181 07102e "$USER" \

echo 
