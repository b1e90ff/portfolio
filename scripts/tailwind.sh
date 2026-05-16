#!/usr/bin/env bash
set -euo pipefail

# Wrapper around the standalone Tailwind CSS v4 binary so the project
# can build its CSS without a Node toolchain. The binary is fetched on
# first use and cached under bin/tailwindcss.

VERSION="${TAILWIND_VERSION:-v4.1.16}"
BIN_DIR="bin"
BIN="${BIN_DIR}/tailwindcss"

uname_s="$(uname -s)"
uname_m="$(uname -m)"

case "${uname_s}" in
    Linux)  os="linux" ;;
    Darwin) os="macos" ;;
    *) echo "unsupported os: ${uname_s}" >&2; exit 1 ;;
esac

case "${uname_m}" in
    x86_64|amd64)  arch="x64" ;;
    arm64|aarch64) arch="arm64" ;;
    *) echo "unsupported arch: ${uname_m}" >&2; exit 1 ;;
esac

asset="tailwindcss-${os}-${arch}"
url="https://github.com/tailwindlabs/tailwindcss/releases/download/${VERSION}/${asset}"

if [[ ! -x "${BIN}" ]]; then
    mkdir -p "${BIN_DIR}"
    echo "fetching ${asset} ${VERSION}" >&2
    curl --proto '=https' --tlsv1.2 --fail --location --silent --show-error \
        -o "${BIN}" "${url}"
    chmod +x "${BIN}"
fi

exec "${BIN}" "$@"
