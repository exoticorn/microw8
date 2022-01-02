rustc --target=wasm32-unknown-unknown --crate-type cdylib -C opt-level="z" -C "link-args=--import-memory --initial-memory=262144 -zstack-size=90000" -o tunnel.wasm tunnel.rs && \
uw8 filter-exports tunnel.wasm tunnel.wasm && \
wasm-opt -Oz --strip-producers -o tunnel.wasm tunnel.wasm && \
uw8 pack -l 9 tunnel.wasm tunnel.uw8
