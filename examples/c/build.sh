#!/bin/bash

clang -O2 -Wno-incompatible-library-redeclaration --no-standard-libraries -ffast-math -Xclang -target-feature -Xclang +nontrapping-fptoint -Wl,--no-entry,--export-all,--import-memory,--initial-memory=262144,--global-base=81920,-zstack-size=4096 -o cart.wasm cart.c --target=wasm32 && \
uw8 filter-exports cart.wasm cart.wasm && \
wasm-opt -Oz --fast-math --strip-producers -o cart.wasm cart.wasm && \
uw8 pack -l 9 cart.wasm cart.uw8