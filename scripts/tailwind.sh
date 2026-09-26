#!/usr/bin/env bash
set -euo pipefail

# Wrapper around the standalone Tailwind CSS v4 binary so the project
# can build its CSS without a Node toolchain. The binary is fetched on
# first use, verified against the release checksums and cached per version.

VERSION="${TAILWIND_VERSION:-v4.3.3}"
BIN_DIR="bin"
BIN="${BIN_DIR}/tailwindcss-${VERSION}"

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

sha256() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$1" | cut -d' ' -f1
    else
        shasum -a 256 "$1" | cut -d' ' -f1
    fi
}

asset="tailwindcss-${os}-${arch}"
base="https://github.com/tailwindlabs/tailwindcss/releases/download/${VERSION}"

if [[ ! -x "${BIN}" ]]; then
    mkdir -p "${BIN_DIR}"
    tmp="$(mktemp "${BIN_DIR}/.tailwindcss.XXXXXX")"
    trap 'rm -f "${tmp}"' EXIT

    echo "fetching ${asset} ${VERSION}" >&2
    curl --proto '=https' --tlsv1.2 --fail --location --silent --show-error -o "${tmp}" "${base}/${asset}"

    expected="$(curl --proto '=https' --tlsv1.2 --fail --location --silent --show-error "${base}/sha256sums.txt" \
        | awk -v f="./${asset}" '$2 == f { print $1 }')"
    actual="$(sha256 "${tmp}")"
    if [[ -z "${expected}" || "${expected}" != "${actual}" ]]; then
        echo "checksum mismatch for ${asset} ${VERSION}: expected '${expected}', got '${actual}'" >&2
        exit 1
    fi

    chmod +x "${tmp}"
    mv "${tmp}" "${BIN}"
    trap - EXIT
fi

exec "${BIN}" "$@"
