#!/bin/sh
set -e
KCOV_VERSION=33

wget https://github.com/SimonKagstrom/kcov/archive/v${KCOV_VERSION}.tar.gz
tar xzf v${KCOV_VERSION}.tar.gz
mkdir kcov-${KCOV_VERSION}/build
cd kcov-${KCOV_VERSION}/build && cmake -DCMAKE_INSTALL_PREFIX:PATH=$HOME/kcov .. && make && make install
