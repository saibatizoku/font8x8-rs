//! Greek Characters. `U+0390 - U+03C9`
use super::{legacy::GREEK_LEGACY, FontUtf16, Utf16Fonts};
use std::fmt;

/// Constant array of `FontUtf16`, which map the items of `GREEK_LEGACY` with their associated
/// utf16 value, encoded as `u16`. Please read [FontUtf16](./struct.FontUtf16.html) documentation
/// for information about the methods and traits it implements.
pub const GREEK_UTF16: [FontUtf16; 58] = [
    FontUtf16(0x0390 as u16, GREEK_LEGACY[0]),
    FontUtf16(0x0391 as u16, GREEK_LEGACY[1]),
    FontUtf16(0x0392 as u16, GREEK_LEGACY[2]),
    FontUtf16(0x0393 as u16, GREEK_LEGACY[3]),
    FontUtf16(0x0394 as u16, GREEK_LEGACY[4]),
    FontUtf16(0x0395 as u16, GREEK_LEGACY[5]),
    FontUtf16(0x0396 as u16, GREEK_LEGACY[6]),
    FontUtf16(0x0397 as u16, GREEK_LEGACY[7]),
    FontUtf16(0x0398 as u16, GREEK_LEGACY[8]),
    FontUtf16(0x0399 as u16, GREEK_LEGACY[9]),
    FontUtf16(0x039A as u16, GREEK_LEGACY[10]),
    FontUtf16(0x039B as u16, GREEK_LEGACY[11]),
    FontUtf16(0x039C as u16, GREEK_LEGACY[12]),
    FontUtf16(0x039D as u16, GREEK_LEGACY[13]),
    FontUtf16(0x039E as u16, GREEK_LEGACY[14]),
    FontUtf16(0x039F as u16, GREEK_LEGACY[15]),
    FontUtf16(0x03A0 as u16, GREEK_LEGACY[16]),
    FontUtf16(0x03A1 as u16, GREEK_LEGACY[17]),
    FontUtf16(0x03A2 as u16, GREEK_LEGACY[18]),
    FontUtf16(0x03A3 as u16, GREEK_LEGACY[19]),
    FontUtf16(0x03A4 as u16, GREEK_LEGACY[20]),
    FontUtf16(0x03A5 as u16, GREEK_LEGACY[21]),
    FontUtf16(0x03A6 as u16, GREEK_LEGACY[22]),
    FontUtf16(0x03A7 as u16, GREEK_LEGACY[23]),
    FontUtf16(0x03A8 as u16, GREEK_LEGACY[24]),
    FontUtf16(0x03A9 as u16, GREEK_LEGACY[25]),
    FontUtf16(0x0399 as u16, GREEK_LEGACY[26]),
    FontUtf16(0x03A5 as u16, GREEK_LEGACY[27]),
    FontUtf16(0x03AC as u16, GREEK_LEGACY[28]),
    FontUtf16(0x03AD as u16, GREEK_LEGACY[29]),
    FontUtf16(0x03AE as u16, GREEK_LEGACY[30]),
    FontUtf16(0x03AF as u16, GREEK_LEGACY[31]),
    FontUtf16(0x03B0 as u16, GREEK_LEGACY[32]),
    FontUtf16(0x03B1 as u16, GREEK_LEGACY[33]),
    FontUtf16(0x03B2 as u16, GREEK_LEGACY[34]),
    FontUtf16(0x03B3 as u16, GREEK_LEGACY[35]),
    FontUtf16(0x03B4 as u16, GREEK_LEGACY[36]),
    FontUtf16(0x03B5 as u16, GREEK_LEGACY[37]),
    FontUtf16(0x03B6 as u16, GREEK_LEGACY[38]),
    FontUtf16(0x03B7 as u16, GREEK_LEGACY[39]),
    FontUtf16(0x03B8 as u16, GREEK_LEGACY[40]),
    FontUtf16(0x03B9 as u16, GREEK_LEGACY[41]),
    FontUtf16(0x03BA as u16, GREEK_LEGACY[42]),
    FontUtf16(0x03BB as u16, GREEK_LEGACY[43]),
    FontUtf16(0x03BC as u16, GREEK_LEGACY[44]),
    FontUtf16(0x03BD as u16, GREEK_LEGACY[45]),
    FontUtf16(0x03BE as u16, GREEK_LEGACY[46]),
    FontUtf16(0x03BF as u16, GREEK_LEGACY[47]),
    FontUtf16(0x03C0 as u16, GREEK_LEGACY[48]),
    FontUtf16(0x03C1 as u16, GREEK_LEGACY[49]),
    FontUtf16(0x03C2 as u16, GREEK_LEGACY[50]),
    FontUtf16(0x03C3 as u16, GREEK_LEGACY[51]),
    FontUtf16(0x03C4 as u16, GREEK_LEGACY[52]),
    FontUtf16(0x03C5 as u16, GREEK_LEGACY[53]),
    FontUtf16(0x03C6 as u16, GREEK_LEGACY[54]),
    FontUtf16(0x03C7 as u16, GREEK_LEGACY[55]),
    FontUtf16(0x03C8 as u16, GREEK_LEGACY[56]),
    FontUtf16(0x03C9 as u16, GREEK_LEGACY[57]),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greek_set_implements_default_trait_with_method_new() {
        let greek_set: GreekFonts = Default::default();
        assert_eq!(greek_set, GreekFonts::new());
    }

    #[test]
    fn greek_fonts_constant_is_equal_to_a_new_instance() {
        assert_eq!(GREEK_FONTS, GreekFonts::new());
    }

    #[test]
    fn greek_fonts_constant_wraps_basic_utf16_constant() {
        let greek = GreekFonts::new();
        assert!(greek.0.len() == GREEK_UTF16.len());
        for (idx, font) in greek.0.iter().enumerate() {
            assert_eq!(font, &GREEK_UTF16[idx]);
        }
    }
}
