//! Fonts with `UTF16` support.
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

    /// Consumes the current `FontUtf16` and returns the inner `(u16, [u8; 8])` tuple.
    pub fn into_inner(self) -> (u16, [u8; 8]) {
        self.into()
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

impl Into<(u16, [u8; 8])> for FontUtf16 {
    fn into(self) -> (u16, [u8; 8]) {
        (self.0, self.1)
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
        let font_tuple: (u16, [u8; 8]) = my_font.into();
        assert_eq!(font_tuple, ('Á' as u16, [110u8; 8]));
    }

    #[test]
    fn font_utf16_api_method_utf16_returns_u16() {
        let my_font = FontUtf16('ñ' as u16, [0x20; 8]);
        assert_eq!(my_font.utf16(), 'ñ' as u16);
    }

    #[test]
    fn font_utf16_api_method_byte_array_returns_array_with_8_bytes() {
        let my_font = FontUtf16('Ñ' as u16, [0x20; 8]);
        assert_eq!(my_font.byte_array(), [0x20; 8]);
    }

    #[test]
    fn font_utf16_api_method_to_string_returns_string_from_utf16() {
        let my_font = FontUtf16('Ñ' as u16, [0x20; 8]);
        assert_eq!(my_font.to_string(), "Ñ".to_string());
    }

    #[test]
    fn font_utf16_api_method_is_whitespace_returns_bool() {
        let my_font = FontUtf16('Ñ' as u16, [0x20; 8]);
        assert_eq!(my_font.is_whitespace(), false);
        let my_font = FontUtf16('Ñ' as u16, NOTHING_TO_DISPLAY);
        assert!(my_font.is_whitespace());
    }

    #[test]
    fn font_utf16_api_method_into_inner_returns_inner_tuple() {
        let my_font = FontUtf16('Á' as u16, [110u8; 8]);
        assert_eq!(my_font.into_inner(), ('Á' as u16, [110u8; 8]));
    }
}
