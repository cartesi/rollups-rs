#!/usr/bin/env bash
# Prepares the images the cartesi-machine tests boot: the pinned release
# kernel and rootfs, and the `yield` machine stored from them. Needs curl and
# the cartesi-machine CLI matching the emulator the bindings link.
#
# usage: scripts/prepare-test-images.sh [DIR]   (default: test-images)
set -euo pipefail

repo_root="$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
dir="${1:-${repo_root}/test-images}"
mkdir -p "$dir"
dir="$(CDPATH= cd -- "$dir" && pwd -P)"

sha256() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum -- "$1" | cut -d ' ' -f 1
    else
        shasum -a 256 -- "$1" | cut -d ' ' -f 1
    fi
}

# fetch URL SHA256 NAME: downloads URL to DIR/NAME unless it already matches.
fetch() {
    local url=$1 sha=$2 path="${dir}/$3"
    if [[ -f "$path" && "$(sha256 "$path")" == "$sha" ]]; then
        return
    fi
    curl -fsSL --retry 3 -o "${path}.tmp" "$url"
    if [[ "$(sha256 "${path}.tmp")" != "$sha" ]]; then
        rm -f -- "${path}.tmp"
        echo "error: ${url} does not match its pinned sha256" >&2
        exit 1
    fi
    mv -- "${path}.tmp" "$path"
}

fetch https://github.com/cartesi/image-kernel/releases/download/v0.21.0/linux-6.5.13-ctsi-2-v0.21.0.bin \
    5c900060da2db2bfa84cd39cd9cd722988c83c42225f3cac55f2d3157e48f32f \
    linux.bin
fetch https://github.com/cartesi/machine-emulator-tools/releases/download/v0.18.0/rootfs-tools.ext2 \
    6c159937485c99f695021c4f2ea2a57bdadcf4e4bce8e71af5bee3bb9552802e \
    rootfs.ext2

# A machine awaiting its first input, which accepts and rejects inputs in
# turn.
rm -rf -- "${dir}/yield"
(
    cd -- "$dir"
    cartesi-machine --ram-image=./linux.bin --final-hash \
        --flash-drive=label:root,data_filename:./rootfs.ext2 \
        --revert-mode=none --store=./yield \
        -- "while true; do yield manual rx-accepted; yield manual rx-rejected; done"
)
