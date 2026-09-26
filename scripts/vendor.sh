#!/usr/bin/env bash
set -euo pipefail

# Refreshes the third-party browser assets under public/vendor and assets/fonts from npm
# tarballs verified against the integrity hashes pinned below.

THREE_VERSION="0.186.1"
THREE_INTEGRITY="sha512-blFeqb49wRCSGUGj7gtpfnSGHy2lwDk94RhUmS1c/hTby70kvChbWpkJ4Pm1390LqzzvTmzgXKHPEafJwCb8jA=="
BRICOLAGE_VERSION="5.3.0"
BRICOLAGE_INTEGRITY="sha512-TLi9Q4hJjS2UvoTMRSS2nHu6c4R56lAw60NR9QYtVRCHn0XtsFpiEhNffZ8Glsoxu6wEEwLKBP8lb94J52PNBA=="
JETBRAINS_VERSION="5.3.0"
JETBRAINS_INTEGRITY="sha512-F32xpS2NsGYoQi2ADSkKTgpJj7ozajsGgDJ8woTnqjmIB+dxDIqImjl4pXZVEExu8UFZ2ndhmX18EBS/hdz3Lw=="

tmp="$(mktemp -d)"
trap 'rm -rf "${tmp}"' EXIT

fetch() {
    local name="$1" url="$2" integrity="$3" out="${tmp}/$1.tgz"
    curl --proto '=https' --tlsv1.2 --fail --location --silent --show-error -o "${out}" "${url}"
    local actual="sha512-$(openssl dgst -sha512 -binary "${out}" | base64 | tr -d '\n')"
    if [[ "${actual}" != "${integrity}" ]]; then
        echo "integrity mismatch for ${name}: expected '${integrity}', got '${actual}'" >&2
        exit 1
    fi
    mkdir -p "${tmp}/${name}"
    tar -xzf "${out}" -C "${tmp}/${name}"
}

fetch three "https://registry.npmjs.org/three/-/three-${THREE_VERSION}.tgz" "${THREE_INTEGRITY}"
fetch bricolage "https://registry.npmjs.org/@fontsource-variable/bricolage-grotesque/-/bricolage-grotesque-${BRICOLAGE_VERSION}.tgz" "${BRICOLAGE_INTEGRITY}"
fetch jetbrains "https://registry.npmjs.org/@fontsource-variable/jetbrains-mono/-/jetbrains-mono-${JETBRAINS_VERSION}.tgz" "${JETBRAINS_INTEGRITY}"

three_src="${tmp}/three/package"
three_dst="public/vendor/three-${THREE_VERSION}"
rm -rf public/vendor/three-*
mkdir -p "${three_dst}/addons/loaders" "${three_dst}/addons/utils"
cp "${three_src}/build/three.module.js" "${three_src}/build/three.core.js" "${three_src}/LICENSE" "${three_dst}/"
cp "${three_src}/examples/jsm/loaders/GLTFLoader.js" "${three_dst}/addons/loaders/"
cp "${three_src}/examples/jsm/utils/BufferGeometryUtils.js" "${three_src}/examples/jsm/utils/SkeletonUtils.js" "${three_dst}/addons/utils/"

mkdir -p assets/fonts
cp "${tmp}/bricolage/package/files/bricolage-grotesque-latin-wght-normal.woff2" assets/fonts/
cp "${tmp}/bricolage/package/LICENSE" assets/fonts/LICENSE-BricolageGrotesque.txt
cp "${tmp}/jetbrains/package/files/jetbrains-mono-latin-wght-normal.woff2" assets/fonts/
cp "${tmp}/jetbrains/package/LICENSE" assets/fonts/LICENSE-JetBrainsMono.txt

echo "vendored three ${THREE_VERSION}, bricolage-grotesque ${BRICOLAGE_VERSION}, jetbrains-mono ${JETBRAINS_VERSION}"
