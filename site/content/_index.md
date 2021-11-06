+++
+++

## Versions

* [v0.1pre1](v0.1pre1)

## Spec

MicroW8 loads WebAssembly modules with a maximum size of 256kb. You module needs to export
a function `fn tic(time: i32)` which will be called once per frame.
After calling `tic` MicroW8 will display the 320x256 8bpp framebuffer located
at offset 120 in memory.

The memory has to be imported as `"env" "memory"` and has a maximum size of 256kb (4 pages).

Other imports provided by the platform:

* in module `math`:
* * `fn acos(f32) -> f32`
* * `fn asin(f32) -> f32`
* * `fn atan(f32) -> f32`
* * `fn atan2(f32, f32) -> f32`
* * `fn cos(f32) -> f32`
* * `fn exp(f32, f32) -> f32`
* * `fn log(f32) -> f32`
* * `fn sin(f32) -> f32`
* * `fn tan(f32) -> f32`
* * `fn pow(f32) -> f32`

## `.uw8` format

The first byte of the file specifies the format version:

#### Format version `00`:

This file is simply a standard WebAssembly module

#### Format version `01`:

The rest of this file is the same as a WebAssembly
module with the 8 byte header removed. This module
can leave out sections which are then taken from
a base module provided by MicroW8.

You can generate this base module yourself using
`uw8-tool`. As a quick summary, it provides all function
types with up to 5 parameters (i32 or f32) where the
`f32` parameters always preceed the `i32` parameters.
Then it includes all imports that MicroW8 provides,
a function section with a single function of type
`(i32) -> void` and an export section that exports
the first function in the file under the name `tic`.

## Tooling

The [Web Assembly Binary Toolkit](https://github.com/WebAssembly/wabt) includes
a few useful tools, eg. `wat2wasm` to compile the WebAssemby text format to binary
wasm and `wasm2wat` to disassemble wasm binaries.

If you don't like the look of the `.wat` text format, you might want to take a
look at [CurlyWas](https://github.com/exoticorn/curlywas), a curly-braces infix
syntax for WebAssembly.

Once you have a size-efficient `.wasm` file, you can use [uw8-tool](https://github.com/exoticorn/microw8/tree/master/uw8-tool)
(currently included in the MicroW8 repository) to strip off sections of the
WebAssembly module that are provided by the MicroW8 platform.

Writing code for MicroW8 in C, Rust, AssemblyScript etc. should absolutely
possible but no examples are provided, yet.

## Examples

* [Technotunnel](v0.1pre1#AQrDAQHAAQIBfwp9A0AgAUEAsiABQcACb7JDmhkgQ5MiBCAEIASUIAFBwAJtQYABa7IiBSAFlJKRIgaVIgcgByAAskHQD7KVIgIQAEPNzEw/lCIDlCAHIAeUIAOUIAOUQQGykiADIAOUk5GSIgiUIAOTQQqylCACkiIJqCAFIAaVIAiUQQqylCACkiIKqHMgCEEyspQgBpUiCyACkkEUspSocUEFcbJBArIgC5OUQRaylJeoOgB4IAFBAWoiAUGAgAVIDQALCw==) (199 bytes): A port of my [entry](https://tic80.com/play?cart=1873) in the Outline'21 bytebattle quater final
* [XorScroll](v0.1pre1#AQovAS0BAX8DQCABIAFBwAJvIABBCm1qIAFBwAJtczoAeCABQQFqIgFBgIAFSA0ACws=) (50 bytes): A simple scrolling XOR pattern. Fun fact: This is the pre-loaded effect when entering a bytebattle.