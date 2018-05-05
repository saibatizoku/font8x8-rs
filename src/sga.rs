//! Special characters with private unicode points.
use super::{legacy::SGA_LEGACY, utf16::{FontUtf16, Utf16Fonts}};
use std::fmt;

/// `SGA_UTF16` description and ASCII-art representation.
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

#[cfg(feature = "unicode")]
pub const SGA_UNICODE: [(u16, [u8; 8]); 26] = [
    (0xE541, SGA[0]),
    (0xE542, SGA[1]),
    (0xE543, SGA[2]),
    (0xE544, SGA[3]),
    (0xE545, SGA[4]),
    (0xE546, SGA[5]),
    (0xE547, SGA[6]),
    (0xE548, SGA[7]),
    (0xE549, SGA[8]),
    (0xE54A, SGA[9]),
    (0xE54B, SGA[10]),
    (0xE54C, SGA[11]),
    (0xE54D, SGA[12]),
    (0xE54E, SGA[13]),
    (0xE54F, SGA[14]),
    (0xE550, SGA[15]),
    (0xE551, SGA[16]),
    (0xE552, SGA[17]),
    (0xE553, SGA[18]),
    (0xE554, SGA[19]),
    (0xE555, SGA[20]),
    (0xE556, SGA[21]),
    (0xE557, SGA[22]),
    (0xE558, SGA[23]),
    (0xE559, SGA[24]),
    (0xE55A, SGA[25]),
];

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
