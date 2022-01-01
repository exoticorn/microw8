+++
+++

## Versions

* [v0.1pre1](v0.1pre1)
* [v0.1pre2](v0.1pre2)
* [v0.1pre3](v0.1pre3)
* [v0.1pre4](v0.1pre4)
* [v0.1pre5](v0.1pre5)

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
* `fn rectangle_outline(x1: f32, y1: f32, x2: f32, y2: f32, color: i32)`
* `fn circle_outline(cx: f32, cy: f32, radius: f32, color: i32)`
* `fn line(x1: f32, y1: f32, x2: f32, y2: f32, color: i32)`

* `fn time() -> f32`
* `fn isButtonPressed(btn: i32) -> i32`
* `fn isButtonTriggered(btn: i32) -> i32`

* `fn printChar(char: i32)`
* `fn printString(ptr: i32)`
* `fn printInt(num: i32)`
* `fn setTextColor(color: i32)`
* `fn setBackgroundColor(color: i32)`
* `fn setCursorPosition(x: i32, y: i32)`

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
`() -> void` and an export section that exports
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
* [Fireworks](v0.1pre5#AgwvgP+M59snqjl4CMKw5sqm1Zw9yJCbSviMjeLUdHus2a3yl/a99+uiBeqZgP/2jqSjrLjRk73COMM6OSLpsxK8ugT1kuk/q4hQUqqPpGozHoa0laulzGGcahzdfdJsYaK1sIdeIYS9M5PnJx/Wk9H+PvWEPy2Zvv7I6IW7Fg==) (127 bytes): Some fireworks to welcome 2022.
* [Skip Ahead](v0.1pre5#AgyfpZ80wkW28kiUZ9VIK4v+RPnVxqjK1dz2BcDoNyQPsS2g4OgEzkTe6jyoAfFOmqKrS8SM2aRljBal9mjNn8i4fP9eBK+RehQKxxGtJa9FqftvqEnh3ez1YaYxqj7jgTdzJ/WAYVmKMovBT1myrX3FamqKSOgMsNedLhVTLAhQup3sNcYEjGNo8b0HZ5+AgMgCwYRGCe//XQOMAaAAzqDILgmpEZ/43RKHcQpHEQwbURfNQJpadJe2sz3q5FlQnTGXQ9oSMokidhlC+aR/IpNHieuBGLhFZ2GfnwVQ0geBbQpTPA==) (229 bytes): A port of my [TIC-80 256byte game](http://tic80.com/play?cart=1735) from LoveByte'21
* [OhNoAnotherTunnel](v0.1pre4#Ag95rdCB5Ww5NofyQaKF4P1mrNRso4azgiem4hK99Gh8OMzSpFq3NsNDo7O7pqln10D11l9uXr/ritw7OEzKwbEfCdvaRnS2Z0Kz0iDEZt/gIqOdvFmxsL1MjPQ4XInPbUJpQUonhQq29oP2omFabnQxn0bzoK7mZjcwc5GetHG+hGajkJcRr8oOnjfCol8RD+ha33GYtPnut+GLe4ktzf5UxZwGs6oT9qqC61lRDakN) (177 bytes): A port of my [entry](http://tic80.com/play?cart=1871) in the Outline'21 bytebattle final
* [Technotunnel](v0.1pre4#AqL8HeK1M9dn2nWNIF5vaq/Vh64pMt5nJIFoFKpBMPUsGtDtpqjo1JbT9LzPhAxCqJ7Yh4TA6oTGd4xhLowf+cWZMY73+7AZmfXJJsBi4cej/hH+4wlAgxFIrnOYnr/18IpnZbsHf0eGm1BhahX74+cVR0TRmNQmYC7GhCNS3mv/3MJn74lCj7t28aBJPjEZhP9fGXdG2u5Egh/Tjdg=) (158 bytes): A port of my [entry](https://tic80.com/play?cart=1873) in the Outline'21 bytebattle quater final
* [Font & Palette](v0.1pre4#AgKaeeOuwg5gCKvFIeiitEwMpUI2rymEcu+DDB1vMu9uBoufvUxIr4Y5p4Jj2ukoNO4PE7QS5cN1ZyDMCRfSzYIGZxKlN2J6NKEWK7KVPk9wVUgn1Ip+hsMinWgEO8ETKfPuHoIa4kjI+ULFOMad7vd3rt/lh1Vy9w+R2MXG/7T61d3c7C6KY+eQNS0eW3ys4iU8R6SycuWZuuZ2Sg3Qxp826s+Kt+2qBojpzNOSoyFqyrVyYMTKEkSl0BZOj59Cs1hPm5bq0F1MmVhGAzMhW9V4YeAe): Just a simple viewer for the default font and palette.
* [Technotunnel B/W](v0.1pre2#AQrDAQHAAQIBfwp9A0AgAUEAsiABQcACb7JDmhkgQ5MiBCAEIASUIAFBwAJtQfgAa7IiBSAFlJKRIgaVIgcgByAAskHQD7KVIgIQAEPNzEw/lCIDlCAHIAeUIAOUIAOUQQGykiADIAOUk5GSIgiUIAOTQQqylCACkiIJqCAFIAaVIAiUQQqylCACkiIKqHMgCEEyspQgBpUiCyACkkEUspSocUEFcbJBArIgC5OUQRaylJeoOgB4IAFBAWoiAUGA2ARIDQALCw==) (199 bytes uncompressed): A port of my [entry](https://tic80.com/play?cart=1873) in the Outline'21 bytebattle quater final (older MicroW8 version with monochrome palette)
* [XorScroll](v0.1pre2#AQovAS0BAX8DQCABIAFBwAJvIABBCm1qIAFBwAJtczoAeCABQQFqIgFBgNgESA0ACws=) (50 bytes uncompressed): A simple scrolling XOR pattern. Fun fact: This is the pre-loaded effect when entering a bytebattle.
* [CircleWorm](v0.1pre2#AQp7AXkCAX8CfUEgEA0DQCABskEEspUiAkECspUgALJBiCeylSIDQQWylJIQAEEBspJBoAGylCACQQOylSADQQSylJIQAEEBspJB+ACylCADQRGylCACQQKylJIQAEECspJBELKUIAFBAmxBP2oQEiABQQFqIgFBP0gNAAsL) (126 bytes uncompressed): Just a test for the circle fill function.