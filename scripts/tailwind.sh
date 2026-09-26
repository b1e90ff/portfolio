#!/usr/bin/env bash
set -euo pipefail

# Wrapper around the standalone Tailwind CSS v4 binary so the project
# can build its CSS without a Node toolchain. The binary is fetched on
# first use, verified against a checksum pinned in this file and cached per version.

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

pinned_sha256() {
    case "${VERSION}/$1" in
        v4.3.3/tailwindcss-linux-arm64) echo 55fd0b241214eff3de1e8ee4f22796662f2d2e7a49bcfca7477cfd0bac398195 ;;
        v4.3.3/tailwindcss-linux-x64)   echo dc61b3ac6b8c9ca874c0cc4c57b2409791a64c5540404ca5f5367360babc313a ;;
        v4.3.3/tailwindcss-macos-arm64) echo cdf646702987a743464dff4d9c60fd4480d1c1e73dd819a9a67f1078815dce9d ;;
        v4.3.3/tailwindcss-macos-x64)   echo 7922e0953f2110c05976e3bf58f14e643d90427575e766b7d433f5f80cbee7e1 ;;
    esac
}

asset="tailwindcss-${os}-${arch}"
url="https://github.com/tailwindlabs/tailwindcss/releases/download/${VERSION}/${asset}"

if [[ ! -x "${BIN}" ]]; then
    expected="${TAILWIND_SHA256:-$(pinned_sha256 "${asset}")}"
    if [[ -z "${expected}" ]]; then
        echo "no pinned checksum for ${asset} ${VERSION}; set TAILWIND_SHA256" >&2
        exit 1
    fi

    mkdir -p "${BIN_DIR}"
    tmp="$(mktemp "${BIN_DIR}/.tailwindcss.XXXXXX")"
    trap 'rm -f "${tmp}"' EXIT

    echo "fetching ${asset} ${VERSION}" >&2
    curl --proto '=https' --tlsv1.2 --fail --location --silent --show-error -o "${tmp}" "${url}"

    actual="$(sha256 "${tmp}")"
    if [[ "${expected}" != "${actual}" ]]; then
        echo "checksum mismatch for ${asset} ${VERSION}: expected '${expected}', got '${actual}'" >&2
        exit 1
    fi

    chmod +x "${tmp}"
    mv "${tmp}" "${BIN}"
    trap - EXIT
fi

exec "${BIN}" "$@"
