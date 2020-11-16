//!  Block Elements. `U+2580` - `U+259F`
use super::{legacy::BLOCK_LEGACY, FontUnicode, UnicodeFonts};
use core::fmt;

/// A constant `[FontUnicode; 32]`, for Block Element fonts (`U+2580` - `U+259F`).
pub const BLOCK_UNICODE: [FontUnicode; 32] = [
    FontUnicode('\u{2580}', BLOCK_LEGACY[0]),
    FontUnicode('\u{2581}', BLOCK_LEGACY[1]),
    FontUnicode('\u{2582}', BLOCK_LEGACY[2]),
    FontUnicode('\u{2583}', BLOCK_LEGACY[3]),
    FontUnicode('\u{2584}', BLOCK_LEGACY[4]),
    FontUnicode('\u{2585}', BLOCK_LEGACY[5]),
    FontUnicode('\u{2586}', BLOCK_LEGACY[6]),
    FontUnicode('\u{2587}', BLOCK_LEGACY[7]),
    FontUnicode('\u{2588}', BLOCK_LEGACY[8]),
    FontUnicode('\u{2589}', BLOCK_LEGACY[9]),
    FontUnicode('\u{258A}', BLOCK_LEGACY[10]),
    FontUnicode('\u{258B}', BLOCK_LEGACY[11]),
    FontUnicode('\u{258C}', BLOCK_LEGACY[12]),
    FontUnicode('\u{258D}', BLOCK_LEGACY[13]),
    FontUnicode('\u{258E}', BLOCK_LEGACY[14]),
    FontUnicode('\u{258F}', BLOCK_LEGACY[15]),
    FontUnicode('\u{2590}', BLOCK_LEGACY[16]),
    FontUnicode('\u{2591}', BLOCK_LEGACY[17]),
    FontUnicode('\u{2592}', BLOCK_LEGACY[18]),
    FontUnicode('\u{2593}', BLOCK_LEGACY[19]),
    FontUnicode('\u{2594}', BLOCK_LEGACY[20]),
    FontUnicode('\u{2595}', BLOCK_LEGACY[21]),
    FontUnicode('\u{2596}', BLOCK_LEGACY[22]),
    FontUnicode('\u{2597}', BLOCK_LEGACY[23]),
    FontUnicode('\u{2598}', BLOCK_LEGACY[24]),
    FontUnicode('\u{2599}', BLOCK_LEGACY[25]),
    FontUnicode('\u{259A}', BLOCK_LEGACY[26]),
    FontUnicode('\u{259B}', BLOCK_LEGACY[27]),
    FontUnicode('\u{259C}', BLOCK_LEGACY[28]),
    FontUnicode('\u{259D}', BLOCK_LEGACY[29]),
    FontUnicode('\u{259E}', BLOCK_LEGACY[30]),
    FontUnicode('\u{259F}', BLOCK_LEGACY[31]),
];

/// A convenient constant for Block Element fonts (`U+2580` - `U+259F`), that implements the `UnicodeFonts` trait.
///
/// ## `BLOCK_UNICODE[0]: U+2580`
/// `"▀"`
///
/// ```text
/// ████████
/// ████████
/// ████████
/// ████████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BLOCK_UNICODE[1]: U+2581`
/// `"▁"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████████
/// ```
///
/// ## `BLOCK_UNICODE[2]: U+2582`
/// `"▂"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████████
/// ████████
/// ```
///
/// ## `BLOCK_UNICODE[3]: U+2583`
/// `"▃"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████████
/// ████████
/// ████████
/// ```
///
/// ## `BLOCK_UNICODE[4]: U+2584`
/// `"▄"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████████
/// ████████
/// ████████
/// ████████
/// ```
///
/// ## `BLOCK_UNICODE[5]: U+2585`
/// `"▅"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████████
/// ████████
/// ████████
/// ████████
/// ████████
/// ```
///
/// ## `BLOCK_UNICODE[6]: U+2586`
/// `"▆"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ████████
/// ████████
/// ████████
/// ████████
/// ████████
/// ████████
/// ```
///
/// ## `BLOCK_UNICODE[7]: U+2587`
/// `"▇"`
///
/// ```text
/// ░░░░░░░░
/// ████████
/// ████████
/// ████████
/// ████████
/// ████████
/// ████████
/// ████████
/// ```
///
/// ## `BLOCK_UNICODE[8]: U+2588`
/// `"█"`
///
/// ```text
/// ████████
/// ████████
/// ████████
/// ████████
/// ████████
/// ████████
/// ████████
/// ████████
/// ```
///
/// ## `BLOCK_UNICODE[9]: U+2589`
/// `"▉"`
///
/// ```text
/// ███████░
/// ███████░
/// ███████░
/// ███████░
/// ███████░
/// ███████░
/// ███████░
/// ███████░
/// ```
///
/// ## `BLOCK_UNICODE[10]: U+258A`
/// `"▊"`
///
/// ```text
/// ██████░░
/// ██████░░
/// ██████░░
/// ██████░░
/// ██████░░
/// ██████░░
/// ██████░░
/// ██████░░
/// ```
///
/// ## `BLOCK_UNICODE[11]: U+258B`
/// `"▋"`
///
/// ```text
/// █████░░░
/// █████░░░
/// █████░░░
/// █████░░░
/// █████░░░
/// █████░░░
/// █████░░░
/// █████░░░
/// ```
///
/// ## `BLOCK_UNICODE[12]: U+258C`
/// `"▌"`
///
/// ```text
/// ████░░░░
/// ████░░░░
/// ████░░░░
/// ████░░░░
/// ████░░░░
/// ████░░░░
/// ████░░░░
/// ████░░░░
/// ```
///
/// ## `BLOCK_UNICODE[13]: U+258D`
/// `"▍"`
///
/// ```text
/// ███░░░░░
/// ███░░░░░
/// ███░░░░░
/// ███░░░░░
/// ███░░░░░
/// ███░░░░░
/// ███░░░░░
/// ███░░░░░
/// ```
///
/// ## `BLOCK_UNICODE[14]: U+258E`
/// `"▎"`
///
/// ```text
/// ██░░░░░░
/// ██░░░░░░
/// ██░░░░░░
/// ██░░░░░░
/// ██░░░░░░
/// ██░░░░░░
/// ██░░░░░░
/// ██░░░░░░
/// ```
///
/// ## `BLOCK_UNICODE[15]: U+258F`
/// `"▏"`
///
/// ```text
/// █░░░░░░░
/// █░░░░░░░
/// █░░░░░░░
/// █░░░░░░░
/// █░░░░░░░
/// █░░░░░░░
/// █░░░░░░░
/// █░░░░░░░
/// ```
///
/// ## `BLOCK_UNICODE[16]: U+2590`
/// `"▐"`
///
/// ```text
/// ░░░░████
/// ░░░░████
/// ░░░░████
/// ░░░░████
/// ░░░░████
/// ░░░░████
/// ░░░░████
/// ░░░░████
/// ```
///
/// ## `BLOCK_UNICODE[17]: U+2591`
/// `"░"`
///
/// ```text
/// █░█░█░█░
/// ░░░░░░░░
/// ░█░█░█░█
/// ░░░░░░░░
/// █░█░█░█░
/// ░░░░░░░░
/// ░█░█░█░█
/// ░░░░░░░░
/// ```
///
/// ## `BLOCK_UNICODE[18]: U+2592`
/// `"▒"`
///
/// ```text
/// █░█░█░█░
/// ░█░█░█░█
/// █░█░█░█░
/// ░█░█░█░█
/// █░█░█░█░
/// ░█░█░█░█
/// █░█░█░█░
/// ░█░█░█░█
/// ```
///
/// ## `BLOCK_UNICODE[19]: U+2593`
/// `"▓"`
///
/// ```text
/// ████████
/// ░█░█░█░█
/// ████████
/// █░█░█░█░
/// ████████
/// ░█░█░█░█
/// ████████
/// █░█░█░█░
/// ```
///
/// ## `BLOCK_UNICODE[20]: U+2594`
/// `"▔"`
///
/// ```text
/// ████████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BLOCK_UNICODE[21]: U+2595`
/// `"▕"`
///
/// ```text
/// ░░░░░░░█
/// ░░░░░░░█
/// ░░░░░░░█
/// ░░░░░░░█
/// ░░░░░░░█
/// ░░░░░░░█
/// ░░░░░░░█
/// ░░░░░░░█
/// ```
///
/// ## `BLOCK_UNICODE[22]: U+2596`
/// `"▖"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████░░░░
/// ████░░░░
/// ████░░░░
/// ████░░░░
/// ```
///
/// ## `BLOCK_UNICODE[23]: U+2597`
/// `"▗"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░████
/// ░░░░████
/// ░░░░████
/// ░░░░████
/// ```
///
/// ## `BLOCK_UNICODE[24]: U+2598`
/// `"▘"`
///
/// ```text
/// ████░░░░
/// ████░░░░
/// ████░░░░
/// ████░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BLOCK_UNICODE[25]: U+2599`
/// `"▙"`
///
/// ```text
/// ████░░░░
/// ████░░░░
/// ████░░░░
/// ████░░░░
/// ████████
/// ████████
/// ████████
/// ████████
/// ```
///
/// ## `BLOCK_UNICODE[26]: U+259A`
/// `"▚"`
///
/// ```text
/// ████░░░░
/// ████░░░░
/// ████░░░░
/// ████░░░░
/// ░░░░████
/// ░░░░████
/// ░░░░████
/// ░░░░████
/// ```
///
/// ## `BLOCK_UNICODE[27]: U+259B`
/// `"▛"`
///
/// ```text
/// ████████
/// ████████
/// ████████
/// ████████
/// ████░░░░
/// ████░░░░
/// ████░░░░
/// ████░░░░
/// ```
///
/// ## `BLOCK_UNICODE[28]: U+259C`
/// `"▜"`
///
/// ```text
/// ████████
/// ████████
/// ████████
/// ████████
/// ░░░░████
/// ░░░░████
/// ░░░░████
/// ░░░░████
/// ```
///
/// ## `BLOCK_UNICODE[29]: U+259D`
/// `"▝"`
///
/// ```text
/// ░░░░████
/// ░░░░████
/// ░░░░████
/// ░░░░████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BLOCK_UNICODE[30]: U+259E`
/// `"▞"`
///
/// ```text
/// ░░░░████
/// ░░░░████
/// ░░░░████
/// ░░░░████
/// ████░░░░
/// ████░░░░
/// ████░░░░
/// ████░░░░
/// ```
///
/// ## `BLOCK_UNICODE[31]: U+259F`
/// `"▟"`
///
/// ```text
/// ░░░░████
/// ░░░░████
/// ░░░░████
/// ░░░░████
/// ████████
/// ████████
/// ████████
/// ████████
/// ```
pub const BLOCK_FONTS: BlockFonts = BlockFonts(BLOCK_UNICODE);

/// Strong-typed collection wrapper for [BLOCK_UNICODE](./constant.BLOCK_UNICODE.html).
pub struct BlockFonts([FontUnicode; 32]);

impl BlockFonts {
    /// Create a new collection of `BLOCK_UNICODE` fonts.
    pub fn new() -> Self {
        BlockFonts(BLOCK_UNICODE)
    }
}

impl fmt::Debug for BlockFonts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, stringify!(BLOCK_UNICODE))
    }
}

impl PartialEq for BlockFonts {
    fn eq(&self, other: &BlockFonts) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(true, |eq, (a, b)| eq && a == b)
    }
}

impl Default for BlockFonts {
    fn default() -> Self {
        BlockFonts::new()
    }
}

impl UnicodeFonts for BlockFonts {
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
    fn block_fonts_inner_array_is_sorted_by_unicode_key() {
        let mut sorted = BLOCK_UNICODE;
        sorted.sort_by_key(|f| f.char());
        for (idx, key) in sorted.iter().enumerate() {
            assert_eq!(BLOCK_UNICODE[idx].char(), key.char());
        }
    }

    #[test]
    fn block_set_implements_default_trait_with_method_new() {
        let block_set: BlockFonts = Default::default();
        assert_eq!(block_set, BlockFonts::new());
    }

    #[test]
    fn block_fonts_constant_is_equal_to_a_new_instance() {
        assert_eq!(BLOCK_FONTS, BlockFonts::new());
    }

    #[test]
    fn block_fonts_constant_wraps_basic_unicode_constant() {
        let block = BlockFonts::new();
        assert!(block.0.len() == BLOCK_UNICODE.len());
        for (idx, font) in block.0.iter().enumerate() {
            assert_eq!(font, &BLOCK_UNICODE[idx]);
        }
    }
}
