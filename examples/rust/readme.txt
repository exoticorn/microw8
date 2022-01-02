A small example how to produce somewhat reasonably small MicroW8
carts in rust.

A nightly rust compiler is needed for the unstable sqrtf32
intrinsic.

Simply compiling with rustc as shown in build.sh results in a
361 byte tunnel.wasm. Using wasm-opt this can be reduced to
255 bytes.

When you disassemble this wasm file using wasm2wat you can see
these globals and exports:

(global (;0;) i32 (i32.const 90000))
(global (;1;) i32 (i32.const 90000))
(export "__data_end" (global 0))
(export "__heap_base" (global 1))

They are meant to be used for heap allocations and stack for any
values that are not simple scalars (i32, f32, etc.). Since our
code doesn't actually use any of that, the globals are only
referenced by the exports and we can remove them using
'uw8 filter-exports' (preferably before running wasm-opt) which
removes all exports except those used by the MicroW8 platform.

This gives us a 211 byte wasm file. Running this through
uw8 pack brings us to the final size of 119 bytes.