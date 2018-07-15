8x8 monochrome bitmap font for rendering. Implemented in Rust.
==============================================================

[![crates.io](https://img.shields.io/crates/v/font8x8.svg)](https://crates.io/crates/font8x8)
[![docs](https://docs.rs/font8x8/badge.svg)](https://docs.rs/font8x8)

A collection of unicode characters in a 8x8 bitmap font.

# Usage

Add this to your `Cargo.toml`:
```cargo
[dependencies]
font8x8 = "0.1"
```

## Legacy constants
You can use the constants provided in the legacy software.

```rust
extern crate font8x8

use font8x8::legacy::BASIC_LEGACY;    // U+0000 - U+007F
use font8x8::legacy::CONTROL_LEGACY;  // U+0080 - U+009F
use font8x8::legacy::LATIN_LEGACY;    // U+00A0 - U+00FF

use font8x8::legacy::BOX_LEGACY;      // U+2500 - U+257F
use font8x8::legacy::BLOCK_LEGACY;    // U+2580 - U+259F
use font8x8::legacy::HIRAGANA_LEGACY; // U+3040 - U+309F
use font8x8::legacy::GREEK_LEGACY;    // U+0390 - U+039C

use font8x8::legacy::MISC_LEGACY;     // U+20A7, U+0192, U+00AA, U+00BA,
                       // U+2310, U+2264, U+2265, U+0060,
                       // U+1EF2, U+1EF3

use font8x8::legacy::SGA_LEGACY;      // U+E541 - U+E55A

```

# Documentation

To generate the crate's documentation, you can use:

`cargo doc -p font8x8 --no-deps --open`

from the terminal.

It's highly recommended that you inspect the docs for each constant, as there is a listing
of the included characters. Better yet, dive into the source, it's pretty straightforward.

# Example

## Working directly with legacy constants
Let's say we want to print out the first character belonging to the
greek subset. In this case, it corresponds to the unicode `U+0390` described as `iota with
tonos and diaeresis`, and we will retrieve it from the `GREEK_LEGACY` constant provided by our library.

Specifically, we will be working with `GREEK_LEGACY[0]`, which is an array of bytes with capacity for
eight separate bytes (`[u8; 8]`).

Here's a program that will print the character to your terminal, by parsing each byte of the
array, inspecting it bit by bit, and printing an empty space `" "` for `0`, and `"█"` for `1`.

```rust
extern crate font8x8;

use font8x8::legacy::GREEK_LEGACY;

fn main() {
    for x in &GREEK_LEGACY[0] {
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


## Working with fonts as UTF16

We can also use UTF16-encoded text to render the font on stdout.

This time, instead of using the index of the GREEK_LEGACY constant, we can use the trait method `Utf16Fonts::get` to retrieve the font rendering using the `u16` as key.

```rust
extern crate font8x8;

use font8x8::{GREEK_FONTS, Utf16Fonts};

fn main() {
    if let Some(font_render) = GREEK_FONTS.get('ΐ' as u16) {
        for x in &font_render {
            for bit in 0..8 {
                match *x & 1 << bit {
                    0 => print!(" "),
                    _ => print!("█"),
                }
            }
            println!()
        }
    }
}
```

Features
========

## `default`

The default features include `unicode`, and `std`.

For information about using none or some of the features, please consult the [features section of the Cargo Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section).

## `unicode`

Provides a wrapper for the font constants, tupled with their corresponding unicode point.

For more information, please refer to the [utf16 module documentation](https://docs.rs/font8x8/0.1.6/font8x8/utf16/index.html).

## `std`

Enables the use of the `std` Rust library. Disable this feature to run with `no_std` programs.


Credits
=======

Summary: 8x8 monochrome bitmap fonts for rendering. Implemented in Rust.

Author(s):

* Please read the `Cargo.toml` manifest for a list of the authors.

License: MIT License.

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
