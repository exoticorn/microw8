+++
title = "Docs"
description = "Docs"
+++

# Overview

MicroW8 loads WebAssembly modules with a maximum size of 256kb. You module needs to export
a function `fn upd()` which will be called once per frame.
After calling `upd` MicroW8 will display the 320x240 8bpp framebuffer located
at offset 120 in memory with the 32bpp palette located at 0x13000.

The memory has to be imported as `env` `memory` and has a maximum size of 256kb (4 pages).

# Memory map

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

### fn atan2(y: f32, y: f32) -> f32

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

### fn rectangle_outline(x: f32, y: f32, w: f32, h: f32, color: i32)

Draws a one pixel outline on the inside of the given rectangle.

(Draws the outermost pixels that are still inside the rectangle area.)

### fn circle_outline(cx: f32, cy: f32, radius: f32, color: i32)

Draws a one pixel outline on the inside of the given circle.

(Draws the outermost pixels that are still inside the circle area.)

### fn line(x1: f32, y1: f32, x2: f32, y2: f32, color: i32)

Draws a line from `x1,y1` to `x2,y2` in the given color index.

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

### fn printChar(char: i32)
### fn printString(ptr: i32)
### fn printInt(num: i32)
### fn setTextColor(color: i32)
### fn setBackgroundColor(color: i32)
### fn setCursorPosition(x: i32, y: i32)

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
types with up to 5 parameters (i32 or f32) where the
`f32` parameters always preceed the `i32` parameters.
Then it includes all imports that MicroW8 provides,
a function section with a single function of type
`() -> void` and an export section that exports
the first function in the file under the name `upd`.

## Format version `02`:

Same as version `01` except everything after the first byte is compressed
using a [custom LZ compression scheme](https://github.com/exoticorn/upkr).
