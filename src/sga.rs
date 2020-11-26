//! Special characters with private unicode points.
use super::{
    legacy::SGA_LEGACY,
    unicode::{FontUnicode, UnicodeFonts},
};
use core::fmt;

/// A constant `[FontUnicode; 26]`, for special SGA fonts (`U+E543` - `U+E55A`).
pub const SGA_UNICODE: [FontUnicode; 26] = [
    FontUnicode('\u{E541}', SGA_LEGACY[0]),
    FontUnicode('\u{E542}', SGA_LEGACY[1]),
    FontUnicode('\u{E543}', SGA_LEGACY[2]),
    FontUnicode('\u{E544}', SGA_LEGACY[3]),
    FontUnicode('\u{E545}', SGA_LEGACY[4]),
    FontUnicode('\u{E546}', SGA_LEGACY[5]),
    FontUnicode('\u{E547}', SGA_LEGACY[6]),
    FontUnicode('\u{E548}', SGA_LEGACY[7]),
    FontUnicode('\u{E549}', SGA_LEGACY[8]),
    FontUnicode('\u{E54A}', SGA_LEGACY[9]),
    FontUnicode('\u{E54B}', SGA_LEGACY[10]),
    FontUnicode('\u{E54C}', SGA_LEGACY[11]),
    FontUnicode('\u{E54D}', SGA_LEGACY[12]),
    FontUnicode('\u{E54E}', SGA_LEGACY[13]),
    FontUnicode('\u{E54F}', SGA_LEGACY[14]),
    FontUnicode('\u{E550}', SGA_LEGACY[15]),
    FontUnicode('\u{E551}', SGA_LEGACY[16]),
    FontUnicode('\u{E552}', SGA_LEGACY[17]),
    FontUnicode('\u{E553}', SGA_LEGACY[18]),
    FontUnicode('\u{E554}', SGA_LEGACY[19]),
    FontUnicode('\u{E555}', SGA_LEGACY[20]),
    FontUnicode('\u{E556}', SGA_LEGACY[21]),
    FontUnicode('\u{E557}', SGA_LEGACY[22]),
    FontUnicode('\u{E558}', SGA_LEGACY[23]),
    FontUnicode('\u{E559}', SGA_LEGACY[24]),
    FontUnicode('\u{E55A}', SGA_LEGACY[25]),
];

/// A convenient constant for special SGA fonts (`U+E541` - `U+E55A`), that implements the `UnicodeFonts` trait.
///
/// ## `SGA_UNICODE[0]: U+E541`
/// `"\u{e541}"`
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
/// ## `SGA_UNICODE[1]: U+E542`
/// `"\u{e542}"`
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
/// ## `SGA_UNICODE[2]: U+E543`
/// `"\u{e543}"`
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
/// ## `SGA_UNICODE[3]: U+E544`
/// `"\u{e544}"`
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
/// ## `SGA_UNICODE[4]: U+E545`
/// `"\u{e545}"`
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
/// ## `SGA_UNICODE[5]: U+E546`
/// `"\u{e546}"`
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
/// ## `SGA_UNICODE[6]: U+E547`
/// `"\u{e547}"`
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
/// ## `SGA_UNICODE[7]: U+E548`
/// `"\u{e548}"`
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
/// ## `SGA_UNICODE[8]: U+E549`
/// `"\u{e549}"`
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
/// ## `SGA_UNICODE[9]: U+E54A`
/// `"\u{e54a}"`
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
/// ## `SGA_UNICODE[10]: U+E54B`
/// `"\u{e54b}"`
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
/// ## `SGA_UNICODE[11]: U+E54C`
/// `"\u{e54c}"`
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
/// ## `SGA_UNICODE[12]: U+E54D`
/// `"\u{e54d}"`
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
/// ## `SGA_UNICODE[13]: U+E54E`
/// `"\u{e54e}"`
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
/// ## `SGA_UNICODE[14]: U+E54F`
/// `"\u{e54f}"`
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
/// ## `SGA_UNICODE[15]: U+E550`
/// `"\u{e550}"`
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
/// ## `SGA_UNICODE[16]: U+E551`
/// `"\u{e551}"`
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
/// ## `SGA_UNICODE[17]: U+E552`
/// `"\u{e552}"`
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
/// ## `SGA_UNICODE[18]: U+E553`
/// `"\u{e553}"`
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
/// ## `SGA_UNICODE[19]: U+E554`
/// `"\u{e554}"`
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
/// ## `SGA_UNICODE[20]: U+E555`
/// `"\u{e555}"`
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
/// ## `SGA_UNICODE[21]: U+E556`
/// `"\u{e556}"`
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
/// ## `SGA_UNICODE[22]: U+E557`
/// `"\u{e557}"`
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
/// ## `SGA_UNICODE[23]: U+E558`
/// `"\u{e558}"`
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
/// ## `SGA_UNICODE[24]: U+E559`
/// `"\u{e559}"`
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
/// ## `SGA_UNICODE[25]: U+E55A`
/// `"\u{e55a}"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░████░░
/// ░██░░██░
/// ░██░░██░
/// ░██░░██░
/// ░░░░░░░░
/// ```
pub const SGA_FONTS: SgaFonts = SgaFonts(SGA_UNICODE);

/// Strong-typed collection wrapper for [SGA_UNICODE](./constant.SGA_UNICODE.html).
pub struct SgaFonts([FontUnicode; 26]);

impl SgaFonts {
    pub fn new() -> Self {
        SgaFonts(SGA_UNICODE)
    }
}

impl fmt::Debug for SgaFonts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, stringify!(SGA_UNICODE))
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

impl UnicodeFonts for SgaFonts {
    fn get(&self, key: char) -> Option<[u8; 8]> {
        match self.get_font(key) {
            Some(font) => Some(font.into()),
            None => None,
        }
    }

    fn get_font(&self, key: char) -> Option<FontUnicode> {
        match self.0.binary_search_by_key(&key, |&f| f.char()) {
            Ok(idx) => Some(self.0[idx]),
            _ => None,
        }
    }

    fn iter(&self) -> ::core::slice::Iter<FontUnicode> {
        self.0.iter()
    }

    #[cfg(feature = "std")]
    fn print_set(&self) {
        println!();
        println!("# `{:?}`", self);
        for (idx, font) in self.0.iter().enumerate() {
            print!("## `{:?}[{:?}]: U+{:04X}`", self, idx, font.char() as u32,);
            if font.is_whitespace() {
                println!(" `WHITESPACE`");
            } else {
                println!();
                println!("`{:?}`", font.to_string());
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
        }
    }

    #[cfg(feature = "std")]
    fn to_vec(&self) -> Vec<(char, FontUnicode)> {
        self.0.iter().fold(Vec::with_capacity(128), |mut v, font| {
            v.push((font.char(), *font));
            v
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sga_fonts_inner_array_is_sorted_by_unicode_key() {
        let mut sorted = SGA_UNICODE;
        sorted.sort_by_key(|f| f.char());
        for (idx, key) in sorted.iter().enumerate() {
            assert_eq!(SGA_UNICODE[idx].char(), key.char());
        }
    }

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
    fn sga_fonts_constant_wraps_basic_unicode_constant() {
        let sga = SgaFonts::new();
        assert!(sga.0.len() == SGA_UNICODE.len());
        for (idx, font) in sga.0.iter().enumerate() {
            assert_eq!(font, &SGA_UNICODE[idx]);
        }
    }
}
