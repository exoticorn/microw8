+++
+++

## About

MicroW8 is a WebAssembly based fantasy console inspired by the likes of [TIC-80](https://tic80.com/), [WASM-4](https://wasm4.org/) and [PICO-8](https://www.lexaloffle.com/pico-8.php).

The initial motivation behind MicroW8 was to explore whether there was a way to make WebAssembly viable for size-coding. (Size coding being the art of creating tiny (often <= 256 bytes) graphical effects and games.) The available examples so far are all in this space, however, I very carefully made sure that all design decisions make sense from the point of view of bigger projects as well.

## Specs

* Screen: 320x240, 256 colors, 60Hz
* Modules: Up to 256KB (WASM)
* Memory: 256KB
* Gamepad input (D-Pad + 4 Buttons)

## Examples
* [Fireworks](v0.1pre5#AgwvgP+M59snqjl4CMKw5sqm1Zw9yJCbSviMjeLUdHus2a3yl/a99+uiBeqZgP/2jqSjrLjRk73COMM6OSLpsxK8ugT1kuk/q4hQUqqPpGozHoa0laulzGGcahzdfdJsYaK1sIdeIYS9M5PnJx/Wk9H+PvWEPy2Zvv7I6IW7Fg==) (127 bytes): Some fireworks to welcome 2022.
* [Skip Ahead](v0.1pre5#AgyfpZ80wkW28kiUZ9VIK4v+RPnVxqjK1dz2BcDoNyQPsS2g4OgEzkTe6jyoAfFOmqKrS8SM2aRljBal9mjNn8i4fP9eBK+RehQKxxGtJa9FqftvqEnh3ez1YaYxqj7jgTdzJ/WAYVmKMovBT1myrX3FamqKSOgMsNedLhVTLAhQup3sNcYEjGNo8b0HZ5+AgMgCwYRGCe//XQOMAaAAzqDILgmpEZ/43RKHcQpHEQwbURfNQJpadJe2sz3q5FlQnTGXQ9oSMokidhlC+aR/IpNHieuBGLhFZ2GfnwVQ0geBbQpTPA==) (229 bytes): A port of my [TIC-80 256byte game](http://tic80.com/play?cart=1735) from LoveByte'21
* [OhNoAnotherTunnel](v0.1pre4#Ag95rdCB5Ww5NofyQaKF4P1mrNRso4azgiem4hK99Gh8OMzSpFq3NsNDo7O7pqln10D11l9uXr/ritw7OEzKwbEfCdvaRnS2Z0Kz0iDEZt/gIqOdvFmxsL1MjPQ4XInPbUJpQUonhQq29oP2omFabnQxn0bzoK7mZjcwc5GetHG+hGajkJcRr8oOnjfCol8RD+ha33GYtPnut+GLe4ktzf5UxZwGs6oT9qqC61lRDakN) (177 bytes): A port of my [entry](http://tic80.com/play?cart=1871) in the Outline'21 bytebattle final
* [Technotunnel](v0.1pre4#AqL8HeK1M9dn2nWNIF5vaq/Vh64pMt5nJIFoFKpBMPUsGtDtpqjo1JbT9LzPhAxCqJ7Yh4TA6oTGd4xhLowf+cWZMY73+7AZmfXJJsBi4cej/hH+4wlAgxFIrnOYnr/18IpnZbsHf0eGm1BhahX74+cVR0TRmNQmYC7GhCNS3mv/3MJn74lCj7t28aBJPjEZhP9fGXdG2u5Egh/Tjdg=) (158 bytes): A port of my [entry](https://tic80.com/play?cart=1873) in the Outline'21 bytebattle quater final
* [Font & Palette](v0.1pre4#AgKaeeOuwg5gCKvFIeiitEwMpUI2rymEcu+DDB1vMu9uBoufvUxIr4Y5p4Jj2ukoNO4PE7QS5cN1ZyDMCRfSzYIGZxKlN2J6NKEWK7KVPk9wVUgn1Ip+hsMinWgEO8ETKfPuHoIa4kjI+ULFOMad7vd3rt/lh1Vy9w+R2MXG/7T61d3c7C6KY+eQNS0eW3ys4iU8R6SycuWZuuZ2Sg3Qxp826s+Kt+2qBojpzNOSoyFqyrVyYMTKEkSl0BZOj59Cs1hPm5bq0F1MmVhGAzMhW9V4YeAe): Just a simple viewer for the default font and palette.
* [Technotunnel B/W](v0.1pre2#AQrDAQHAAQIBfwp9A0AgAUEAsiABQcACb7JDmhkgQ5MiBCAEIASUIAFBwAJtQfgAa7IiBSAFlJKRIgaVIgcgByAAskHQD7KVIgIQAEPNzEw/lCIDlCAHIAeUIAOUIAOUQQGykiADIAOUk5GSIgiUIAOTQQqylCACkiIJqCAFIAaVIAiUQQqylCACkiIKqHMgCEEyspQgBpUiCyACkkEUspSocUEFcbJBArIgC5OUQRaylJeoOgB4IAFBAWoiAUGA2ARIDQALCw==) (199 bytes uncompressed): A port of my [entry](https://tic80.com/play?cart=1873) in the Outline'21 bytebattle quater final (older MicroW8 version with monochrome palette)
* [XorScroll](v0.1pre2#AQovAS0BAX8DQCABIAFBwAJvIABBCm1qIAFBwAJtczoAeCABQQFqIgFBgNgESA0ACws=) (50 bytes uncompressed): A simple scrolling XOR pattern. Fun fact: This is the pre-loaded effect when entering a bytebattle.
* [CircleWorm](v0.1pre2#AQp7AXkCAX8CfUEgEA0DQCABskEEspUiAkECspUgALJBiCeylSIDQQWylJIQAEEBspJBoAGylCACQQOylSADQQSylJIQAEEBspJB+ACylCADQRGylCACQQKylJIQAEECspJBELKUIAFBAmxBP2oQEiABQQFqIgFBP0gNAAsL) (126 bytes uncompressed): Just a test for the circle fill function.

## Versions

* [v0.1pre1](v0.1pre1)
* [v0.1pre2](v0.1pre2)
* [v0.1pre3](v0.1pre3)
* [v0.1pre4](v0.1pre4)
* [v0.1pre5](v0.1pre5)

## Tooling

WARNING: severely out of date. The `uw8` binary includes a lot of the tool features below.

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
