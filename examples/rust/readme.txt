A small example how to produce somewhat reasonably small MicroW8
carts in rust.

A nightly rust compiler is needed for the unstable sqrtf32
intrinsic.

Simply compiling with rustc as shown in build.sh results in a
339 byte tunnel.wasm. Using wasm-opt this can be reduced to
244 bytes.

When you disassemble this wasm file using wasm2wat you can see
these globals and exports:

(global (;0;) i32 (i32.const 65536))
(global (;1;) i32 (i32.const 65536))
(export "__data_end" (global 0))
(export "__heap_base" (global 1))

They are meant to be used for heap allocations and stack for any
values that are not simple scalars (i32, f32, etc.). Since our
code doesn't actually use any of that, we can just delete them
in a text editor and assemble the code again with wat2wasm.

This gives us a 200 byte wasm file. Running this through
uw8-tool pack brings us to the final size of 137 bytes.