#!/bin/bash -l
set -e
if [ "$#" -eq 0 ]; then
    cargo test --no-default-features
    tox
else
  exec "$@"
fi
