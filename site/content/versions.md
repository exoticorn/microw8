+++
description = "Versions"
+++

### v0.4.1

* [Web runtime](../v0.4.1)
* [Linux](https://github.com/exoticorn/microw8/releases/download/v0.4.1/microw8-0.4.1-linux.tgz)
* [MacOS](https://github.com/exoticorn/microw8/releases/download/v0.4.1/microw8-0.4.1-macos.tgz)
* [Windows](https://github.com/exoticorn/microw8/releases/download/v0.4.1/microw8-0.4.1-windows.zip)

Changes:

* Windows: fix bad/inconsistent frame rate
* fix choppy sound on devices with sample rates != 44100 kHz
* add scale mode 'fill' option

### v0.4.0

* [Web runtime](../v0.4.0)
* [Linux](https://github.com/exoticorn/microw8/releases/download/v0.4.0/microw8-0.4.0-linux.tgz)
* [MacOS](https://github.com/exoticorn/microw8/releases/download/v0.4.0/microw8-0.4.0-macos.tgz)
* [Windows](https://github.com/exoticorn/microw8/releases/download/v0.4.0/microw8-0.4.0-windows.zip)

Changes:

* add support for sound on mono- and surround-only devices
* update wasmtime dependency to fix performance regression in 0.3.0
* add frame counter since module start at location 72
* add 6 and 7 parameter function types to base module

### v0.3.0

* [Web runtime](../v0.3.0)
* [Linux](https://github.com/exoticorn/microw8/releases/download/v0.3.0/microw8-0.3.0-linux.tgz)
* [MacOS](https://github.com/exoticorn/microw8/releases/download/v0.3.0/microw8-0.3.0-macos.tgz)
* [Windows](https://github.com/exoticorn/microw8/releases/download/v0.3.0/microw8-0.3.0-windows.zip)

Changes:

* add blitSprite and grabSprite API calls
* add support for integer scaling up to 16x for printing text
* fix incompatibility with sound devices only offering 16bit audio formats
* add support for br_table instruction in packed carts

### v0.2.2

* [Web runtime](../v0.2.2)
* [Linux](https://github.com/exoticorn/microw8/releases/download/v0.2.2/microw8-0.2.2-linux.tgz)
* [MacOS](https://github.com/exoticorn/microw8/releases/download/v0.2.2/microw8-0.2.2-macos.tgz)
* [Windows](https://github.com/exoticorn/microw8/releases/download/v0.2.2/microw8-0.2.2-windows.zip)

Changes:

* call `start` function after loading cart if the cart exports one
* fix `sndGes` having the wrong name and not being included in the auto imports
* fix control codes 4-6 (change text output mode) being invoked when used as parameters in other control sequences
* only open browser window once a cart was compiled sucessfully when running with `-b`

### v0.2.1

* [Web runtime](../v0.2.1)
* [Linux](https://github.com/exoticorn/microw8/releases/download/v0.2.1/microw8-0.2.1-linux.tgz)
* [MacOS](https://github.com/exoticorn/microw8/releases/download/v0.2.1/microw8-0.2.1-macos.tgz)
* [Windows](https://github.com/exoticorn/microw8/releases/download/v0.2.1/microw8-0.2.1-windows.zip)

Changes:

* new gpu accelerated renderer with (optional) crt filter
* optimized `hline` function, a big speed-up when drawing large filled circles or rectangles
* print fractional size of packed `uw8` cart

### v0.2.0

* [Web runtime](../v0.2.0)
* [Linux](https://github.com/exoticorn/microw8/releases/download/v0.2.0/microw8-0.2.0-linux.tgz)
* [MacOS](https://github.com/exoticorn/microw8/releases/download/v0.2.0/microw8-0.2.0-macos.tgz)
* [Windows](https://github.com/exoticorn/microw8/releases/download/v0.2.0/microw8-0.2.0-windows.zip)

Changes:

* [add sound support!](docs#sound)
* add support to redirect text output to the console for debugging using control code 6
* update curlywas:
  * add support for `else if`
  * add support for escape sequences in strings
  * add support for char literals
  * add support for binop-assignment, eg. `+=`, `^=`, `<<=` etc. (also support for the tee operator: `+:=`)
  * "integer constant cast to float" literal syntax in CurlyWas (ex. `1_f` is equivalent to `1 as f32`)

### v0.2.0-rc3

* [Web runtime](../v0.2.0-rc3)
* [Linux](https://github.com/exoticorn/microw8/releases/download/v0.2.0-rc3/microw8-0.2.0-rc3-linux.tgz)
* [MacOS](https://github.com/exoticorn/microw8/releases/download/v0.2.0-rc3/microw8-0.2.0-rc3-macos.tgz)
* [Windows](https://github.com/exoticorn/microw8/releases/download/v0.2.0-rc3/microw8-0.2.0-rc3-windows.zip)

Changes:

* improve timing stability some more. essentially now guaranteeing that "frame = time_ms * 6 / 100" returns
  consecutive frame numbers, provided the module can be run at 60 fps
* add support to redirect text output to the console for debugging using control code 6
* update curlywas:
  * add support for `else if`
  * add support for escape sequences in strings
  * add support for char literals
  * add support for binop-assignment, eg. `+=`, `^=`, `<<=` etc. (also support for the tee operator: `+:=`)

### v0.2.0-rc2

* [Web runtime](../v0.2.0-rc2)
* [Linux](https://github.com/exoticorn/microw8/releases/download/v0.2.0-rc2/microw8-0.2.0-rc2-linux.tgz)
* [MacOS](https://github.com/exoticorn/microw8/releases/download/v0.2.0-rc2/microw8-0.2.0-rc2-macos.tgz)
* [Windows](https://github.com/exoticorn/microw8/releases/download/v0.2.0-rc2/microw8-0.2.0-rc2-windows.zip)

Changes:

* fix timing issues of sound playback, especially on systems with large sound buffers

### v0.2.0-rc1

* [Web runtime](../v0.2.0-rc1)
* [Linux](https://github.com/exoticorn/microw8/releases/download/v0.2.0-rc1/microw8-0.2.0-rc1-linux.tgz)
* [MacOS](https://github.com/exoticorn/microw8/releases/download/v0.2.0-rc1/microw8-0.2.0-rc1-macos.tgz)
* [Windows](https://github.com/exoticorn/microw8/releases/download/v0.2.0-rc1/microw8-0.2.0-rc1-windows.zip)

Changes:

* [add sound support](docs#sound)
* "integer constant cast to float" literal syntax in CurlyWas (ex. `1_f` is equivalent to `1 as f32`)

Known issues:

* timing accuracy/update frequency of sound support currently depends on sound buffer size

### v0.1.2

* [Web runtime](../v0.1.2)
* [Linux](https://github.com/exoticorn/microw8/releases/download/v0.1.2/microw8-0.1.2-linux.tgz)
* [MacOS](https://github.com/exoticorn/microw8/releases/download/v0.1.2/microw8-0.1.2-macos.tgz)
* [Windows](https://github.com/exoticorn/microw8/releases/download/v0.1.2/microw8-0.1.2-windows.zip)

Changes:

* add option to `uw8 run` to run the cart in the browser using the web runtime
*../ CurlyWas: implement `include` support
* CurlyWas: implement support for constants
* fix crash when trying to draw zero sized line

### v0.1.1

* [Web runtime](../v0.1.1)
* [Linux](https://github.com/exoticorn/microw8/releases/download/v0.1.1/microw8-0.1.1-linux.tgz)
* [MacOS](https://github.com/exoticorn/microw8/releases/download/v0.1.1/microw8-0.1.1-macos.tgz)
* [Windows](https://github.com/exoticorn/microw8/releases/download/v0.1.1/microw8-0.1.1-windows.zip)

Changes:

* implement more robust file watcher
* add basic video recording on F10 in web runtime
*../ add screenshot on F9
* add watchdog to interrupt hanging update in native runtime
* add devkit mode to web runtime
*../ add unpack and compile commands to uw8
* add support for table/element section in pack command
* disable wayland support (caused missing window decorations in gnome)

### v0.1.0

* [Web runtime](../v0.1.0)
* [Linux](https://github.com/exoticorn/microw8/releases/download/v0.1.0/microw8-0.1.0-linux.tgz)
* [MacOS](https://github.com/exoticorn/microw8/releases/download/v0.1.0/microw8-0.1.0-macos.tgz)
* [Windows](https://github.com/exoticorn/microw8/releases/download/v0.1.0/microw8-0.1.0-windows.zip)
