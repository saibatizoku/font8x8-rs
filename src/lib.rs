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
pub mod utf16 {
    //! `utf16` support for fonts.
    use super::legacy::NOTHING_TO_DISPLAY;
    pub use super::basic::{BASIC_UTF16, BasicFonts};
    pub use super::block::{BLOCK_UTF16, BlockFonts};
    pub use super::box_chars::{BOX_UTF16, BoxFonts};
    pub use super::greek::{GREEK_UTF16, GreekFonts};
    pub use super::hiragana::{HIRAGANA_UTF16, HiraganaFonts};
    pub use super::latin::{LATIN_UTF16, LatinFonts};
    pub use super::misc::{MISC_UTF16, MiscFonts};
    pub use super::sga::{SGA_UTF16, SgaFonts};
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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn font_utf16_converts_into_u16() {
            let my_font = FontUtf16('á' as u16, [110u8; 8]);
            let utf16: u16 = my_font.into();
            assert_eq!(utf16, 'á' as u16);
        }

        #[test]
        fn font_utf16_converts_into_byte_array() {
            let my_font = FontUtf16('C' as u16, NOTHING_TO_DISPLAY);
            let byte_array: [u8; 8] = my_font.into();
            assert_eq!(byte_array, NOTHING_TO_DISPLAY);
        }

        #[test]
        fn font_utf16_converts_into_inner_tuple() {
            let my_font = FontUtf16('Á' as u16, [110u8; 8]);
            assert_eq!(my_font.into_inner(), ('Á' as u16, [110u8; 8]));
        }
    }
}

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
pub use self::utf16::{FontUtf16, FromUtf16Error, Utf16Fonts};
