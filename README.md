# MicroW8

MicroW8 is a WebAssembly based fantasy console inspired by the likes of [TIC-80](https://tic80.com/), [WASM-4](https://wasm4.org/) and [PICO-8](https://www.lexaloffle.com/pico-8.php).

The initial motivation behind MicroW8 was to explore whether there was a way to make WebAssembly viable for size-coding. (Size coding being the art of creating tiny (often <= 256 bytes) graphical effects and games.) The available examples so far are all in this space, however, I very carefully made sure that all design decisions make sense from the point of view of bigger projects as well.

See [here](https://exoticorn.github.io/microw8/) for more information and docs.

## Specs

* Screen: 320x240, 256 colors, 60Hz
* Modules: Up to 256KB (WASM)
* Memory: 256KB
* Gamepad input (D-Pad + 4 Buttons)

## Downloads

* [Linux](https://github.com/exoticorn/microw8/releases/download/v0.4.1/microw8-0.4.1-linux.tgz)
* [MacOS](https://github.com/exoticorn/microw8/releases/download/v0.4.1/microw8-0.4.1-macos.tgz)
* [Windows](https://github.com/exoticorn/microw8/releases/download/v0.4.1/microw8-0.4.1-windows.zip)

The download includes

* `microw8.html`: The web runtime, a small, self-contained html file that can be opened in any modern browser to load and run MicroW8 carts.
* `uw8`/`uw8.exe`: The MicroW8 dev tool, including a native runtime.
* `examples`: Example source code in CurlyWas and Wat (WebAssembly text format).
* `carts`: The examples compiled to `.uw8` carts.

## uw8 dev tool

```
uw8 run [<options>] <file>

Runs <file> which can be a binary WebAssembly module, an `.uw8` cart, a wat (WebAssembly text format) source file or a CurlyWas source file.

Options:

-b, --browser           : Run in browser instead of using native runtime
-t, --timeout FRAMES    : Sets the timeout in frames (1/60s)
-w, --watch             : Reloads the given file every time it changes on disk.
-p, --pack              : Pack the file into an .uw8 cart before running it and print the resulting size.
-u, --uncompressed      : Use the uncompressed uw8 format for packing.
-l LEVEL, --level LEVEL : Compression level (0-9). Higher compression levels are really slow.
-o FILE, --output FILE  : Write the loaded and optionally packed cart back to disk.

when using the native runtime:

-m, --no-audio          : Disable audio, also reduces cpu load a bit
--no-gpu                : Force old cpu-only window code
--filter FILTER         : Select an upscale filter at startup
--fullscreen            : Start in fullscreen mode

Note that the cpu-only window does not support fullscreen nor upscale filters.

Unless --no-gpu is given, uw8 will first try to open a gpu accelerated window, falling back to the old cpu-only window if that fails.
Therefore you should rarely need to manually pass --no-gpu. If you prefer the old pixel doubling look to the now default crt filter,
you can just pass "--filter nearest" or "--filter 1".

The upscale filter options are:
1, nearest              : Anti-aliased nearest filter
2, fast_crt             : Very simple, cheap crt filter, not very good below a window size of 960x720
3, ss_crt               : Super sampled crt filter, a little more demanding on the GPU but scales well to smaller window sizes
4, chromatic_crt        : Variant of fast_crt with a slight offset of the three color dots of a pixel, still pretty cheap
5, auto_crt (default)   : ss_crt below 960x720, chromatic_crt otherwise

You can switch the upscale filter at any time using the keys 1-5. You can toggle fullscreen with F.

uw8 pack [<options>] <infile> <outfile>

Packs the WebAssembly module or text file, or CurlyWas source file into a .uw8 cart.

Options:

-u, --uncompressed      : Use the uncompressed uw8 format for packing.
-l LEVEL, --level LEVEL : Compression level (0-9). Higher compression levels are really slow.


uw8 unpack <infile> <outfile>

Unpacks a MicroW8 module into a standard WebAssembly module.


uw8 compile [<options>] <infile> <outfile>

Compiles a CurlyWas source file to a standard WebAssembly module. Most useful together with
the --debug option to get a module that works well in the Chrome debugger.

Options:

-d, --debug             : Generate a name section to help debugging


uw8 filter-exports <infile> <outfile>

Reads a binary WebAssembly module, removes all exports not used by the MicroW8 platform + everything that is unreachable without those exports and writes the resulting module to <outfile>.
```

## Examples

* [Fireworks](https://exoticorn.github.io/microw8/v0.1pre5#AgwvgP+M59snqjl4CMKw5sqm1Zw9yJCbSviMjeLUdHus2a3yl/a99+uiBeqZgP/2jqSjrLjRk73COMM6OSLpsxK8ugT1kuk/q4hQUqqPpGozHoa0laulzGGcahzdfdJsYaK1sIdeIYS9M5PnJx/Wk9H+PvWEPy2Zvv7I6IW7Fg==) (127 bytes): Some fireworks to welcome 2022.
* [Skip Ahead](https://exoticorn.github.io/microw8/v0.1pre5#AgyfpZ80wkW28kiUZ9VIK4v+RPnVxqjK1dz2BcDoNyQPsS2g4OgEzkTe6jyoAfFOmqKrS8SM2aRljBal9mjNn8i4fP9eBK+RehQKxxGtJa9FqftvqEnh3ez1YaYxqj7jgTdzJ/WAYVmKMovBT1myrX3FamqKSOgMsNedLhVTLAhQup3sNcYEjGNo8b0HZ5+AgMgCwYRGCe//XQOMAaAAzqDILgmpEZ/43RKHcQpHEQwbURfNQJpadJe2sz3q5FlQnTGXQ9oSMokidhlC+aR/IpNHieuBGLhFZ2GfnwVQ0geBbQpTPA==) (229 bytes): A port of my [TIC-80 256byte game](http://tic80.com/play?cart=1735) from LoveByte'21
* [OhNoAnotherTunnel](https://exoticorn.github.io/microw8/v0.1pre4#Ag95rdCB5Ww5NofyQaKF4P1mrNRso4azgiem4hK99Gh8OMzSpFq3NsNDo7O7pqln10D11l9uXr/ritw7OEzKwbEfCdvaRnS2Z0Kz0iDEZt/gIqOdvFmxsL1MjPQ4XInPbUJpQUonhQq29oP2omFabnQxn0bzoK7mZjcwc5GetHG+hGajkJcRr8oOnjfCol8RD+ha33GYtPnut+GLe4ktzf5UxZwGs6oT9qqC61lRDakN) (177 bytes): A port of my [entry](http://tic80.com/play?cart=1871) in the Outline'21 bytebattle final
* [Technotunnel](https://exoticorn.github.io/microw8/v0.1pre4#AqL8HeK1M9dn2nWNIF5vaq/Vh64pMt5nJIFoFKpBMPUsGtDtpqjo1JbT9LzPhAxCqJ7Yh4TA6oTGd4xhLowf+cWZMY73+7AZmfXJJsBi4cej/hH+4wlAgxFIrnOYnr/18IpnZbsHf0eGm1BhahX74+cVR0TRmNQmYC7GhCNS3mv/3MJn74lCj7t28aBJPjEZhP9fGXdG2u5Egh/Tjdg=) (158 bytes): A port of my [entry](https://tic80.com/play?cart=1873) in the Outline'21 bytebattle quater final
