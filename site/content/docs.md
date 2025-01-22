+++
title = "Docs"
description = "Docs"
+++

# Overview

MicroW8 loads WebAssembly modules with a maximum size of 256kb. Your module needs to export
a function `fn upd()` which will be called once per frame.
After calling `upd` MicroW8 will display the 320x240 8bpp framebuffer located
at offset 120 in memory with the 32bpp palette located at 0x13000.

The memory has to be imported as `env` `memory` and has a maximum size of 256kb (4 pages).

If the module exports a function called `start`, it will be called once after the module is
loaded.

# Memory map

```
00000-00040: user memory
00040-00044: time since module start in ms
00044-0004c: gamepad state
0004c-00050: number of frames since module start
00050-00070: sound data (synced to sound thread)
00070-00078: reserved
00078-12c78: frame buffer
12c78-12c7c: sound registers/work area base address (for sndGes function)
12c7c-13000: reserved
13000-13400: palette
13400-13c00: font
13c00-14000: reserved
14000-40000: user memory
```

# API

All API functions are found in the `env` module.

## Math

These all do what you'd expect them to. All angles are in radians.

### fn asin(x: f32) -> f32

Returns the arcsine of `x`.

### fn acos(x: f32) -> f32

Returns the arccosine of `x`.

### fn atan(f32) -> f32

Returns the arctangent of `x`.

### fn atan2(y: f32, x: f32) -> f32

Returns the angle between the point `(x, y)` and the positive x-axis.

### fn sin(angle: f32) -> f32

Returns the sine of `angle`.

### fn tan(angle: f32) -> f32

Returns the tangent of `angle`.

### fn cos(angle: f32) -> f32

Returns the cosine of `angle`.

### fn exp(x: f32) -> f32

Returns `e^x`.

### fn log(x: f32) -> f32

Returns the natural logarithmus of `x`. Ie. `e^log(x) == x`.

### fn pow(x: f32, y: f32) -> f32

Returns `x^y`.

### fn fmod(x: f32, y: f32) -> f32

Returns `x` modulo `y`, ie. `x - floor(x / y) * y`. This means the sign of the result of `fmod` is the same as `y`.

## Random

MicroW8 provides a pretty good PRNG, namely xorshift64*. It is initialized to a constant seed at each startup, so if you
want to vary the random sequence you'll need to provide a seed yourself.

### fn random() -> i32

Returns a (pseudo-)random 32bit integer.

### fn randomf() -> f32

Returns a (pseudo-)random float equally distributed in `[0,1)`.

### fn randomSeed(seed: i32)

Seeds the PRNG with the given seed. The seed function is reasonably strong so that you can use

```
randomSeed(index);
random()
```

as a cheap random-access PRNG (aka noise function).

## Graphics

The default palette can be seen [here](../v0.1.0#At/p39+IBnj6ry1TRe7jzVy2A4tXgBvmoW2itzoyF2aM28pGy5QDiKxqrk8l9sbWZLtnAb+jgOfU+9QhpuyCAkhN6gPOU481IUL/df96vNe3h288Dqwhd3sfFpothIVFsMwRK72kW2hiR7zWsaXyy5pNmjR6BJk4piWx9ApT1ZwoUajhk6/zij6itq/FD1U3jj/J3MOwqZ2ef8Bv6ZPQlJIYVf62icGa69wS6SI1qBpIFiF14F8PcztRVbKIxLpT4ArCS6nz6FPnyUkqATGSBNPJ). (Press Z on the keyboard to switch to palette.)

The palette can be changed by writing 32bit rgba colors to addresses 0x13000-0x13400.

The drawing functions are sub-pixel accurate where applicable (line, circle). Pixel centers lie halfway between integer
coordinates. Ie. the top-left pixel covers the area `0,0 - 1,1`, with `0.5,0.5` being the pixel center.

### fn cls(color: i32)

Clears the screen to the given color index. Also sets the text cursor to `0, 0` and disables graphical text mode.

### fn setPixel(x: i32, y: i32, color: i32)

Sets the pixel at `x, y` to the given color index.

### fn getPixel(x: i32, y: i32) -> i32

Returns the color index at `x, y`. Returns `0` if the given coordinates are outside the screen.

### fn hline(left: i32, right: i32, y: i32, color: i32)

Fills the horizontal line `[left, right), y` with the given color index.

### fn rectangle(x: f32, y: f32, w: f32, h: f32, color: i32)

Fills the rectangle `x,y - x+w,y+h` with the given color index.

(Sets all pixels where the pixel center lies inside the rectangle.)

### fn circle(cx: f32, cy: f32, radius: f32, color: i32)

Fills the circle at `cx, cy` and with `radius` with the given color index.

(Sets all pixels where the pixel center lies inside the circle.)

### fn rectangleOutline(x: f32, y: f32, w: f32, h: f32, color: i32)

Draws a one pixel outline on the inside of the given rectangle.

(Draws the outermost pixels that are still inside the rectangle area.)

### fn circleOutline(cx: f32, cy: f32, radius: f32, color: i32)

Draws a one pixel outline on the inside of the given circle.

(Draws the outermost pixels that are still inside the circle area.)

### fn line(x1: f32, y1: f32, x2: f32, y2: f32, color: i32)

Draws a line from `x1,y1` to `x2,y2` in the given color index.

### fn blitSprite(spriteData: i32, size: i32, x: i32, y: i32, control: i32)

Copies the pixel data at `spriteData` onto the screen at `x`, `y`. The size of the sprite is passed as `width | (height << 16)`.
If the height is given as 0, the sprite is is treated as square (width x width).

The control parameter controls masking and flipping of the sprite:
* bits 0-7: color mask index
* bit 8: switch on masked blit (pixel with color mask index are treated as transparent)
* bit 9: flip sprite x
* bit 10: flip sprite y

### fn grabSprite(spriteData: i32, size: i32, x: i32, y: i32, control: i32)

Copies the pixel data on the screen at `x`, `y` to `spriteData`. Parameters are exactly the same as `blitSprite`.

## Input

MicroW8 provides input from a gamepad with one D-Pad and 4 buttons, or a keyboard emulation thereof.

The buttons are numbered

| Button | Keyboard    | Index |
| ------ | ----------- | ----- |
| Up     | Arrow-Up    | 0     |
| Down   | Arrow-Down  | 1     |
| Left   | Arrow-Left  | 2     |
| Right  | Arrow-Right | 3     |
| A      | Z           | 4     |
| B      | X           | 5     |
| X      | A           | 6     |
| Y      | S           | 7     |

In addition to using the API functions below, the gamepad state can also be read as a bitfield of
pressed buttons at address 0x44. 0x48 holds the buttons that were pressed last frame.

### fn isButtonPressed(btn: i32) -> i32

Returns whether the buttons with the given index is pressed this frame.

### fn isButtonTriggered(btn: i32) -> i32

Returns whether the given button is newly pressed this frame.

### fn time() -> f32

Returns the time in seconds since the start of the cart.

The integer time in milliseconds can also be read at address 0x40.

## Text output

The default font can be seen [here](../v0.1.0#At/p39+IBnj6ry1TRe7jzVy2A4tXgBvmoW2itzoyF2aM28pGy5QDiKxqrk8l9sbWZLtnAb+jgOfU+9QhpuyCAkhN6gPOU481IUL/df96vNe3h288Dqwhd3sfFpothIVFsMwRK72kW2hiR7zWsaXyy5pNmjR6BJk4piWx9ApT1ZwoUajhk6/zij6itq/FD1U3jj/J3MOwqZ2ef8Bv6ZPQlJIYVf62icGa69wS6SI1qBpIFiF14F8PcztRVbKIxLpT4ArCS6nz6FPnyUkqATGSBNPJ).

The font can be changed by writing 1bpp 8x8 characters to addresses 0x13400-0x13c00.

All text printing is done at the cursor position, which is advanced after printing each character.
The cursor is not visible.

Text printing can operate in two modes - normal and graphics. After startup and after `cls()` normal mode is active.

### Normal mode

In normal mode, text printing is constrained to an 8x8 character grid. Setting the cursor position to `2,3` will start printing at pixel coordinates `16,24`.

When printing characters, the full 8x8 pixels are painted with the text and background colors according to the character graphics in the font.

When moving/printing past the left or right border the cursor will automatically wrap to the previous/next line. When moving/printing past the upper/lower border, the screen will be scrolled down/up 8 pixels, filling the fresh line with the background color.

### Graphics mode

In graphics mode, text can be printed to any pixel position, the cursor position is set in pixel coordinates.

When printing characters only the foreground pixels are set, the background is "transparent".

Moving/printing past any border does not cause any special operation, the cursor just goes off-screen.

### Text scale

An integer text scale factor in the range 1x-16x can be set with control char 30. An attempt to
set a scale outside that range will reset the scale to 1x.

After startup and `cls` the scale is initialized to 1x.

### Control chars

Characters 0-31 are control characters and don't print by default. They take the next 0-2 following characters as parameters.
Avoid the reserved control chars, they are currently NOPs but their behavior can change in later MicroW8 versions.

| Code  | Parameters | Operation                                  |
| ----- | ---------- | ------------------------------------------ |
| 0     | -          | Nop                                        |
| 1     | char       | Print char (including control chars)       |
| 2-3   | -          | Reserved                                   |
| 4     | -          | Switch to normal mode, reset cursor to 0,0 |
| 5     | -          | Switch to graphics mode                    |
| 6     | -          | Switch output to (debug) console           |
| 7     | -          | Bell / trigger sound channel 0             |
| 8     | -          | Move cursor left                           |
| 9     | -          | Move cursor right                          |
| 10    | -          | Move cursor down                           |
| 11    | -          | Move cursor up                             |
| 12    | -          | do `cls(background_color)`                 |
| 13    | -          | Move cursor to the left border             |
| 14    | color      | Set the background color                   |
| 15    | color      | Set the text color                         |
| 16-23 | -          | Reserved                                   |
| 24    | -          | Swap text/background colors                |
| 25-29 | -          | Reserved                                   |
| 30    | scale      | Set text scale (1-16)                      |
| 31    | x, y       | Set cursor position (*)                    |

(*) In graphics mode, the x coordinate is doubled when using control char 31 to be able to cover the whole screen with one byte.

#### Debug output

Control code 6 switches all text output (except codes 4 and 5 to switch output back to the screen) to the console. Where exactly this ends
up (if at all) is an implementation detail of the runtimes. The native dev-runtime writes the debug output to `stdout`, the web runtime to
the debug console using `console.log`. Both implementations buffer the output until they encounter a newline character (10) in the output stream.

There may be future runtimes that ignore the debug output completely.

In CurlyWas, a simple way to log some value might look like this:
```
printChar('\06V: '); // switch to console out, print some prefix
printInt(some_value);
printChar('\n\4'); // newline and switch back to screen
```

### fn printChar(char: i32)

Prints the character in the lower 8 bits of `char`. If the upper 24 bits are non-zero, right-shifts `char` by 8 bits and loops back to the beginning.

### fn printString(ptr: i32)

Prints the zero-terminated string at the given memory address.

### fn printInt(num: i32)

Prints `num` as a signed decimal number.

### fn setTextColor(color: i32)

Sets the text color.

### fn setBackgroundColor(color: i32)

Sets the background color.

### fn setCursorPosition(x: i32, y: i32)

Sets the cursor position. In normal mode `x` and `y` are multiplied by 8 to get the pixel position, in graphics mode they are used as is.

## Sound

### Low level operation

MicroW8 actually runs two instances of your module. On the first instance, it calls `upd` and displays the framebuffer found in its memory. On the
second instance, it calls `snd` instead with an incrementing sample index and expects that function to return sound samples for the left and right
channel at 44100 Hz. If your module does not export a `snd` function, it calls the api function `sndGes` instead.

As the only means of communication, 32 bytes starting at address 0x00050 are copied from main to sound memory after `upd` returns.

By default, the `sndGes` function generates sound based on the 32 bytes at 0x00050. This means that in the default configuration those 32 bytes act
as sound registers. See the `sndGes` function for the meaning of those registers.

### export fn snd(sampleIndex: i32) -> f32

If the module exports a `snd` function, it is called 88200 times per second to provide PCM sample data for playback (44.1kHz stereo).
The `sampleIndex` will start at 0 and increments by 1 for each call. On even indices the function is expected to return a sample value for
the left channel, on odd indices for the right channel.

### fn playNote(channel: i32, note: i32)

Triggers a note (1-127) on the given channel (0-3). Notes are semitones with 69 being A4 (same as MIDI). A note value of 0 stops the
sound playing on that channel. A note value 128-255 will trigger note-128 and immediately stop it (playing attack+release parts of envelope).

This function assumes the default setup, with the `sndGes` registers located at 0x00050.

### fn sndGes(sampleIndex: i32) -> f32

This implements a sound chip, generating sound based on 32 bytes of sound registers.

The spec of this sound chip are:

- 4 channels with individual volume control (0-15)
- rect, saw, tri, noise wave forms selectable per channel
- each wave form supports some kind of pulse width modulation
- each channel has an optional automatic low pass filter, or can be sent to one of two manually controllable filters
- each channel can select between a narrow and a wide stereo positioning. The two stereo positions of each channel are fixed.
- optional ring modulation

This function requires 1024 bytes of working memory, the first 32 bytes of which are interpreted as the sound registers.
The base address of its working memory can be configured by writing the address to 0x12c78. It defaults to 0x00050.

Here is a short description of the 32 sound registers.

```
00 - CTRL0
06 - CTRL1
0c - CTRL2
12 - CTRL3
  | 7  6 |   5  |   4  |  3  2  |    1    |    0    |
  | wave | ring | wide | filter | trigger | note on |

  note on: stay in decay/sustain part of envelope
  trigger: the attack part of the envlope is triggered when either this changes
           or note on is changed from 0 to 1.
  filter : 0 - no filter
           1 - fixed 6db 1-pole filter with cutoff two octaves above note
           2 - programmable filter 0
           3 - programmable filter 1
  wide   : use wide stereo panning
  ring   : ring modulate with triangle wave at frequency of previous channel
  wave   : 0 - rectangle
           1 - saw
           2 - triangle
           3 - noise

01 - PULS0
07 - PULS1
0d - PULS2
13 - PULS3
  Pulse width 0-255, with 0 being the plain version of each wave form.
  rectangle - 50%-100% pulse width
  saw       - inverts 0%-100% of the saw wave form around the center
  triangle  - morphs into an octave up triangle wave
  noise     - blends into a decimated saw wave (just try it out)

02 - FINE0
08 - FINE1
0e - FINE2
14 - FINE3
  Fractional note

03 - NOTE0
09 - NOTE1
0f - NOTE2
15 - NOTE3
  Note, 69 = A4

04 - ENVA0
0a - ENVA1
10 - ENVA2
16 - ENVA3
  | 7 6 5 4 | 3 2 1 0 |
  | decay   | attack  |

05 - ENVB0
0b - ENVB1
11 - ENVB2
17 - ENVB3
  | 7 6 5 4 | 3 2 1 0 |
  | release | sustain |

18 - VO01
  | 7 6 5 4  | 3 2 1 0  |
  | volume 1 | volume 0 |

19 - VO23
  | 7 6 5 4  | 3 2 1 0  |
  | volume 3 | volume 2 |

1a - FCTR0
1b - FCTR1
  | 7 6 5 4   | 3 | 2    | 1    | 0   |
  | resonance | 0 | band | high | low |

1c - FFIN0
1e - FFIN1
  cutoff frequency - fractional note

1d - FNOT0
1f - FNOT1
  cutoff frequency - note
```

# The `uw8` tool

The `uw8` tool included in the MicroW8 download includes a number of useful tools for developing MicroW8 carts. For small productions written in
wat or CurlyWas you don't need anything apart from `uw8` and a text editor of your choice.

## `uw8 run`

Usage:

`uw8 run [<options>] <file>`

Runs `<file>` which can be a binary WebAssembly module, an `.uw8` cart, a wat (WebAssembly text format) source file or a [CurlyWas](https://github.com/exoticorn/curlywas) source file.

Options:

* `-b`, `--browser`: Run in browser instead of using native runtime
* `-t FRAMES`, `--timeout FRAMES`: Sets the timeout in frames (1/60s). If the start or update function runs longer than this it is forcibly interupted
and execution of the cart is stopped. Defaults to 30 (0.5s)
* `-w`, `--watch`: Reloads the given file every time it changes on disk.
* `-p`, `--pack`: Pack the file into an `.uw8` cart before running it and print the resulting size.
* `-u`, `--uncompressed`: Use the uncompressed `uw8` format for packing.
* `-l LEVEL`, `--level LEVEL`: Compression level (0-9). Higher compression levels are really slow.
* `-o FILE`, `--output FILE`: Write the loaded and optionally packed cart back to disk.

when using the native runtime:

* `-m`, `--no-audio`: Disable audio, also reduces cpu load a bit
* `--no-gpu`:  Force old cpu-only window code
* `--filter FILTER`:  Select an upscale filter at startup
* `--fullscreen`:  Start in fullscreen mode

Note that the cpu-only window does not support fullscreen nor upscale filters.

Unless --no-gpu is given, uw8 will first try to open a gpu accelerated window, falling back to the old cpu-only window if that fails.
Therefore you should rarely need to manually pass --no-gpu. If you prefer the old pixel doubling look to the now default crt filter,
you can just pass `--filter nearest` or `--filter 1`.

The upscale filter options are:
```
1, nearest              : Anti-aliased nearest filter
2, fast_crt             : Very simple, cheap crt filter, not very good below a window size of 960x720
3, ss_crt               : Super sampled crt filter, a little more demanding on the GPU but scales well to smaller window sizes
4, chromatic_crt        : Variant of fast_crt with a slight offset of the three color dots of a pixel, still pretty cheap
5, auto_crt (default)   : ss_crt below 960x720, chromatic_crt otherwise
```

You can switch the upscale filter at any time using the keys 1-5. You can toggle fullscreen with F.

## `uw8 pack`

Usage:

`uw8 pack [<options>] <infile> <outfile>`

Packs the WebAssembly module or text file, or [CurlyWas](https://github.com/exoticorn/curlywas) source file into a `.uw8` cart.

Options:

* `-u`, `--uncompressed`: Use the uncompressed `uw8` format for packing.
* `-l LEVEL`, `--level LEVEL`: Compression level (0-9). Higher compression levels are really slow.

## `uw8 unpack`

Usage:

`uw8 unpack <infile> <outfile>`

Unpacks a MicroW8 module into a standard WebAssembly module.

## `uw8 compile`

Usage:

`uw8 compile [<options>] <infile> <outfile>`

Compiles a [CurlyWas](https://github.com/exoticorn/curlywas) source file to a standard WebAssembly module. Most useful together with
the `--debug` option to get a module that works well in the Chrome debugger.

Options:

* `-d`, `--debug`: Generate a name section to help debugging

## `uw8 filter-exports`

Usage:

`uw8 filter-exports <infile> <outfile>`

Reads a binary WebAssembly module, removes all exports not used by the MicroW8 platform + everything that is unreachable without those exports and writes the resulting module to `outfile`.

When compiling C code (or Rust, zig or others) to WebAssembly, you end up with a few exported global variables that are used for managing the heap and C stack, even if the code doesn't actually use those features. You can use this command to automatically remove them and gain a few bytes. See the C, Rust and zig examples in the MicroW8 repository.

# Other useful tools

The [Web Assembly Binary Toolkit](https://github.com/WebAssembly/wabt) includes
a few useful tools, eg. `wat2wasm` to compile the WebAssemby text format to binary
wasm and `wasm2wat` to disassemble wasm binaries.

[Binaryen](https://github.com/WebAssembly/binaryen) includes `wasm-opt` which enable additional optimizations over what LLVM (the backend that is used by most compilers that target WebAssembly) can do.

# Distribution

The classical distribution option is just to put the `.uw8` cart into a zip file, let people run it themselves, either in the `uw8` tool or in the web runtime.

If you want to go this way, you might consider including `microw8.html` in your download. It's specifically designed to be a small (~10KB at the moment), self-contained HTML file for just this reason. That way, anyone who has downloaded you production can run it, even when offline, provided they have a modern web browser at hand. Also, should future versions of MicroW8 ever introduce any kind of incompatibilities, they'd still have a compatible version right there without hunting arround for an old version.

## Base64 encoded link

For small productions (<= 1024 bytes), when you load them in the web runtime, the URL is automatically updated to include the cart as base64 encoded data. You can just give that URL to others for them to run your prod.

## url parameter

Another option is to put the cart on a webserver and add `#url=url/to/the/cart.uw8` to the end of the web runtime URL. ([Like this](../v0.1pre5#url=../uw8/skipahead.uw8))

If the cart and the web runtime are on different domains, you'll have to make sure that [CORS header](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS#the_http_response_headers) are enabled for the cart, otherwise the web runtime won't be able to load it.

Feel free to put the web runtime on your own server if it makes sense to you, its [license](https://unlicense.org/) allows you to do anything you want with it.

## `.html` + `.uw8`

At startup the web runtime will try to load a cart in the same directory as the `.html` file. If the URL of the web runtime ends in `.html` it will try to load a cart with the same name and the extension `.uw8`. If the URL of the web runtime ends in a `/` it will try to load a `cart.uw8` at that location.

So, you could for example serve the web runtime as `https://example.org/mytunnel.html` and the cart as `https://example.org/mytunnel.uw8` and send people to the HTML page to run the cart. Or you could put them up as `https://example.org/mytunnel/index.html` and `https://example.org/mytunnel/cart.uw8` and send people to `https://example.org/mytunnel`.

If a cart is found and loaded in this way, the load button is hidden.

## Itch.io

The above `.html` + `.uw8` option works great on [Itch.io](https://itch.io) as well. Put these two files into a zip archive:

* `index.html`: a copy of the web runtime (`microw8.html` in the MicroW8 download)
* `index.uw8`: Your game cart

Upload the zip file to itch.io and make sure to set the embedded viewport size to exactly (!) 640x480 pixel. At that exact size the web runtime hides everything except for the MicroW8 screen.

If instead you actually *want* to display the border around the screen and the byte size you can try a size of about 720x620.

[See here for an example upload.](https://exoticorn.itch.io/skipahead)

# `.uw8` format

The first byte of the file specifies the format version:

## Format version `00`:

This file is simply a standard WebAssembly module

## Format version `01`:

The rest of this file is the same as a WebAssembly
module with the 8 byte header removed. This module
can leave out sections which are then taken from
a base module provided by MicroW8.

You can generate this base module yourself using
`uw8-tool`. As a quick summary, it provides all function
types with up to 7 parameters (i32 or f32) where the
`f32` parameters always preceed the `i32` parameters.
Then it includes all imports that MicroW8 provides,
a function section with a single function of type
`() -> void` and an export section that exports
the first function in the file under the name `upd`.

## Format version `02`:

Same as version `01` except everything after the first byte is compressed
using a [custom LZ compression scheme](https://github.com/exoticorn/upkr).

# The web runtime

Load carts into the web runtime either by using the "Load cart..." button, or by dragging the file
onto the screen area.

## Input

For input, you can either use a standard gamepad or keyboard. On a keyboard use the arrow keys and the keys Z, X, A and S to emulate the A, B, X and Y buttons.

## Video recording

Press F10 to start recording, press again to stop. Then a download dialog will open for the video file.
The file might miss some metadata needed to load in some video editing tools, in that case you can run
it through ffmpeg like this `ffmpeg -i IN_NAME.webm -c copy -o OUT_NAME.webm to fix it up.

To convert it to 1280x720, for example for a lovebyte upload, you can use:

```
ffmpeg -i IN.webm -vf "scale=960:720:flags=neighbor,pad=1280:720:160:0" -r 60 OUT.mp4
```

## Screenshot

Pressing F9 opens a download dialog with a screenshot.

## Devkit mode

Append `#devkit` to the web runtime url in order to switch to devkit mode. In devkit mode, standard web assembly modules
are loaded bypassing the loader, removing all size restrictions. At the same time, the memory limit is increased to 1GB.
