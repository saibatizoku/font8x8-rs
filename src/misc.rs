//! A miscellanous set of characters.
use super::{legacy::MISC_LEGACY, FontUnicode, UnicodeFonts};
use core::fmt;

/// A constant `[FontUnicode; 10]`, for  Miscellanous fonts (`U+20A7`, `U+0192`, `U+00AA`,
/// `U+00BA`, `U+2310`, `U+2264`, `U+2265`, `U+0060`, `U+1EF2`, and `U+1EF3`).
pub const MISC_UNICODE: [FontUnicode; 10] = [
    FontUnicode('\u{0060}', MISC_LEGACY[7]),
    FontUnicode('\u{00AA}', MISC_LEGACY[2]),
    FontUnicode('\u{00BA}', MISC_LEGACY[3]),
    FontUnicode('\u{0192}', MISC_LEGACY[1]),
    FontUnicode('\u{1EF2}', MISC_LEGACY[8]),
    FontUnicode('\u{1EF3}', MISC_LEGACY[9]),
    FontUnicode('\u{20A7}', MISC_LEGACY[0]),
    FontUnicode('\u{2264}', MISC_LEGACY[5]),
    FontUnicode('\u{2265}', MISC_LEGACY[6]),
    FontUnicode('\u{2310}', MISC_LEGACY[4]),
];

/// A convenient constant for Miscellanous fonts (`U+20A7`, `U+0192`, `U+00AA`, `U+00BA`,
/// `U+2310`, `U+2264`, `U+2265`, `U+0060`, `U+1EF2`, and `U+1EF3`), that implements the `UnicodeFonts` trait.
///
/// ## `MISC_UNICODE[0]: U+20A7`
/// `"₧"`
///
/// ```text
/// █████░░░
/// ██░░██░░
/// ██░░██░░
/// █████░█░
/// ██░░░██░
/// ██░░████
/// ██░░░██░
/// ██░░░███
/// ```
///
/// ## `MISC_UNICODE[1]: U+0192`
/// `"ƒ"`
///
/// ```text
/// ░░░░███░
/// ░░░██░██
/// ░░░██░░░
/// ░░████░░
/// ░░░██░░░
/// ░░░██░░░
/// ██░██░░░
/// ░███░░░░
/// ```
///
/// ## `MISC_UNICODE[2]: U+00AA`
/// `"ª"`
///
/// ```text
/// ░░████░░
/// ░██░██░░
/// ░██░██░░
/// ░░█████░
/// ░░░░░░░░
/// ░██████░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `MISC_UNICODE[3]: U+00BA`
/// `"º"`
///
/// ```text
/// ░░███░░░
/// ░██░██░░
/// ░██░██░░
/// ░░███░░░
/// ░░░░░░░░
/// ░█████░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `MISC_UNICODE[4]: U+2310`
/// `"⌐"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ██████░░
/// ██░░░░░░
/// ██░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `MISC_UNICODE[5]: U+2264`
/// `"≤"`
///
/// ```text
/// ░░░░██░░
/// ░░░██░░░
/// ░░██░░░░
/// ░░░██░░░
/// ░░░░██░░
/// ░░░░░░░░
/// ░██████░
/// ░░░░░░░░
/// ```
///
/// ## `MISC_UNICODE[6]: U+2265`
/// `"≥"`
///
/// ```text
/// ░░██░░░░
/// ░░░██░░░
/// ░░░░██░░
/// ░░░██░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ░██████░
/// ░░░░░░░░
/// ```
///
/// ## `MISC_UNICODE[7]: U+0060`
/// `"`"`
///
/// ```text
/// ░░██░░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `MISC_UNICODE[8]: U+1EF2`
/// `"Ỳ"`
///
/// ```text
/// ░███░░░░
/// ░░░░░░░░
/// ░██░░██░
/// ░██░░██░
/// ░░████░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `MISC_UNICODE[9]: U+1EF3`
/// `"ỳ"`
///
/// ```text
/// ░░░░░░░░
/// ███░░░░░
/// ░░░░░░░░
/// ██░░██░░
/// ██░░██░░
/// ░█████░░
/// ░░░░██░░
/// █████░░░
/// ```
pub const MISC_FONTS: MiscFonts = MiscFonts(MISC_UNICODE);

/// Strong-typed collection wrapper for [MISC_UNICODE](./constant.MISC_UNICODE.html).
pub struct MiscFonts([FontUnicode; 10]);

impl MiscFonts {
    pub fn new() -> Self {
        MiscFonts(MISC_UNICODE)
    }
}

impl fmt::Debug for MiscFonts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, stringify!(MISC_UNICODE))
    }
}

impl PartialEq for MiscFonts {
    fn eq(&self, other: &MiscFonts) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(true, |eq, (a, b)| eq && a == b)
    }
}

impl Default for MiscFonts {
    fn default() -> Self {
        MiscFonts::new()
    }
}

impl UnicodeFonts for MiscFonts {
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
    fn misc_fonts_inner_array_is_sorted_by_unicode_key() {
        let mut sorted = MISC_UNICODE;
        sorted.sort_by_key(|f| f.char());
        for (idx, key) in sorted.iter().enumerate() {
            assert_eq!(MISC_UNICODE[idx].char(), key.char());
        }
    }

    #[test]
    fn misc_set_implements_default_trait_with_method_new() {
        let misc_set: MiscFonts = Default::default();
        assert_eq!(misc_set, MiscFonts::new());
    }

    #[test]
    fn misc_fonts_constant_is_equal_to_a_new_instance() {
        assert_eq!(MISC_FONTS, MiscFonts::new());
    }

    #[test]
    fn misc_fonts_constant_wraps_basic_unicode_constant() {
        let misc = MiscFonts::new();
        assert!(misc.0.len() == MISC_UNICODE.len());
        for (idx, font) in misc.0.iter().enumerate() {
            assert_eq!(font, &MISC_UNICODE[idx]);
        }
    }
}
