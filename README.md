8x8 monochrome bitmap font for rendering. Implemented in Rust.
==============================================================

A collection of unicode characters in a 8x8 bitmap font.

# Usage

Add this to your `Cargo.toml`:
```cargo
[dependencies]
font8x8 = "0.1"
```

Then you can use it with your crate:

```rust
extern crate font8x8

use font8x8::BASIC; // U+0000 - U+007F
use font8x8::CONTROL; // U+0080 - U+009F
use font8x8::LATIN; // U+00A0 - U+00FF

use font8x8::BOX; // U+2500 - U+257F
use font8x8::BLOCK; // U+2580 - U+259F
use font8x8::HIRAGANA; // U+3040 - U+309F
use font8x8::GREEK; // U+0390 - U+039C

use font8x8::MISC; // U+____ - U+____
use font8x8::SGA; // U+____ - U+____
```

# Documentation

To generate the crate's documentation, you can use:

`cargo doc -p font8x8 --no-deps --open`

from the terminal.

It's highly recommended that you inspect the docs for each constant, as there is a listing
of the included characters.

# Example

## Working directly with constants
Let's say we want to print out the first character belonging to the
greek subset. In this case, it corresponds to the unicode `U+0390` described as `iota with
tonos and diaeresis`, and we will retrieve it from the `GREEK` constant provided by our library.

Specifically, we will be working with `GREEK[0]`, which is an array of bytes with capacity for
eight separate bytes (`[u8; 8]`).

Here's a program that will print the character to your terminal, by parsing each byte of the
array, inspecting it bit by bit, and printing an empty space `" "` for `0`, and `"█"` for `1`.

```rust
extern crate font8x8;

use font8x8::GREEK;

fn main() {
    for x in &GREEK[0] {
        for bit in 0..8 {
            match *x & 1 << bit {
                0 => print!(" "),
                _ => print!("█"),
            }
        }
        println!()
    }
}
```

The generated output should mostly resemble this (it will depend on your terminal's font settings):
```text
 █ ██ █  
         
   ██    
   ██    
   ██    
   ██ █  
    ██   
```

and, it's meant to look like this: `ΐ`.

Credits
=======

Summary: 8x8 monochrome bitmap fonts for rendering. Implemented in Rust.

Author: Joaquín Rosales <globojorro@gmail.com>

License: Public Domain

These header files are directly derived from an assembler file fetched from:
`http://dimensionalrift.homelinux.net/combuster/mos3/?p=viewsource&file=/modules/gfx/font8_8.asm`

Original header:

```
; Summary: `font8_8.asm`
; 8x8 monochrome bitmap fonts for rendering
;
; Author:
;     Marcel Sondaar
;     International Business Machines (public domain VGA fonts)
;
; License:
;     Public Domain
;
```

C header:

```
; Summary: `font8x8.h`
; 8x8 monochrome bitmap fonts for rendering
;
; https://github.com/dhepper/font8x8
;
; Author: Daniel Hepper <daniel@hepper.net>
; License: Public Domain
```
