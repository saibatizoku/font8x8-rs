//! # 8x8 monochrome bitmap fonts for rendering
//!
//! This crate contains a Rust implementation of a public domain 8-bit by 8-bit font.
//!
//! # Usage
//!
//! To use this crate, add to your `Cargo.toml`:
//!
//! ```Cargo
//! [dependencies]
//! font8x8 = "0.1"
//! ```
//!
//! Then, in your code:
//!
//! ```rust,norun
//! extern crate font8x8;
//!
//! use font8x8;
//! ```
//!
//! # Example
//!
//! ## Working directly with legacy constants
//!
//! Let's say we want to print out the first character belonging to the
//! greek subset. In this case, it corresponds to the unicode `U+0390` described as `iota with
//! tonos and diaeresis`, and we will retrieve it from the `GREEK_LEGACY` constant provided by our library.
//!
//! Specifically, we will be working with `GREEK_LEGACY[0]`, which is an array of bytes with capacity for
//! eight separate bytes (`[u8; 8]`).
//!
//! Here's a program that will print the character to your terminal, by parsing each byte of the
//! array, inspecting it bit by bit, and printing an empty space `" "` for `0`, and `"█"` for `1`.
//!
//! ```rust
//! extern crate font8x8;
//!
//! use font8x8::legacy::GREEK_LEGACY;
//!
//! fn main() {
//!     for x in &GREEK_LEGACY[0] {
//!         for bit in 0..8 {
//!             match *x & 1 << bit {
//!                 0 => print!(" "),
//!                 _ => print!("█"),
//!             }
//!         }
//!         println!()
//!     }
//! }
//! ```
//!
//! The generated output should mostly resemble this (it will depend on your terminal's font settings):
//! ```text
//!  █ ██ █
//!
//!    ██
//!    ██
//!    ██
//!    ██ █
//!     ██
//! ```
//!
//! and, it's meant to look like this: `ΐ`.
//!
//! ## Working with fonts as UTF16
//!
//! We can also use UTF16-encoded text to render the font on stdout.
//!
//! This time, instead of using the index of the GREEK_LEGACY constant, we can use the trait method `Utf16Fonts::get` to retrieve the font rendering using the `u16` as key.
//!
//! ```rust
//! extern crate font8x8;
//!
//! use font8x8::{GREEK_FONTS, Utf16Fonts};
//!
//! fn main() {
//!     if let Some(font_render) = GREEK_FONTS.get('ΐ' as u16) {
//!         for x in &font_render {
//!             for bit in 0..8 {
//!                 match *x & 1 << bit {
//!                     0 => print!(" "),
//!                     _ => print!("█"),
//!                 }
//!             }
//!             println!()
//!         }
//!     }
//! }
//! ```
//!
//! # On the miscellanous characters included
//! These characters are provided as-is, and some don't have a proper unicode point.
//! They are provided in the `MISC` and `SGA` contants. For a description, please
//! read the constant documentation.
//!
//! # Credits
//! Initially extracted from an `asm` file, fetched from a now broken link:
//! `http://dimensionalrift.homelinux.net/combuster/mos3/?p=viewsource&file=/modules/gfx/font8_8.asm`
//!
//! It was then ported to a C header file, also in the Public Domain,
//! [https://github.com/dhepper/font8x8](https://github.com/dhepper/font8x8).
//!
//! This crate is an extension of that work.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "unicode")]
mod basic;
#[cfg(feature = "unicode")]
mod block;
#[cfg(feature = "unicode")]
#[path = "box.rs"]
mod box_chars;
#[cfg(feature = "unicode")]
mod greek;
#[cfg(feature = "unicode")]
mod hiragana;
#[cfg(feature = "unicode")]
mod latin;
/// Re-export the original `[u8; 8]` constants, taken from C-header files.
pub mod legacy;
#[cfg(feature = "unicode")]
mod misc;
#[cfg(feature = "unicode")]
mod sga;

#[cfg(feature = "unicode")]
pub mod utf16;

#[cfg(feature = "unicode")]
pub use self::basic::BASIC_FONTS;

#[cfg(feature = "unicode")]
pub use self::latin::LATIN_FONTS;

#[cfg(feature = "unicode")]
pub use self::greek::GREEK_FONTS;

#[cfg(feature = "unicode")]
pub use self::block::BLOCK_FONTS;

#[cfg(feature = "unicode")]
pub use self::box_chars::BOX_FONTS;

#[cfg(feature = "unicode")]
pub use self::hiragana::HIRAGANA_FONTS;

#[cfg(feature = "unicode")]
pub use self::misc::MISC_FONTS;

#[cfg(feature = "unicode")]
pub use self::sga::SGA_FONTS;

#[cfg(feature = "unicode")]
pub use self::utf16::{FontUtf16, Utf16Fonts};
#[cfg(all(feature = "unicode", feature = "std"))]
pub use self::utf16::FromUtf16Error;

#[cfg(feature = "std")]
mod core {
    pub use std::*;
}
