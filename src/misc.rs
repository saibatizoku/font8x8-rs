//! A miscellanous set of characters.
use super::{legacy::MISC_LEGACY, FontUtf16, Utf16Fonts};
use std::fmt;

/// A constant `[FontUtf16; 10]`, for  Miscellanous fonts (`U+20A7`, `U+0192`, `U+00AA`,
/// `U+00BA`, `U+2310`, `U+2264`, `U+2265`, `U+0060`, `U+1EF2`, and `U+1EF3`).
pub const MISC_UTF16: [FontUtf16; 10] = [
    FontUtf16(0x20A7 as u16, MISC_LEGACY[0]),
    FontUtf16(0x0192 as u16, MISC_LEGACY[1]),
    FontUtf16(0x00AA as u16, MISC_LEGACY[2]),
    FontUtf16(0x00BA as u16, MISC_LEGACY[3]),
    FontUtf16(0x2310 as u16, MISC_LEGACY[4]),
    FontUtf16(0x2264 as u16, MISC_LEGACY[5]),
    FontUtf16(0x2265 as u16, MISC_LEGACY[6]),
    FontUtf16(0x0060 as u16, MISC_LEGACY[7]),
    FontUtf16(0x1EF2 as u16, MISC_LEGACY[8]),
    FontUtf16(0x1EF3 as u16, MISC_LEGACY[9]),
];

/// A convenient constant for Miscellanous fonts (`U+20A7`, `U+0192`, `U+00AA`, `U+00BA`,
/// `U+2310`, `U+2264`, `U+2265`, `U+0060`, `U+1EF2`, and `U+1EF3`), that implements the [`Utf16Fonts`](./utf16/trait.Utf16Fonts.html) trait.
pub const MISC_FONTS: MiscFonts = MiscFonts(MISC_UTF16);

/// Strong-typed collection wrapper for [MISC_UTF16](./constant.MISC_UTF16.html).
pub struct MiscFonts([FontUtf16; 10]);

impl MiscFonts {
    pub fn new() -> Self {
        MiscFonts(MISC_UTF16)
    }
}

impl fmt::Debug for MiscFonts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", stringify!(MISC_UTF16))
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

impl Utf16Fonts for MiscFonts {
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
            if font.is_whitespace() {
                println!("## {:3?}: 0x{:04X} \" \"", idx, font.utf16());
                continue;
            }
            println!(
                "## `{:?}[{:?}]`: `0x{:04X}` `{:?}`",
                self,
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
    }

    fn to_vec(&self) -> Vec<(u16, FontUtf16)> {
        self.0.into_iter().fold(Vec::with_capacity(128), |mut v, font| {
            v.push((font.utf16(), *font));
            v
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn misc_fonts_constant_wraps_basic_utf16_constant() {
        let misc = MiscFonts::new();
        assert!(misc.0.len() == MISC_UTF16.len());
        for (idx, font) in misc.0.iter().enumerate() {
            assert_eq!(font, &MISC_UTF16[idx]);
        }
    }
}
