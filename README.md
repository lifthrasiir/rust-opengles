# rust-opengles-angle

This is a version of
[rust-opengles](https://github.com/mozilla-servo/rust-opengles) bundled with the
[ANGLE](http://code.google.com/p/angleproject/) library for Windows. Compiling
rust-opengles-angle produces two additional DLL files `libGLESv2.dll` and
`libEGL.dll` that ensure the portable OpenGL ES support.

Currently rust-opengles-angle uses ANGLE version 1.0.0.2042 with local patches
by Mozilla (`mozilla-central` hg rev e9b37726c020). This complication is because
ANGLE by default does not support MinGW and Mozilla had a working version of
ANGLE with Makefile. Ultimately, `Makefile.in` and `moz.build` are removed in
order to be MPL-free and I have rolled my own Makefile. Except for this removal
and local patches (`angle/angle-rust-opengles-angle-local.patch`) the code
remains intact.

## Compilation

In theory, MinGW and `./configure && make` should work. Unfortunately ANGLE uses
several header files not present in MinGW w32api headers so we need a twist: if
you ever note that some header file is missing, then you should bring it to the
separate directory and run make with `DIRECTX_SDK_INCLUDES=/path/to/headers`.

I have prepared a [separate header
package](https://github.com/lifthrasiir/w32api-directx-standalone) for this very
reason; it couldn't be included as is due to its GNU LGPL licensing.

## License

Every modification and addition (including Makefile) to ANGLE is licensed under
the BSD-style license at `angle/LICENSE`; every modification and addition to
rust-opengles is licensed under the Apache License, Version 2.0
(`LICENSE-APACHE`) or the MIT license (`LICENSE-MIT`).

