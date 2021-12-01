+++
+++

## Versions

* [v0.1pre1](v0.1pre1)
* [v0.1pre2](v0.1pre2)
* [v0.1pre3](v0.1pre3)

## Spec

MicroW8 loads WebAssembly modules with a maximum size of 256kb. You module needs to export
a function `fn upd()` which will be called once per frame.
After calling `upd` MicroW8 will display the 320x240 8bpp framebuffer located
at offset 120 in memory with the 32bpp palette located at 0x13000.

The memory has to be imported as `"env" "memory"` and has a maximum size of 256kb (4 pages).

Other imports provided by the platform, also all in module `env`:

* `fn acos(f32) -> f32`
* `fn asin(f32) -> f32`
* `fn atan(f32) -> f32`
* `fn atan2(f32, f32) -> f32`
* `fn cos(f32) -> f32`
* `fn exp(f32, f32) -> f32`
* `fn log(f32) -> f32`
* `fn sin(f32) -> f32`
* `fn tan(f32) -> f32`
* `fn pow(f32) -> f32`
* `fn fmod(f32, f32) -> f32`

* `fn random() -> i32`
* `fn randomf() -> f32`
* `fn randomSeed(i32)`

* `fn cls(color: i32)`
* `fn setPixel(x: i32, y: i32, color: i32)`
* `fn getPixel(x: i32, y: i32) -> i32`
* `fn hline(left: i32, right: i32, y: i32, color: i32)`
* `fn rectangle(x1: f32, y1: f32, x2: f32, y2: f32, color: i32)`
* `fn circle(cx: f32, cy: f32, radius: f32, color: i32)`

* `fn time() -> f32`
* `fn isButtonPressed(btn: i32) -> i32`
* `fn isButtonTriggered(btn: i32) -> i32`

* `fn printChar(char: i32)`
* `fn printString(ptr: i32)`
* `fn printInt(num: i32)`

### Memory map

```
00000-00040: user memory
00040-00044: time since module start in ms
00044-0004c: gamepad state
0004c-00078: reserved
00078-12c78: frame buffer
12c78-13000: reserved
13000-13400: palette
13400-13c00: font
13c00-14000: reserved
14000-40000: user memory
```

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
the first function in the file under the name `upd`.

#### Format version `02`:

Same as version `01` except everything after the first byte is compressed
using a [custom LZ compression scheme](https://github.com/exoticorn/upkr).

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
* [Skip Ahead](v0.1pre3#Agj9nQYWw+yYP6xi7SUL2urlNtvh2dOZFuYL4PUxAUz5qqATDey0JsAVat2VQKEOyXK1bE+3WiFK0GGhdi4VAd8Tlf3YuU7xfvvBwN4oNuIoY29jbbuEnEnPZFjC4ym9L2QDUmig+RsF++FubWcyqOt7CAFGNEaAiMISCIM43bfPQriE6sD3orstkMjH3LPOqeuUPpitgzaIsAf860CYHlrAG2t5CSjRGobcPLJ+CSeYjzZSLYs7+u2xpthsfoIvBnk1+xxwEWYfZOJ3Madfo5BME5nceVCQVOEBMCTLSE+xVCkyelOW) (231 bytes): A port of my [TIC-80 256byte game](http://tic80.com/play?cart=1735) from LoveByte'21
* [OhNoAnotherTunnel](v0.1pre3#Apr9u4e6Rsy7tRABjq4o7dCGPLQR9dVTSGK9FWXemB7tybsZHT+TxtfHlarRbcekGcg7qZY/eK6/VVCp9ceNBXlh4v0QGS63LTzfEjb8XC4jg5KifbYBodSIS0DPVjwq32PbgjL2+C+QOCx6ZxqRYP0KQpcTxuBUKx1NXVM2EV4l0rEWBQW9SjLcbURKHYaRLcI4FOcLOfASFiQ4wFWgEBA0VD6hGdemN0tPYp9BfUaN) (177 bytes): A port of my [entry](http://tic80.com/play?cart=1871) in the Outline'21 bytebattle final
* [Technotunnel](v0.1pre3#AkL/tETJ+XRrvcB8gD9brftZ26zjwEsiATnAd+szCtw3Haq41srEMFO8aDS71c2CX8W87RQ9EY3V+YuTn/2CPRfn6CpgxMHUnIxOEWhVDRULXeYGP70dTiL8tYLAc8LrBWay9h8jCX/4Jbb39XVnISlZNd7In4Mts9LvSkSIz/E0sBfhjwp65aeoSU8BNFZpyKlxU+u5DdBtuxx3bFE=) (158 bytes): A port of my [entry](https://tic80.com/play?cart=1873) in the Outline'21 bytebattle quater final
* [Technotunnel B/W](v0.1pre2#AQrDAQHAAQIBfwp9A0AgAUEAsiABQcACb7JDmhkgQ5MiBCAEIASUIAFBwAJtQfgAa7IiBSAFlJKRIgaVIgcgByAAskHQD7KVIgIQAEPNzEw/lCIDlCAHIAeUIAOUIAOUQQGykiADIAOUk5GSIgiUIAOTQQqylCACkiIJqCAFIAaVIAiUQQqylCACkiIKqHMgCEEyspQgBpUiCyACkkEUspSocUEFcbJBArIgC5OUQRaylJeoOgB4IAFBAWoiAUGA2ARIDQALCw==) (199 bytes uncompressed): A port of my [entry](https://tic80.com/play?cart=1873) in the Outline'21 bytebattle quater final (older MicroW8 version with monochrome palette)
* [XorScroll](v0.1pre2#AQovAS0BAX8DQCABIAFBwAJvIABBCm1qIAFBwAJtczoAeCABQQFqIgFBgNgESA0ACws=) (50 bytes uncompressed): A simple scrolling XOR pattern. Fun fact: This is the pre-loaded effect when entering a bytebattle.
* [CircleWorm](v0.1pre2#AQp7AXkCAX8CfUEgEA0DQCABskEEspUiAkECspUgALJBiCeylSIDQQWylJIQAEEBspJBoAGylCACQQOylSADQQSylJIQAEEBspJB+ACylCADQRGylCACQQKylJIQAEECspJBELKUIAFBAmxBP2oQEiABQQFqIgFBP0gNAAsL) (126 bytes uncompressed): Just a test for the circle fill function.