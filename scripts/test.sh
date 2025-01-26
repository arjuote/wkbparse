#!/bin/bash -l
set -e
if [ "$#" -eq 0 ]; then
    cargo test --no-default-features
    cargo test --no-default-features -F proj
    tox
else
  exec "$@"
fi
