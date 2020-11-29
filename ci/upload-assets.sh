#!/bin/bash

set -euo pipefail
IFS=$'\n\t'

PACKAGE="parse-changelog"

cd "$(cd "$(dirname "${0}")" && pwd)"/..

if [[ "${GITHUB_REF:?}" != "refs/tags/"* ]]; then
  echo "GITHUB_REF should start with 'refs/tags/'"
  exit 1
fi
tag="${GITHUB_REF#refs/tags/}"

export CARGO_PROFILE_RELEASE_LTO=true
host=$(rustc -Vv | grep host | sed 's/host: //')

cargo build --bin "${PACKAGE}" --release

cd target/release
case "${OSTYPE}" in
  linux* | darwin*)
    strip "${PACKAGE}"
    asset="${PACKAGE}-${host}.tar.gz"
    tar czf ../../"${asset}" "${PACKAGE}"
    ;;
  cygwin* | msys*)
    asset="${PACKAGE}-${host}.zip"
    7z a ../../"${asset}" "${PACKAGE}".exe
    ;;
  *)
    echo "unrecognized OSTYPE: ${OSTYPE}"
    exit 1
    ;;
esac
cd ../..

if [[ -z "${GITHUB_TOKEN:-}" ]]; then
  echo "GITHUB_TOKEN not set, skipping deploy"
  exit 1
else
  gh release upload "${tag}" "${asset}" --clobber
fi
