#!/bin/sh
set -e
wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz
tar xzf master.tar.gz
mkdir kcov-master/build
cd kcov-master/build && cmake -DCMAKE_INSTALL_PREFIX:PATH=$HOME/kcov .. && make && make install && make install
