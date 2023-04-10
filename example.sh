#!/bin/bash

# Set some variables with PS2 control codes
WDIR="\w"
THETIME="\t"
USTRING="\u@\h"

# Create a function for PROMPT_COMMAND to use
function proomptme {
    # In Bash 4.4 and up, PS1 control codes are expandable with @P. Passing
    # them expanded into proompt will let it properly calculate the size of the text.
    PS1="$(proompt -i $EUID \
    -g "$(git status --porcelain=v2 --branch 2>&1)" \
    -s f76c59 07102e "${THETIME@P}" \
    -s f04181 07102e " ${USTRING@P}" \
    -s faaa5a 07102e "${WDIR@P}" \
    --git-s committed 4422ff 000000 \
    --git-s staged 4499ff 000000 \
    --git-s unstaged ff3399 000000 \
    )"
}

# Set the command that will be ran when a prompt is needed. This is
# better then embedding a call in a PS2 string, as it
# lets the prompt engine properly handle some of the extra control
# codes we need to use in order to get accurate text wrapping.
PROMPT_COMMAND=proomptme
