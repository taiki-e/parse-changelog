#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

# Run code generators.
#
# USAGE:
#    ./tools/gen.sh

cd "$(cd "$(dirname "$0")" && pwd)"/..

cargo run --manifest-path tools/codegen/Cargo.toml
