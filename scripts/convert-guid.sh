#!/usr/bin/env bash
set -euo pipefail

if (( $# > 0 )); then
    input="$*"
else
    input="$(cat)"
fi

mapfile -t values < <(
    printf '%s' "$input" |
        grep -oE '0[xX][0-9a-fA-F]+'
)

if (( ${#values[@]} != 11 )); then
    printf 'Error: expected 11 hexadecimal values, found %d\n' \
        "${#values[@]}" >&2
    exit 1
fi

printf '%08x-%04x-%04x-%02x%02x-%02x%02x%02x%02x%02x%02x\n' \
    "$((values[0]))" \
    "$((values[1]))" \
    "$((values[2]))" \
    "$((values[3]))" \
    "$((values[4]))" \
    "$((values[5]))" \
    "$((values[6]))" \
    "$((values[7]))" \
    "$((values[8]))" \
    "$((values[9]))" \
    "$((values[10]))"
