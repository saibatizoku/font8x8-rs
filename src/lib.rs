//! # 8x8 monochrome bitmap fonts for rendering
//!
//! This crate contains a Rust implementation of a public domain 8-bit by 8-bit font.
//!
//! See '../README.md`.
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
pub mod unicode;

#[cfg(feature = "unicode")]
pub use self::basic::{BASIC_FONTS, BASIC_UNICODE};

#[cfg(feature = "unicode")]
pub use self::latin::{LATIN_FONTS, LATIN_UNICODE};

#[cfg(feature = "unicode")]
pub use self::greek::{GREEK_FONTS, GREEK_UNICODE};

#[cfg(feature = "unicode")]
pub use self::block::{BLOCK_FONTS, BLOCK_UNICODE};

#[cfg(feature = "unicode")]
pub use self::box_chars::{BOX_FONTS, BOX_UNICODE};

#[cfg(feature = "unicode")]
pub use self::hiragana::{HIRAGANA_FONTS, HIRAGANA_UNICODE};

#[cfg(feature = "unicode")]
pub use self::misc::{MISC_FONTS, MISC_UNICODE};

#[cfg(feature = "unicode")]
pub use self::sga::{SGA_FONTS, SGA_UNICODE};

#[cfg(all(feature = "unicode", feature = "std"))]
pub use self::unicode::FromUtf16Error;
#[cfg(feature = "unicode")]
pub use self::unicode::{FontUnicode, UnicodeFonts};

#[cfg(feature = "std")]
mod core {
    pub use std::*;
}
