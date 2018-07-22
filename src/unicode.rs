//! Unicode support for 8x8 fonts.
pub use super::basic::{BasicFonts, BASIC_UNICODE};
pub use super::block::{BlockFonts, BLOCK_UNICODE};
pub use super::box_chars::{BoxFonts, BOX_UNICODE};
pub use super::greek::{GreekFonts, GREEK_UNICODE};
pub use super::hiragana::{HiraganaFonts, HIRAGANA_UNICODE};
pub use super::latin::{LatinFonts, LATIN_UNICODE};
use super::legacy::NOTHING_TO_DISPLAY;
pub use super::misc::{MiscFonts, MISC_UNICODE};
pub use super::sga::{SgaFonts, SGA_UNICODE};
#[cfg(feature = "std")]
pub use std::string::FromUtf16Error;

/// A single 8x8 font which supports `UTF-16` encoding/decoding.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FontUnicode(pub char, pub [u8; 8]);

impl FontUnicode {
    /// Return the char value
    pub fn char(&self) -> char {
        self.0
    }
    /// Return the `[u8; 8]`-representation for this font.
    pub fn byte_array(&self) -> [u8; 8] {
        self.1
    }
    /// Return a result with the corresponding `String` for the font.
    #[cfg(feature = "std")]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    /// Returns a `bool` indicating whether this font renders as a whitespace (all `0`).
    pub fn is_whitespace(&self) -> bool {
        self.1 == NOTHING_TO_DISPLAY
    }

    /// Consumes the current `FontUnicode` and returns the inner `(char, [u8; 8])` tuple.
    pub fn into_inner(self) -> (char, [u8; 8]) {
        self.into()
    }
}

impl Into<char> for FontUnicode {
    fn into(self) -> char {
        self.0
    }
}

impl Into<[u8; 8]> for FontUnicode {
    fn into(self) -> [u8; 8] {
        self.1
    }
}

impl Into<(char, [u8; 8])> for FontUnicode {
    fn into(self) -> (char, [u8; 8]) {
        (self.0, self.1)
    }
}

/// A trait for collections of `FontUnicode`, which provide methods for retrieving
/// the `Option<[u8; 8]>`, using the corresponding `char` as key.
pub trait UnicodeFonts {
    fn get(&self, key: char) -> Option<[u8; 8]>;

    fn get_font(&self, key: char) -> Option<FontUnicode>;

    fn iter(&self) -> ::core::slice::Iter<FontUnicode>;

    #[cfg(feature = "std")]
    fn print_set(&self);

    #[cfg(feature = "std")]
    fn to_vec(&self) -> Vec<(char, FontUnicode)>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_unicode_converts_into_char() {
        let my_font = FontUnicode('á', [110u8; 8]);
        let ch: char = my_font.into();
        assert_eq!(ch, 'á');
    }

    #[test]
    fn font_unicode_converts_into_byte_array() {
        let my_font = FontUnicode('C', NOTHING_TO_DISPLAY);
        let byte_array: [u8; 8] = my_font.into();
        assert_eq!(byte_array, NOTHING_TO_DISPLAY);
    }

    #[test]
    fn font_unicode_converts_into_inner_tuple() {
        let my_font = FontUnicode('Á', [110u8; 8]);
        let font_tuple: (char, [u8; 8]) = my_font.into();
        assert_eq!(font_tuple, ('Á', [110u8; 8]));
    }

    #[test]
    fn font_unicode_api_method_unicode_returns_char() {
        let my_font = FontUnicode('ñ', [0x20; 8]);
        assert_eq!(my_font.char(), 'ñ');
    }

    #[test]
    fn font_unicode_api_method_byte_array_returns_array_with_8_bytes() {
        let my_font = FontUnicode('Ñ', [0x20; 8]);
        assert_eq!(my_font.byte_array(), [0x20; 8]);
    }

    #[cfg(feature = "std")]
    #[test]
    fn font_unicode_api_method_to_string_returns_string_from_unicode() {
        let my_font = FontUnicode('Ñ', [0x20; 8]);
        assert_eq!(my_font.to_string(), "Ñ".to_string());
    }

    #[cfg(feature = "std")]
    #[test]
    fn font_unicode_api_method_is_whitespace_returns_bool() {
        let my_font = FontUnicode('Ñ', [0x20; 8]);
        assert_eq!(my_font.is_whitespace(), false);
        let my_font = FontUnicode('Ñ', NOTHING_TO_DISPLAY);
        assert!(my_font.is_whitespace());
    }

    #[test]
    fn font_unicode_api_method_into_inner_returns_inner_tuple() {
        let my_font = FontUnicode('Á', [110u8; 8]);
        assert_eq!(my_font.into_inner(), ('Á', [110u8; 8]));
    }
}
