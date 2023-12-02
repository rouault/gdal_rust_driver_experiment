#!/bin/sh
set -eu
cargo build
g++ -Wall -Wextra -fPIC -g lib.cpp   -I. -shared -o target/debug/libgdalcore.so
g++  -Wall -g -O0 main.cpp -I. -Ltarget/debug/ -lgdalcore  -lgdal_rust_driver -lpthread -ldl -o target/debug/test
LD_LIBRARY_PATH=target/debug valgrind --leak-check=full --num-callers=20 ./target/debug/test

