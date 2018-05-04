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
//! ## Working directly with constants
//! Let's say we want to print out the first character belonging to the
//! greek subset. In this case, it corresponds to the unicode `U+0390` described as `iota with
//! tonos and diaeresis`, and we will retrieve it from the `GREEK` constant provided by our library.
//!
//! Specifically, we will be working with `GREEK[0]`, which is an array of bytes with capacity for
//! eight separate bytes (`[u8; 8]`).
//!
//! Here's a program that will print the character to your terminal, by parsing each byte of the
//! array, inspecting it bit by bit, and printing an empty space `" "` for `0`, and `"█"` for `1`.
//!
//! ```rust,norun
//! extern crate font8x8;
//!
//! use font8x8::GREEK_LEGACY;
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
mod basic;
mod block;
#[path = "box.rs"]
mod box_chars;
mod control;
mod greek;
mod hiragana;
mod latin;
/// Re-export the original `[u8; 8]` constants, taken from C-header files.
pub mod legacy;
mod misc;
mod sga;

#[cfg(feature = "unicode")]
pub use self::basic::BASIC_UNICODE;
pub mod utf16 {
    //! `utf16` support for fonts.
    pub use std::string::FromUtf16Error;

    /// A single 8x8 font which supports `UTF-16` encoding/decoding.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct FontUtf16(pub u16, pub [u8; 8]);

    impl FontUtf16 {
        /// Return the utf16 value as `u16`.
        pub fn utf16(&self) -> u16 {
            self.0
        }
        /// Return the `[u8; 8]`-representation for this font.
        pub fn byte_array(&self) -> [u8; 8] {
            self.1
        }
        /// Return a result with the corresponding `String` for the font.
        pub fn to_string(&self) -> String {
            String::from_utf16(&[self.0]).unwrap()
        }

        /// Returns a `bool` indicating whether this font renders as a whitespace (all `0`).
        pub fn is_whitespace(&self) -> bool {
            self.1 == NOTHING_TO_DISPLAY
        }
    }

    impl Into<u16> for FontUtf16 {
        fn into(self) -> u16 {
            self.0
        }
    }

    impl Into<[u8; 8]> for FontUtf16 {
        fn into(self) -> [u8; 8] {
            self.1
        }
    }

    /// A trait for collections of `FontUtf16`, which provide methods for retrieving
    /// the `[u8; 8]` by key, using the corresponding Unicode value, encoded as UTF-16 (`u16`).
    pub trait Utf16Fonts {
        fn get(&self, key: u16) -> Option<[u8; 8]>;
        fn get_font(&self, key: u16) -> Option<FontUtf16>;
        fn print_set(&self);
    }
}

use self::legacy::NOTHING_TO_DISPLAY;

#[cfg(feature = "unicode")]
pub use self::control::CONTROL_UNICODE;

#[cfg(feature = "unicode")]
pub use self::latin::LATIN_UNICODE;

#[cfg(feature = "unicode")]
pub use self::greek::GREEK_UNICODE;

#[cfg(feature = "unicode")]
pub use self::block::BLOCK_UNICODE;

#[cfg(feature = "unicode")]
pub use self::box_chars::BOX_UNICODE;

#[cfg(feature = "unicode")]
pub use self::hiragana::HIRAGANA_UNICODE;

#[cfg(feature = "unicode")]
pub use self::misc::MISC_UNICODE;

#[cfg(feature = "unicode")]
pub use self::sga::SGA_UNICODE;

#[cfg(feature = "unicode")]
pub use self::utf16::{FontUtf16, FromUtf16Error, Utf16Fonts};
