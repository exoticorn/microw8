#!/bin/bash

tinygo build -o cart.wasm -target target.json ./main.go
uw8 filter-exports cart.wasm cart.wasm && \
#wasm-opt -Oz --fast-math --strip-producers -o cart.wasm cart.wasm && \
uw8 pack -l 9 cart.wasm cart.uw8
