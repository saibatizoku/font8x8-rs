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
//! use font8x8::GREEK;
//!
//! fn main() {
//!     for x in &GREEK[0] {
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
//! or, this: `ΐ`.
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
mod latin;
mod greek;
mod hiragana;
mod misc;
mod sga;

pub use self::basic::BASIC;
pub use self::control::CONTROL;
pub use self::latin::LATIN;
pub use self::greek::GREEK;
pub use self::block::BLOCK;
pub use self::box_chars::BOX;
pub use self::hiragana::HIRAGANA;
pub use self::misc::MISC;
pub use self::sga::SGA;
