#!/bin/bash
set -euo pipefail
IFS=$'\n\t'
cd "$(dirname "$0")"/..

# Run code generators.
#
# USAGE:
#    ./tools/gen.sh

cargo run --manifest-path tools/codegen/Cargo.toml
