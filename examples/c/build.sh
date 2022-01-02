#!/bin/bash

clang -O2 -Wno-incompatible-library-redeclaration --no-standard-libraries -ffast-math -Wl,--no-entry -Wl,--export-all -Wl,--import-memory -Wl,--initial-memory=262144 -Wl,-zstack-size=90000 -o cart.wasm cart.c --target=wasm32 && \
uw8 filter-exports cart.wasm cart.wasm && \
wasm-opt -Oz --fast-math --strip-producers -o cart.wasm cart.wasm && \
uw8 pack -l 9 cart.wasm cart.uw8