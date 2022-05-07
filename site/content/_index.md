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
* [Fireworks](v0.1.2#AgwvgP+M59snqjl4CMKw5sqm1Zw9yJCbSviMjeLUdHus2a3yl/a99+uiBeqZgP/2jqSjrLjRk73COMM6OSLpsxK8ugT1kuk/q4hQUqqPpGozHoa0laulzGGcahzdfdJsYaK1sIdeIYS9M5PnJx/Wk9H+PvWEPy2Zvv7I6IW7Fg==) (127 bytes): Some fireworks to welcome 2022.
* [Skip Ahead](v0.1.2#AgyfpZ80wkW28kiUZ9VIK4v+RPnVxqjK1dz2BcDoNyQPsS2g4OgEzkTe6jyoAfFOmqKrS8SM2aRljBal9mjNn8i4fP9eBK+RehQKxxGtJa9FqftvqEnh3ez1YaYxqj7jgTdzJ/WAYVmKMovBT1myrX3FamqKSOgMsNedLhVTLAhQup3sNcYEjGNo8b0HZ5+AgMgCwYRGCe//XQOMAaAAzqDILgmpEZ/43RKHcQpHEQwbURfNQJpadJe2sz3q5FlQnTGXQ9oSMokidhlC+aR/IpNHieuBGLhFZ2GfnwVQ0geBbQpTPA==) (229 bytes): A port of my [TIC-80 256byte game](http://tic80.com/play?cart=1735) from LoveByte'21
* [OhNoAnotherTunnel](v0.1.2#AgPP1oEFvPzY/rBZwTumtYn37zeMFgpir1Bkn91jsNcp26VzoUpkAOOJTtnzVBfW+/dGnnIdbq/irBUJztY5wuua80DORTYZndgdwZHcSk15ajc4nyO0g1A6kGWyW56oZk0iPYJA9WtUmoj0Plvy1CGwIZrMe57X7QZcdqc3u6zjTA41Tpiqi9vnO3xbhi8o594Vx0XPXwVzpYq1ZCTYenfAGaXKkDmAFJqiVIsiCg==) (175 bytes): A port of my [entry](http://tic80.com/play?cart=1871) in the Outline'21 bytebattle final
* [Technotunnel](v0.1.2#AhPXpq894LaUhp5+HQf39f39/Jc8g5zUrBSc0uyKh36ivskczhY84h55zL8gWpkdvKuRQI+KIt80isKzh8jkM8nILcx0RUvyk8yjE8TgNsgkcORVI0RY5k3qE4ySjaycxa2DVZH61UWZuLsCouuwT7I80TbmmetQSbMywJ/avrrCZIAH0UzQfvOiCJNG48NI0FFY1vjB7a7dcp8Uqg==) (157 bytes): A port of my [entry](https://tic80.com/play?cart=1873) in the Outline'21 bytebattle quater final
* [Font & Palette](v0.1.2#At/p39+IBnj6ry1TRe7jzVy2A4tXgBvmoW2itzoyF2aM28pGy5QDiKxqrk8l9sbWZLtnAb+jgOfU+9QhpuyCAkhN6gPOU481IUL/df96vNe3h288Dqwhd3sfFpothIVFsMwRK72kW2hiR7zWsaXyy5pNmjR6BJk4piWx9ApT1ZwoUajhk6/zij6itq/FD1U3jj/J3MOwqZ2ef8Bv6ZPQlJIYVf62icGa69wS6SI1qBpIFiF14F8PcztRVbKIxLpT4ArCS6nz6FPnyUkqATGSBNPJ): Just a simple viewer for the default font and palette.

Examplers for older versions:

* [Technotunnel B/W](v0.1pre2#AQrDAQHAAQIBfwp9A0AgAUEAsiABQcACb7JDmhkgQ5MiBCAEIASUIAFBwAJtQfgAa7IiBSAFlJKRIgaVIgcgByAAskHQD7KVIgIQAEPNzEw/lCIDlCAHIAeUIAOUIAOUQQGykiADIAOUk5GSIgiUIAOTQQqylCACkiIJqCAFIAaVIAiUQQqylCACkiIKqHMgCEEyspQgBpUiCyACkkEUspSocUEFcbJBArIgC5OUQRaylJeoOgB4IAFBAWoiAUGA2ARIDQALCw==) (199 bytes uncompressed): A port of my [entry](https://tic80.com/play?cart=1873) in the Outline'21 bytebattle quater final (older MicroW8 version with monochrome palette)
* [XorScroll](v0.1pre2#AQovAS0BAX8DQCABIAFBwAJvIABBCm1qIAFBwAJtczoAeCABQQFqIgFBgNgESA0ACws=) (50 bytes uncompressed): A simple scrolling XOR pattern. Fun fact: This is the pre-loaded effect when entering a bytebattle.
* [CircleWorm](v0.1pre2#AQp7AXkCAX8CfUEgEA0DQCABskEEspUiAkECspUgALJBiCeylSIDQQWylJIQAEEBspJBoAGylCACQQOylSADQQSylJIQAEEBspJB+ACylCADQRGylCACQQKylJIQAEECspJBELKUIAFBAmxBP2oQEiABQQFqIgFBP0gNAAsL) (126 bytes uncompressed): Just a test for the circle fill function.

## Versions

### v0.2.0-rc3

* [Web runtime](v0.2.0-rc3)
* [Linux](https://github.com/exoticorn/microw8/releases/download/v0.2.0-rc3/microw8-0.2.0-rc3-linux.tgz)
* [MacOS](https://github.com/exoticorn/microw8/releases/download/v0.2.0-rc3/microw8-0.2.0-rc3-macos.tgz)
* [Windows](https://github.com/exoticorn/microw8/releases/download/v0.2.0-rc3/microw8-0.2.0-rc3-windows.zip)

Changes:

* improve timing stability some more. essentially now guaranteeing that "frame = time_ms * 6 / 100" returns
  consecutive frame numbers, provided the module can be run at 60 fps
* add support to redirect text output to the console for debugging using control code 6
* update curlywas:
* * add support for `else if`
* * add support for escape sequences in strings
* * add support for char literals
* * add support for binop-assignment, eg. `+=`, `^=`, `<<=` etc. (also support for the tee operator: `+:=`)

### v0.2.0-rc2

* [Web runtime](v0.2.0-rc2)
* [Linux](https://github.com/exoticorn/microw8/releases/download/v0.2.0-rc2/microw8-0.2.0-rc2-linux.tgz)
* [MacOS](https://github.com/exoticorn/microw8/releases/download/v0.2.0-rc2/microw8-0.2.0-rc2-macos.tgz)
* [Windows](https://github.com/exoticorn/microw8/releases/download/v0.2.0-rc2/microw8-0.2.0-rc2-windows.zip)

Changes:

* fix timing issues of sound playback, especially on systems with large sound buffers

### v0.2.0-rc1

* [Web runtime](v0.2.0-rc1)
* [Linux](https://github.com/exoticorn/microw8/releases/download/v0.2.0-rc1/microw8-0.2.0-rc1-linux.tgz)
* [MacOS](https://github.com/exoticorn/microw8/releases/download/v0.2.0-rc1/microw8-0.2.0-rc1-macos.tgz)
* [Windows](https://github.com/exoticorn/microw8/releases/download/v0.2.0-rc1/microw8-0.2.0-rc1-windows.zip)

Changes:

* [add sound support](docs#sound)
* "integer constant cast to float" literal syntax in CurlyWas (ex. `1_f` is equivalent to `1 as f32`)

Known issues:

* timing accuracy/update frequency of sound support currently depends on sound buffer size

### v0.1.2

* [Web runtime](v0.1.2)
* [Linux](https://github.com/exoticorn/microw8/releases/download/v0.1.2/microw8-0.1.2-linux.tgz)
* [MacOS](https://github.com/exoticorn/microw8/releases/download/v0.1.2/microw8-0.1.2-macos.tgz)
* [Windows](https://github.com/exoticorn/microw8/releases/download/v0.1.2/microw8-0.1.2-windows.zip)

Changes:

* add option to `uw8 run` to run the cart in the browser using the web runtime
* CurlyWas: implement `include` support
* CurlyWas: implement support for constants
* fix crash when trying to draw zero sized line

### Older versions

[Find older versions here.](versions)