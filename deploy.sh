#!/bin/bash

set -e
set -x

cargo login $CARGO_TOKEN
cargo package
cargo publish
