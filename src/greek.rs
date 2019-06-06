//! Greek Characters. `U+0390 - U+03C9`
use super::{legacy::GREEK_LEGACY, FontUnicode, UnicodeFonts};
use core::fmt;

/// A constant `[FontUnicode; 128]`, for Greek fonts (`U+0390` - `U+03C9`).
pub const GREEK_UNICODE: [FontUnicode; 58] = [
    FontUnicode('\u{0390}', GREEK_LEGACY[0]),
    FontUnicode('\u{0391}', GREEK_LEGACY[1]),
    FontUnicode('\u{0392}', GREEK_LEGACY[2]),
    FontUnicode('\u{0393}', GREEK_LEGACY[3]),
    FontUnicode('\u{0394}', GREEK_LEGACY[4]),
    FontUnicode('\u{0395}', GREEK_LEGACY[5]),
    FontUnicode('\u{0396}', GREEK_LEGACY[6]),
    FontUnicode('\u{0397}', GREEK_LEGACY[7]),
    FontUnicode('\u{0398}', GREEK_LEGACY[8]),
    FontUnicode('\u{0399}', GREEK_LEGACY[9]),
    FontUnicode('\u{039A}', GREEK_LEGACY[10]),
    FontUnicode('\u{039B}', GREEK_LEGACY[11]),
    FontUnicode('\u{039C}', GREEK_LEGACY[12]),
    FontUnicode('\u{039D}', GREEK_LEGACY[13]),
    FontUnicode('\u{039E}', GREEK_LEGACY[14]),
    FontUnicode('\u{039F}', GREEK_LEGACY[15]),
    FontUnicode('\u{03A0}', GREEK_LEGACY[16]),
    FontUnicode('\u{03A1}', GREEK_LEGACY[17]),
    FontUnicode('\u{03A2}', GREEK_LEGACY[18]),
    FontUnicode('\u{03A3}', GREEK_LEGACY[19]),
    FontUnicode('\u{03A4}', GREEK_LEGACY[20]),
    FontUnicode('\u{03A5}', GREEK_LEGACY[21]),
    FontUnicode('\u{03A6}', GREEK_LEGACY[22]),
    FontUnicode('\u{03A7}', GREEK_LEGACY[23]),
    FontUnicode('\u{03A8}', GREEK_LEGACY[24]),
    FontUnicode('\u{03A9}', GREEK_LEGACY[25]),
    FontUnicode('\u{0399}', GREEK_LEGACY[26]),
    FontUnicode('\u{03A5}', GREEK_LEGACY[27]),
    FontUnicode('\u{03AC}', GREEK_LEGACY[28]),
    FontUnicode('\u{03AD}', GREEK_LEGACY[29]),
    FontUnicode('\u{03AE}', GREEK_LEGACY[30]),
    FontUnicode('\u{03AF}', GREEK_LEGACY[31]),
    FontUnicode('\u{03B0}', GREEK_LEGACY[32]),
    FontUnicode('\u{03B1}', GREEK_LEGACY[33]),
    FontUnicode('\u{03B2}', GREEK_LEGACY[34]),
    FontUnicode('\u{03B3}', GREEK_LEGACY[35]),
    FontUnicode('\u{03B4}', GREEK_LEGACY[36]),
    FontUnicode('\u{03B5}', GREEK_LEGACY[37]),
    FontUnicode('\u{03B6}', GREEK_LEGACY[38]),
    FontUnicode('\u{03B7}', GREEK_LEGACY[39]),
    FontUnicode('\u{03B8}', GREEK_LEGACY[40]),
    FontUnicode('\u{03B9}', GREEK_LEGACY[41]),
    FontUnicode('\u{03BA}', GREEK_LEGACY[42]),
    FontUnicode('\u{03BB}', GREEK_LEGACY[43]),
    FontUnicode('\u{03BC}', GREEK_LEGACY[44]),
    FontUnicode('\u{03BD}', GREEK_LEGACY[45]),
    FontUnicode('\u{03BE}', GREEK_LEGACY[46]),
    FontUnicode('\u{03BF}', GREEK_LEGACY[47]),
    FontUnicode('\u{03C0}', GREEK_LEGACY[48]),
    FontUnicode('\u{03C1}', GREEK_LEGACY[49]),
    FontUnicode('\u{03C2}', GREEK_LEGACY[50]),
    FontUnicode('\u{03C3}', GREEK_LEGACY[51]),
    FontUnicode('\u{03C4}', GREEK_LEGACY[52]),
    FontUnicode('\u{03C5}', GREEK_LEGACY[53]),
    FontUnicode('\u{03C6}', GREEK_LEGACY[54]),
    FontUnicode('\u{03C7}', GREEK_LEGACY[55]),
    FontUnicode('\u{03C8}', GREEK_LEGACY[56]),
    FontUnicode('\u{03C9}', GREEK_LEGACY[57]),
];

/// A convenient constant for Greek fonts (`U+0390` - `U+03C9`), that implements the `UnicodeFonts` trait.
///
/// ## `GREEK_UNICODE[0]`: `0x0390` `"ΐ"`
///
/// ```text
/// █░██░█░░
/// ░░░░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░█░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[1]`: `0x0391` `"Α"`
///
/// ```text
/// ░░██░░░░
/// ░████░░░
/// ██░░██░░
/// ██░░██░░
/// ██████░░
/// ██░░██░░
/// ██░░██░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[2]`: `0x0392` `"Β"`
///
/// ```text
/// ██████░░
/// ░██░░██░
/// ░██░░██░
/// ░█████░░
/// ░██░░██░
/// ░██░░██░
/// ██████░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[3]`: `0x0393` `"Γ"`
///
/// ```text
/// ██████░░
/// ██░░██░░
/// ██░░░░░░
/// ██░░░░░░
/// ██░░░░░░
/// ██░░░░░░
/// ██░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[4]`: `0x0394` `"Δ"`
///
/// ```text
/// ░░░█░░░░
/// ░░███░░░
/// ░░███░░░
/// ░██░██░░
/// ░██░██░░
/// ██░░░██░
/// ███████░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[5]`: `0x0395` `"Ε"`
///
/// ```text
/// ███████░
/// ░██░░░█░
/// ░██░█░░░
/// ░████░░░
/// ░██░█░░░
/// ░██░░░█░
/// ███████░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[6]`: `0x0396` `"Ζ"`
///
/// ```text
/// ███████░
/// ██░░░██░
/// █░░░██░░
/// ░░░██░░░
/// ░░██░░█░
/// ░██░░██░
/// ███████░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[7]`: `0x0397` `"Η"`
///
/// ```text
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██████░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[8]`: `0x0398` `"Θ"`
///
/// ```text
/// ░░███░░░
/// ░██░██░░
/// ██░░░██░
/// ███████░
/// ██░░░██░
/// ░██░██░░
/// ░░███░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[9]`: `0x0399` `"Ι"`
///
/// ```text
/// ░████░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[10]`: `0x039A` `"Κ"`
///
/// ```text
/// ███░░██░
/// ░██░░██░
/// ░██░██░░
/// ░████░░░
/// ░██░██░░
/// ░██░░██░
/// ███░░██░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[11]`: `0x039B` `"Λ"`
///
/// ```text
/// ░░░█░░░░
/// ░░███░░░
/// ░░███░░░
/// ░██░██░░
/// ░██░██░░
/// ██░░░██░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[12]`: `0x039C` `"Μ"`
///
/// ```text
/// ██░░░██░
/// ███░███░
/// ███████░
/// ███████░
/// ██░█░██░
/// ██░░░██░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[13]`: `0x039D` `"Ν"`
///
/// ```text
/// ██░░░██░
/// ███░░██░
/// ████░██░
/// ██░████░
/// ██░░███░
/// ██░░░██░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[14]`: `0x039E` `"Ξ"`
///
/// ```text
/// ███████░
/// ██░░░██░
/// ░░░░░░░░
/// ░█████░░
/// ░░░░░░░░
/// ██░░░██░
/// ███████░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[15]`: `0x039F` `"Ο"`
///
/// ```text
/// ░░███░░░
/// ░██░██░░
/// ██░░░██░
/// ██░░░██░
/// ██░░░██░
/// ░██░██░░
/// ░░███░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[16]`: `0x03A0` `"Π"`
///
/// ```text
/// ███████░
/// ░██░██░░
/// ░██░██░░
/// ░██░██░░
/// ░██░██░░
/// ░██░██░░
/// ░██░██░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[17]`: `0x03A1` `"Ρ"`
///
/// ```text
/// ██████░░
/// ░██░░██░
/// ░██░░██░
/// ░█████░░
/// ░██░░░░░
/// ░██░░░░░
/// ████░░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[18]`: `0x03A2` `"\u{3a2}"`
///
/// ```text
/// ░░░░░░░░
/// █░░░░░░░
/// ░█░░░░░░
/// ░░█░░░░░
/// ████░░█░
/// ░░░░█░░█
/// ░░░░░█░█
/// ░░░░░░█░
/// ```
///
/// ## `GREEK_UNICODE[19]`: `0x03A3` `"Σ"`
///
/// ```text
/// ███████░
/// ██░░░██░
/// ░██░░░░░
/// ░░██░░░░
/// ░██░░░░░
/// ██░░░██░
/// ███████░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[20]`: `0x03A4` `"Τ"`
///
/// ```text
/// ██████░░
/// █░██░█░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[21]`: `0x03A5` `"Υ"`
///
/// ```text
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[22]`: `0x03A6` `"Φ"`
///
/// ```text
/// ░░░██░░░
/// ░██████░
/// ██░██░██
/// ██░██░██
/// ██░██░██
/// ░██████░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[23]`: `0x03A7` `"Χ"`
///
/// ```text
/// ██░░░██░
/// ██░░░██░
/// ░██░██░░
/// ░░███░░░
/// ░██░██░░
/// ██░░░██░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[24]`: `0x03A8` `"Ψ"`
///
/// ```text
/// ██░██░██
/// ██░██░██
/// ██░██░██
/// ░██████░
/// ░░░██░░░
/// ░░░██░░░
/// ░░████░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[25]`: `0x03A9` `"Ω"`
///
/// ```text
/// ░█████░░
/// ██░░░██░
/// ██░░░██░
/// ██░░░██░
/// ░██░██░░
/// ░██░██░░
/// ███░███░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[26]`: `0x0399` `"Ι"`
///
/// ```text
/// ██░░██░░
/// ░░░░░░░░
/// ░████░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[27]`: `0x03A5` `"Υ"`
///
/// ```text
/// ██░░██░░
/// ░░░░░░░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░██░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[28]`: `0x03AC` `"ά"`
///
/// ```text
/// ░░░░███░
/// ░░░░░░░░
/// ░███░██░
/// ██░███░░
/// ██░░█░░░
/// ██░███░░
/// ░███░██░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[29]`: `0x03AD` `"έ"`
///
/// ```text
/// ░░░███░░
/// ░░░░░░░░
/// ░████░░░
/// ██░░░░░░
/// ░███░░░░
/// ██░░░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[30]`: `0x03AE` `"ή"`
///
/// ```text
/// ░░░███░░
/// ░░░░░░░░
/// █████░░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░░░░██░░
/// ```
///
/// ## `GREEK_UNICODE[31]`: `0x03AF` `"ί"`
///
/// ```text
/// ░░░███░░
/// ░░░░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░█░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[32]`: `0x03B0` `"ΰ"`
///
/// ```text
/// █░██░█░░
/// ░░░░░░░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[33]`: `0x03B1` `"α"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░███░██░
/// ██░███░░
/// ██░░█░░░
/// ██░███░░
/// ░███░██░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[34]`: `0x03B2` `"β"`
///
/// ```text
/// ░░░░░░░░
/// ░████░░░
/// ██░░██░░
/// █████░░░
/// ██░░██░░
/// █████░░░
/// ██░░░░░░
/// ██░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[35]`: `0x03B3` `"γ"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[36]`: `0x03B4` `"δ"`
///
/// ```text
/// ░░░███░░
/// ░░██░░░░
/// ░░░██░░░
/// ░█████░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[37]`: `0x03B5` `"ε"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░████░░░
/// ██░░░░░░
/// ░███░░░░
/// ██░░░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[38]`: `0x03B6` `"ζ"`
///
/// ```text
/// ░░░░░░░░
/// ██████░░
/// ░██░░░░░
/// ██░░░░░░
/// ██░░░░░░
/// ░████░░░
/// ░░░░██░░
/// ░░███░░░
/// ```
///
/// ## `GREEK_UNICODE[39]`: `0x03B7` `"η"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// █████░░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░░░░██░░
/// ```
///
/// ## `GREEK_UNICODE[40]`: `0x03B8` `"θ"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░████░░░
/// ██░░██░░
/// ██████░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[41]`: `0x03B9` `"ι"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░█░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[42]`: `0x03BA` `"κ"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░░██░░
/// ██░██░░░
/// ████░░░░
/// ██░██░░░
/// ██░░██░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[43]`: `0x03BB` `"λ"`
///
/// ```text
/// ░░░░░░░░
/// ██░░░░░░
/// ░██░░░░░
/// ░░██░░░░
/// ░░███░░░
/// ░██░██░░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[44]`: `0x03BC` `"μ"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░██░░██░
/// ░██░░██░
/// ░██░░██░
/// ░█████░░
/// ░██░░░░░
/// ██░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[45]`: `0x03BD` `"ν"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[46]`: `0x03BE` `"ξ"`
///
/// ```text
/// ░████░░░
/// ██░░░░░░
/// ░███░░░░
/// ██░░░░░░
/// ██░░░░░░
/// ░████░░░
/// ░░░░██░░
/// ░░███░░░
/// ```
///
/// ## `GREEK_UNICODE[47]`: `0x03BF` `"ο"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░████░░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[48]`: `0x03C0` `"π"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ███████░
/// ░██░██░░
/// ░██░██░░
/// ░██░██░░
/// ░██░██░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[49]`: `0x03C1` `"ρ"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░████░░
/// ░██░░██░
/// ░██░░██░
/// ░██░██░░
/// ░██░░░░░
/// ░██░░░░░
/// ```
///
/// ## `GREEK_UNICODE[50]`: `0x03C2` `"ς"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░█████░░
/// ██░░░░░░
/// ██░░░░░░
/// ░████░░░
/// ░░░░██░░
/// ░░███░░░
/// ```
///
/// ## `GREEK_UNICODE[51]`: `0x03C3` `"σ"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░██████░
/// ██░██░░░
/// ██░██░░░
/// ██░██░░░
/// ░███░░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[52]`: `0x03C4` `"τ"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░██████░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░█░
/// ░░░░██░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[53]`: `0x03C5` `"υ"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[54]`: `0x03C6` `"φ"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░██░███░
/// ██░██░██
/// ██░██░██
/// ░██████░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[55]`: `0x03C7` `"χ"`
///
/// ```text
/// ░░░░░░░░
/// ██░░░██░
/// ░██░██░░
/// ░░███░░░
/// ░░███░░░
/// ░██░██░░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[56]`: `0x03C8` `"ψ"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░██░██
/// ██░██░██
/// ██░██░██
/// ░██████░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `GREEK_UNICODE[57]`: `0x03C9` `"ω"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░██░██░░
/// ██░░░██░
/// ██░█░██░
/// ███████░
/// ░██░██░░
/// ░░░░░░░░
/// ```
pub const GREEK_FONTS: GreekFonts = GreekFonts(GREEK_UNICODE);

/// Strong-typed collection wrapper for [GREEK_UNICODE](./constant.GREEK_UNICODE.html).
pub struct GreekFonts([FontUnicode; 58]);

impl GreekFonts {
    pub fn new() -> Self {
        GreekFonts(GREEK_UNICODE)
    }
}

impl fmt::Debug for GreekFonts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, stringify!(GREEK_UNICODE))
    }
}

impl PartialEq for GreekFonts {
    fn eq(&self, other: &GreekFonts) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(true, |eq, (a, b)| eq && a == b)
    }
}

impl Default for GreekFonts {
    fn default() -> Self {
        GreekFonts::new()
    }
}

impl UnicodeFonts for GreekFonts {
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
            if font.is_whitespace() {
                println!("## {:3?}: 0x{:04X} \" \"", idx, font.char() as u32);
                continue;
            }
            println!(
                "## `{:?}[{:?}]`: `0x{:04X}` `{:?}`",
                self,
                idx,
                font.char() as u32,
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
    fn greek_set_implements_default_trait_with_method_new() {
        let greek_set: GreekFonts = Default::default();
        assert_eq!(greek_set, GreekFonts::new());
    }

    #[test]
    fn greek_fonts_constant_is_equal_to_a_new_instance() {
        assert_eq!(GREEK_FONTS, GreekFonts::new());
    }

    #[test]
    fn greek_fonts_constant_wraps_basic_unicode_constant() {
        let greek = GreekFonts::new();
        assert!(greek.0.len() == GREEK_UNICODE.len());
        for (idx, font) in greek.0.iter().enumerate() {
            assert_eq!(font, &GREEK_UNICODE[idx]);
        }
    }
}
