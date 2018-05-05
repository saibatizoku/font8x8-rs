//! Special characters with private unicode points.
use super::{legacy::SGA_LEGACY, utf16::{FontUtf16, Utf16Fonts}};
use std::fmt;

/// A constant `[FontUtf16; 26]`, for special SGA fonts (`U+E543` - `U+E55A`).
///
/// ## `SGA_UTF16[0]`: `0xE541` `"\u{e541}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░███░░
/// ░██░░██░
/// ░██░░░░░
/// ░██░░░░░
/// ███░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[1]`: `0xE542` `"\u{e542}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░░██░░░
/// ░░░░██░░
/// ███████░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[2]`: `0xE543` `"\u{e543}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ░░██░░░░
/// ░░░░██░░
/// ░░░░██░░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[3]`: `0xE544` `"\u{e544}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ███████░
/// ░░░░░░░░
/// ██░░░░░░
/// ░░███░░░
/// ░░░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[4]`: `0xE545` `"\u{e545}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░░░██░
/// ██░░░░░░
/// ██░░░░░░
/// ██░░░░░░
/// ███████░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[5]`: `0xE546` `"\u{e546}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████████
/// ░░░░░░░░
/// ██░██░██
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[6]`: `0xE547` `"\u{e547}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░██░░
/// ░░░░██░░
/// ░█████░░
/// ░░░░██░░
/// ░░░░██░░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[7]`: `0xE548` `"\u{e548}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░██████░
/// ░░░░░░░░
/// ░██████░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[8]`: `0xE549` `"\u{e549}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[9]`: `0xE54A` `"\u{e54a}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[10]`: `0xE54B` `"\u{e54b}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░█░██░█░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[11]`: `0xE54C` `"\u{e54c}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░░░░░░
/// ██░░██░░
/// ██░░░░░░
/// ██░░██░░
/// ██░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[12]`: `0xE54D` `"\u{e54d}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░░░██░
/// ░░░░░██░
/// ░░░░░██░
/// ░░░░░██░
/// ███████░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[13]`: `0xE54E` `"\u{e54e}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░██░░██░
/// ░░░░░██░
/// ░░░░██░░
/// ░░░██░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[14]`: `0xE54F` `"\u{e54f}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░████░░
/// ░░░░░██░
/// ░░░░██░░
/// ░░░██░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[15]`: `0xE550` `"\u{e550}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░██░░██░
/// ░░░░░██░
/// ░██░░██░
/// ░██░░░░░
/// ░██░░██░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[16]`: `0xE551` `"\u{e551}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ░██████░
/// ░░░░░██░
/// ░██████░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[17]`: `0xE552` `"\u{e552}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░██░░██░
/// ░░░░░░░░
/// ░██░░██░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[18]`: `0xE553` `"\u{e553}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░████░░
/// ░░░░██░░
/// ░░░░██░░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[19]`: `0xE554` `"\u{e554}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░████░░
/// ░░░░██░░
/// ░░░░██░░
/// ░░░░░░░░
/// ░░░░██░░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[20]`: `0xE555` `"\u{e555}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░██░██░░
/// ░░░░░░░░
/// ███████░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[21]`: `0xE556` `"\u{e556}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░██████░
/// ░░░░░░░░
/// ░██████░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[22]`: `0xE557` `"\u{e557}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ░██░░██░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[23]`: `0xE558` `"\u{e558}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░██░░██░
/// ░░░░██░░
/// ░░░██░░░
/// ░░██░░░░
/// ░██░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[24]`: `0xE559` `"\u{e559}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░██░██░░
/// ░██░██░░
/// ░██░██░░
/// ░██░██░░
/// ░██░██░░
/// ░░░░░░░░
/// ```
///
/// ## `SGA_UTF16[25]`: `0xE55A` `"\u{e55a}"`
///
/// ```markdown
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░████░░
/// ░██░░██░
/// ░██░░██░
/// ░██░░██░
/// ░░░░░░░░
/// ```
pub const SGA_UTF16: [FontUtf16; 26] = [
    FontUtf16(0xE541 as u16, SGA_LEGACY[0]),
    FontUtf16(0xE542 as u16, SGA_LEGACY[1]),
    FontUtf16(0xE543 as u16, SGA_LEGACY[2]),
    FontUtf16(0xE544 as u16, SGA_LEGACY[3]),
    FontUtf16(0xE545 as u16, SGA_LEGACY[4]),
    FontUtf16(0xE546 as u16, SGA_LEGACY[5]),
    FontUtf16(0xE547 as u16, SGA_LEGACY[6]),
    FontUtf16(0xE548 as u16, SGA_LEGACY[7]),
    FontUtf16(0xE549 as u16, SGA_LEGACY[8]),
    FontUtf16(0xE54A as u16, SGA_LEGACY[9]),
    FontUtf16(0xE54B as u16, SGA_LEGACY[10]),
    FontUtf16(0xE54C as u16, SGA_LEGACY[11]),
    FontUtf16(0xE54D as u16, SGA_LEGACY[12]),
    FontUtf16(0xE54E as u16, SGA_LEGACY[13]),
    FontUtf16(0xE54F as u16, SGA_LEGACY[14]),
    FontUtf16(0xE550 as u16, SGA_LEGACY[15]),
    FontUtf16(0xE551 as u16, SGA_LEGACY[16]),
    FontUtf16(0xE552 as u16, SGA_LEGACY[17]),
    FontUtf16(0xE553 as u16, SGA_LEGACY[18]),
    FontUtf16(0xE554 as u16, SGA_LEGACY[19]),
    FontUtf16(0xE555 as u16, SGA_LEGACY[20]),
    FontUtf16(0xE556 as u16, SGA_LEGACY[21]),
    FontUtf16(0xE557 as u16, SGA_LEGACY[22]),
    FontUtf16(0xE558 as u16, SGA_LEGACY[23]),
    FontUtf16(0xE559 as u16, SGA_LEGACY[24]),
    FontUtf16(0xE55A as u16, SGA_LEGACY[25]),
];

/// A convenient constant for special SGA fonts (`U+E541` - `U+E55A`), that implements the [`Utf16Fonts`](./utf16/trait.Utf16Fonts.html) trait.
pub const SGA_FONTS: SgaFonts = SgaFonts(SGA_UTF16);

/// Strong-typed collection wrapper for [SGA_UTF16](./constant.SGA_UTF16.html).
pub struct SgaFonts([FontUtf16; 26]);

impl SgaFonts {
    pub fn new() -> Self {
        SgaFonts(SGA_UTF16)
    }
}

impl fmt::Debug for SgaFonts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", stringify!(SGA_UTF16))
    }
}

impl PartialEq for SgaFonts {
    fn eq(&self, other: &SgaFonts) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(true, |eq, (a, b)| eq && a == b)
    }
}

impl Default for SgaFonts {
    fn default() -> Self {
        SgaFonts::new()
    }
}

impl Utf16Fonts for SgaFonts {
    fn get(&self, key: u16) -> Option<[u8; 8]> {
        match self.get_font(key) {
            Some(font) => Some(font.into()),
            None => None,
        }
    }

    fn get_font(&self, key: u16) -> Option<FontUtf16> {
        match self.0.binary_search_by_key(&key, |&f| f.utf16()) {
            Ok(idx) => Some(self.0[idx]),
            _ => None,
        }
    }

    fn print_set(&self) {
        println!();
        println!("# `{:?}`", self);
        for (idx, font) in self.0.iter().enumerate() {
            print_set(idx, &font);
        }
    }
}
fn print_set(idx: usize, font: &FontUtf16) {
    if font.is_whitespace() {
        println!("## {:3?}: 0x{:04X} \" \"", idx, font.utf16());
        return;
    }
    println!(
        "## `[{:?}]`: `0x{:04X}` `{:?}`",
        idx,
        font.utf16(),
        font.to_string()
    );
    println!();
    println!("```text");
    for x in &font.byte_array() {
        for bit in 0..8 {
            match *x & 1 << bit {
                0 => print!("░"),
                _ => print!("█"),
            }
        }
        println!();
    }
    println!("```");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sga_set_implements_default_trait_with_method_new() {
        let sga_set: SgaFonts = Default::default();
        assert_eq!(sga_set, SgaFonts::new());
    }

    #[test]
    fn sga_fonts_constant_is_equal_to_a_new_instance() {
        assert_eq!(SGA_FONTS, SgaFonts::new());
    }

    #[test]
    fn sga_fonts_constant_wraps_basic_utf16_constant() {
        let sga = SgaFonts::new();
        assert!(sga.0.len() == SGA_UTF16.len());
        for (idx, font) in sga.0.iter().enumerate() {
            assert_eq!(font, &SGA_UTF16[idx]);
        }
    }
}
