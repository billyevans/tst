#!/bin/sh
set -e
KCOV_VERSION=34

wget https://github.com/SimonKagstrom/kcov/archive/v${KCOV_VERSION}.tar.gz
tar xzf master.tar.gz
mkdir kcov-${KCOV_VERSION}/build
cd kcov-${KCOV_VERSION}/build && cmake -DCMAKE_INSTALL_PREFIX:PATH=$HOME/kcov .. && make && make install && make install
