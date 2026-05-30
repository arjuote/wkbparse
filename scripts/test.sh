#!/bin/bash -l
set -e
if [ "$#" -eq 0 ]; then
    cargo test --no-default-features
    cargo build --no-default-features -F proj
    PROJ_DATA=$(find target -path "*/build/proj-sys-*/out/share/proj" -type d 2>/dev/null | head -1)
    PROJ_DATA=$PROJ_DATA cargo test --no-default-features -F proj
    tox
else
  exec "$@"
fi
