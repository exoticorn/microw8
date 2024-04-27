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

For detailed [documentation see here](docs).

## Examples
* [Skip Ahead](v0.2.0#AgVfq24KI2Ok2o8qVtPYj27fSuGnfeSKgbOkIOsaEQMov8TDYQ6UjdjwkZrYcM1i9alo4/+Bhm1PRFEa0YHJlJAk/PGoc2K41rejv9ZSqJqIHNjr7cappqhOR2jT+jk+0b0+U6hO+geRCTP2aufWs7L+f/Z27NFY8LKlqPSv+C6Rd6+ohoKi6sYl5Kcrlf1cyTinV7jTTnmbcXWVDBA5rRKxAGMUTDS8rHxqSztRITOaQVP1pSdYgi/BDdOJOxSOIkeaId84S+Ycls5na7EgwSfVIpgqF+tcfkUecb8t2mQrXA7pyKrh/wzHn5N6Oe5aOgmzY2YpTIct) (249 bytes): A port of my [TIC-80 256byte game](http://tic80.com/play?cart=1735) from LoveByte'21, now with sound
* [Fireworks](v0.2.0#AgwvgP+M59snqjl4CMKw5sqm1Zw9yJCbSviMjeLUdHus2a3yl/a99+uiBeqZgP/2jqSjrLjRk73COMM6OSLpsxK8ugT1kuk/q4hQUqqPpGozHoa0laulzGGcahzdfdJsYaK1sIdeIYS9M5PnJx/Wk9H+PvWEPy2Zvv7I6IW7Fg==) (127 bytes): Some fireworks to welcome 2022.
* [OhNoAnotherTunnel](v0.2.0#AgPP1oEFvPzY/rBZwTumtYn37zeMFgpir1Bkn91jsNcp26VzoUpkAOOJTtnzVBfW+/dGnnIdbq/irBUJztY5wuua80DORTYZndgdwZHcSk15ajc4nyO0g1A6kGWyW56oZk0iPYJA9WtUmoj0Plvy1CGwIZrMe57X7QZcdqc3u6zjTA41Tpiqi9vnO3xbhi8o594Vx0XPXwVzpYq1ZCTYenfAGaXKkDmAFJqiVIsiCg==) (175 bytes): A port of my [entry](http://tic80.com/play?cart=1871) in the Outline'21 bytebattle final
* [Technotunnel](v0.2.0#AhPXpq894LaUhp5+HQf39f39/Jc8g5zUrBSc0uyKh36ivskczhY84h55zL8gWpkdvKuRQI+KIt80isKzh8jkM8nILcx0RUvyk8yjE8TgNsgkcORVI0RY5k3qE4ySjaycxa2DVZH61UWZuLsCouuwT7I80TbmmetQSbMywJ/avrrCZIAH0UzQfvOiCJNG48NI0FFY1vjB7a7dcp8Uqg==) (157 bytes): A port of my [entry](https://tic80.com/play?cart=1873) in the Outline'21 bytebattle quater final
* [Font & Palette](v0.2.0#At/p39+IBnj6ry1TRe7jzVy2A4tXgBvmoW2itzoyF2aM28pGy5QDiKxqrk8l9sbWZLtnAb+jgOfU+9QhpuyCAkhN6gPOU481IUL/df96vNe3h288Dqwhd3sfFpothIVFsMwRK72kW2hiR7zWsaXyy5pNmjR6BJk4piWx9ApT1ZwoUajhk6/zij6itq/FD1U3jj/J3MOwqZ2ef8Bv6ZPQlJIYVf62icGa69wS6SI1qBpIFiF14F8PcztRVbKIxLpT4ArCS6nz6FPnyUkqATGSBNPJ): Just a simple viewer for the default font and palette.

Examplers for older versions:

* [Technotunnel B/W](v0.1pre2#AQrDAQHAAQIBfwp9A0AgAUEAsiABQcACb7JDmhkgQ5MiBCAEIASUIAFBwAJtQfgAa7IiBSAFlJKRIgaVIgcgByAAskHQD7KVIgIQAEPNzEw/lCIDlCAHIAeUIAOUIAOUQQGykiADIAOUk5GSIgiUIAOTQQqylCACkiIJqCAFIAaVIAiUQQqylCACkiIKqHMgCEEyspQgBpUiCyACkkEUspSocUEFcbJBArIgC5OUQRaylJeoOgB4IAFBAWoiAUGA2ARIDQALCw==) (199 bytes uncompressed): A port of my [entry](https://tic80.com/play?cart=1873) in the Outline'21 bytebattle quater final (older MicroW8 version with monochrome palette)
* [XorScroll](v0.1pre2#AQovAS0BAX8DQCABIAFBwAJvIABBCm1qIAFBwAJtczoAeCABQQFqIgFBgNgESA0ACws=) (50 bytes uncompressed): A simple scrolling XOR pattern. Fun fact: This is the pre-loaded effect when entering a bytebattle.
* [CircleWorm](v0.1pre2#AQp7AXkCAX8CfUEgEA0DQCABskEEspUiAkECspUgALJBiCeylSIDQQWylJIQAEEBspJBoAGylCACQQOylSADQQSylJIQAEEBspJB+ACylCADQRGylCACQQKylJIQAEECspJBELKUIAFBAmxBP2oQEiABQQFqIgFBP0gNAAsL) (126 bytes uncompressed): Just a test for the circle fill function.

## Versions

### v0.3.0

* [Web runtime](v0.3.0)
* [Linux](https://github.com/exoticorn/microw8/releases/download/v0.3.0/microw8-0.3.0-linux.tgz)
* [MacOS](https://github.com/exoticorn/microw8/releases/download/v0.3.0/microw8-0.3.0-macos.tgz)
* [Windows](https://github.com/exoticorn/microw8/releases/download/v0.3.0/microw8-0.3.0-windows.zip)

Changes:

* add blitSprite and grabSprite API calls
* add support for integer scaling up to 16x for printing text
* fix incompatibility with sound devices only offering 16bit audio formats
* add support for br_table instruction in packed carts

### Older versions

[Find older versions here.](versions)
