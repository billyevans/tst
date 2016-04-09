#!/bin/bash

set -e

cargo login $CARGO_TOKEN
cargo package
cargo publish
