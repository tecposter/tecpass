#!/usr/bin/env bash
# pass: the standard unix password manager
# https://www.passwordstore.org/
# export passwords to external file

shopt -s nullglob globstar
prefix=${PASSWORD_STORE_DIR:-$HOME/.password-store}

for file in "$prefix"/**/*.gpg; do                           
    file="${file/$prefix//}"
    # printf "%s\n" "Name: ${file%.*}"  >> exported_passes
    # printf "${file%.*}"  >> exported_passes
    printf "%s\n" "Name: ${file%.*}" >> exported_passes
    pass "${file%.*}" >> exported_passes
    printf "\n\n" >> exported_passes
done
