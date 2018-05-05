//!  Block Elements. `U+2580` - `U+259F`
use super::{legacy::BLOCK_LEGACY, FontUtf16, Utf16Fonts};
use std::fmt;

/// A constant `[FontUtf16; 32]`, for Block Element fonts (`U+2580` - `U+259F`).
///
/// ## `BLOCK_UTF16[0]`: `U+2580` `"▀"`
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
/// ## `BLOCK_UTF16[1]`: `U+2581` `"▁"`
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
/// ## `BLOCK_UTF16[2]`: `U+2582` `"▂"`
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
/// ## `BLOCK_UTF16[3]`: `U+2583` `"▃"`
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
/// ## `BLOCK_UTF16[4]`: `U+2584` `"▄"`
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
/// ## `BLOCK_UTF16[5]`: `U+2585` `"▅"`
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
/// ## `BLOCK_UTF16[6]`: `U+2586` `"▆"`
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
/// ## `BLOCK_UTF16[7]`: `U+2587` `"▇"`
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
/// ## `BLOCK_UTF16[8]`: `U+2588` `"█"`
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
/// ## `BLOCK_UTF16[9]`: `U+2589` `"▉"`
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
/// ## `BLOCK_UTF16[10]`: `U+258A` `"▊"`
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
/// ## `BLOCK_UTF16[11]`: `U+258B` `"▋"`
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
/// ## `BLOCK_UTF16[12]`: `U+258C` `"▌"`
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
/// ## `BLOCK_UTF16[13]`: `U+258D` `"▍"`
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
/// ## `BLOCK_UTF16[14]`: `U+258E` `"▎"`
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
/// ## `BLOCK_UTF16[15]`: `U+258F` `"▏"`
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
/// ## `BLOCK_UTF16[16]`: `U+2590` `"▐"`
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
/// ## `BLOCK_UTF16[17]`: `U+2591` `"░"`
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
/// ## `BLOCK_UTF16[18]`: `U+2592` `"▒"`
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
/// ## `BLOCK_UTF16[19]`: `U+2593` `"▓"`
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
/// ## `BLOCK_UTF16[20]`: `U+2594` `"▔"`
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
/// ## `BLOCK_UTF16[21]`: `U+2595` `"▕"`
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
/// ## `BLOCK_UTF16[22]`: `U+2596` `"▖"`
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
/// ## `BLOCK_UTF16[23]`: `U+2597` `"▗"`
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
/// ## `BLOCK_UTF16[24]`: `U+2598` `"▘"`
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
/// ## `BLOCK_UTF16[25]`: `U+2599` `"▙"`
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
/// ## `BLOCK_UTF16[26]`: `U+259A` `"▚"`
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
/// ## `BLOCK_UTF16[27]`: `U+259B` `"▛"`
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
/// ## `BLOCK_UTF16[28]`: `U+259C` `"▜"`
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
/// ## `BLOCK_UTF16[29]`: `U+259D` `"▝"`
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
/// ## `BLOCK_UTF16[30]`: `U+259E` `"▞"`
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
/// ## `BLOCK_UTF16[31]`: `U+259F` `"▟"`
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
pub const BLOCK_UTF16: [FontUtf16; 32] = [
    FontUtf16(0x2580 as u16, BLOCK_LEGACY[0]),
    FontUtf16(0x2581 as u16, BLOCK_LEGACY[1]),
    FontUtf16(0x2582 as u16, BLOCK_LEGACY[2]),
    FontUtf16(0x2583 as u16, BLOCK_LEGACY[3]),
    FontUtf16(0x2584 as u16, BLOCK_LEGACY[4]),
    FontUtf16(0x2585 as u16, BLOCK_LEGACY[5]),
    FontUtf16(0x2586 as u16, BLOCK_LEGACY[6]),
    FontUtf16(0x2587 as u16, BLOCK_LEGACY[7]),
    FontUtf16(0x2588 as u16, BLOCK_LEGACY[8]),
    FontUtf16(0x2589 as u16, BLOCK_LEGACY[9]),
    FontUtf16(0x258A as u16, BLOCK_LEGACY[10]),
    FontUtf16(0x258B as u16, BLOCK_LEGACY[11]),
    FontUtf16(0x258C as u16, BLOCK_LEGACY[12]),
    FontUtf16(0x258D as u16, BLOCK_LEGACY[13]),
    FontUtf16(0x258E as u16, BLOCK_LEGACY[14]),
    FontUtf16(0x258F as u16, BLOCK_LEGACY[15]),
    FontUtf16(0x2590 as u16, BLOCK_LEGACY[16]),
    FontUtf16(0x2591 as u16, BLOCK_LEGACY[17]),
    FontUtf16(0x2592 as u16, BLOCK_LEGACY[18]),
    FontUtf16(0x2593 as u16, BLOCK_LEGACY[19]),
    FontUtf16(0x2594 as u16, BLOCK_LEGACY[20]),
    FontUtf16(0x2595 as u16, BLOCK_LEGACY[21]),
    FontUtf16(0x2596 as u16, BLOCK_LEGACY[22]),
    FontUtf16(0x2597 as u16, BLOCK_LEGACY[23]),
    FontUtf16(0x2598 as u16, BLOCK_LEGACY[24]),
    FontUtf16(0x2599 as u16, BLOCK_LEGACY[25]),
    FontUtf16(0x259A as u16, BLOCK_LEGACY[26]),
    FontUtf16(0x259B as u16, BLOCK_LEGACY[27]),
    FontUtf16(0x259C as u16, BLOCK_LEGACY[28]),
    FontUtf16(0x259D as u16, BLOCK_LEGACY[29]),
    FontUtf16(0x259E as u16, BLOCK_LEGACY[30]),
    FontUtf16(0x259F as u16, BLOCK_LEGACY[31]),
];

/// A convenient constant for Block Element fonts (`U+2580` - `U+259F`), that implements the [`Utf16Fonts`](./utf16/trait.Utf16Fonts.html) trait.
pub const BLOCK_FONTS: BlockFonts = BlockFonts(BLOCK_UTF16);

/// Strong-typed collection wrapper for [BLOCK_UTF16](./constant.BLOCK_UTF16.html).
pub struct BlockFonts([FontUtf16; 32]);

impl BlockFonts {
    /// Create a new collection of `BLOCK_UTF16` fonts.
    pub fn new() -> Self {
        BlockFonts(BLOCK_UTF16)
    }
}

impl fmt::Debug for BlockFonts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", stringify!(BLOCK_UTF16))
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

impl Utf16Fonts for BlockFonts {
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
                "## `{:?}[{:?}]`: `U+{:04X}` `{:?}`",
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn block_fonts_constant_wraps_basic_utf16_constant() {
        let block = BlockFonts::new();
        assert!(block.0.len() == BLOCK_UTF16.len());
        for (idx, font) in block.0.iter().enumerate() {
            assert_eq!(font, &BLOCK_UTF16[idx]);
        }
    }
}
