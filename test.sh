#!/bin/bash
cargo watch -x "run -- -o -t indev 797979 903030 'yo man how goes' -i $EUID -c '󰣙' -s f04181 07102e ' $USER' -g '$(git status --porcelain=v2 --branch 2>&1)' -s f76c59 07102e '$(date +%H:%M:%M)' -s faaa5a 07102e '$(pwd)'"

