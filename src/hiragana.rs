//! Hiragana. `U+3040 - U+309F`
use super::{legacy::HIRAGANA_LEGACY, FontUnicode, UnicodeFonts};
use core::fmt;

/// A constant `[FontUnicode; 96]`, for Hiragana fonts (`U+3040` - `U+309F`).
pub const HIRAGANA_UNICODE: [FontUnicode; 96] = [
    FontUnicode('\u{3040}', HIRAGANA_LEGACY[0]),
    FontUnicode('\u{3041}', HIRAGANA_LEGACY[1]),
    FontUnicode('\u{3042}', HIRAGANA_LEGACY[2]),
    FontUnicode('\u{3043}', HIRAGANA_LEGACY[3]),
    FontUnicode('\u{3044}', HIRAGANA_LEGACY[4]),
    FontUnicode('\u{3045}', HIRAGANA_LEGACY[5]),
    FontUnicode('\u{3046}', HIRAGANA_LEGACY[6]),
    FontUnicode('\u{3047}', HIRAGANA_LEGACY[7]),
    FontUnicode('\u{3048}', HIRAGANA_LEGACY[8]),
    FontUnicode('\u{3049}', HIRAGANA_LEGACY[9]),
    FontUnicode('\u{304A}', HIRAGANA_LEGACY[10]),
    FontUnicode('\u{304B}', HIRAGANA_LEGACY[11]),
    FontUnicode('\u{304C}', HIRAGANA_LEGACY[12]),
    FontUnicode('\u{304D}', HIRAGANA_LEGACY[13]),
    FontUnicode('\u{304E}', HIRAGANA_LEGACY[14]),
    FontUnicode('\u{304F}', HIRAGANA_LEGACY[15]),
    FontUnicode('\u{3050}', HIRAGANA_LEGACY[16]),
    FontUnicode('\u{3051}', HIRAGANA_LEGACY[17]),
    FontUnicode('\u{3052}', HIRAGANA_LEGACY[18]),
    FontUnicode('\u{3053}', HIRAGANA_LEGACY[19]),
    FontUnicode('\u{3054}', HIRAGANA_LEGACY[20]),
    FontUnicode('\u{3055}', HIRAGANA_LEGACY[21]),
    FontUnicode('\u{3056}', HIRAGANA_LEGACY[22]),
    FontUnicode('\u{3057}', HIRAGANA_LEGACY[23]),
    FontUnicode('\u{3058}', HIRAGANA_LEGACY[24]),
    FontUnicode('\u{3059}', HIRAGANA_LEGACY[25]),
    FontUnicode('\u{305A}', HIRAGANA_LEGACY[26]),
    FontUnicode('\u{305B}', HIRAGANA_LEGACY[27]),
    FontUnicode('\u{305C}', HIRAGANA_LEGACY[28]),
    FontUnicode('\u{305D}', HIRAGANA_LEGACY[29]),
    FontUnicode('\u{305E}', HIRAGANA_LEGACY[30]),
    FontUnicode('\u{305F}', HIRAGANA_LEGACY[31]),
    FontUnicode('\u{3060}', HIRAGANA_LEGACY[32]),
    FontUnicode('\u{3061}', HIRAGANA_LEGACY[33]),
    FontUnicode('\u{3062}', HIRAGANA_LEGACY[34]),
    FontUnicode('\u{3063}', HIRAGANA_LEGACY[35]),
    FontUnicode('\u{3064}', HIRAGANA_LEGACY[36]),
    FontUnicode('\u{3065}', HIRAGANA_LEGACY[37]),
    FontUnicode('\u{3066}', HIRAGANA_LEGACY[38]),
    FontUnicode('\u{3067}', HIRAGANA_LEGACY[39]),
    FontUnicode('\u{3068}', HIRAGANA_LEGACY[40]),
    FontUnicode('\u{3069}', HIRAGANA_LEGACY[41]),
    FontUnicode('\u{306A}', HIRAGANA_LEGACY[42]),
    FontUnicode('\u{306B}', HIRAGANA_LEGACY[43]),
    FontUnicode('\u{306C}', HIRAGANA_LEGACY[44]),
    FontUnicode('\u{306D}', HIRAGANA_LEGACY[45]),
    FontUnicode('\u{306E}', HIRAGANA_LEGACY[46]),
    FontUnicode('\u{306F}', HIRAGANA_LEGACY[47]),
    FontUnicode('\u{3070}', HIRAGANA_LEGACY[48]),
    FontUnicode('\u{3071}', HIRAGANA_LEGACY[49]),
    FontUnicode('\u{3072}', HIRAGANA_LEGACY[50]),
    FontUnicode('\u{3073}', HIRAGANA_LEGACY[51]),
    FontUnicode('\u{3074}', HIRAGANA_LEGACY[52]),
    FontUnicode('\u{3075}', HIRAGANA_LEGACY[53]),
    FontUnicode('\u{3076}', HIRAGANA_LEGACY[54]),
    FontUnicode('\u{3077}', HIRAGANA_LEGACY[55]),
    FontUnicode('\u{3078}', HIRAGANA_LEGACY[56]),
    FontUnicode('\u{3079}', HIRAGANA_LEGACY[57]),
    FontUnicode('\u{307A}', HIRAGANA_LEGACY[58]),
    FontUnicode('\u{307B}', HIRAGANA_LEGACY[59]),
    FontUnicode('\u{307C}', HIRAGANA_LEGACY[60]),
    FontUnicode('\u{307D}', HIRAGANA_LEGACY[61]),
    FontUnicode('\u{307E}', HIRAGANA_LEGACY[62]),
    FontUnicode('\u{307F}', HIRAGANA_LEGACY[63]),
    FontUnicode('\u{3080}', HIRAGANA_LEGACY[64]),
    FontUnicode('\u{3081}', HIRAGANA_LEGACY[65]),
    FontUnicode('\u{3082}', HIRAGANA_LEGACY[66]),
    FontUnicode('\u{3083}', HIRAGANA_LEGACY[67]),
    FontUnicode('\u{3084}', HIRAGANA_LEGACY[68]),
    FontUnicode('\u{3085}', HIRAGANA_LEGACY[69]),
    FontUnicode('\u{3086}', HIRAGANA_LEGACY[70]),
    FontUnicode('\u{3087}', HIRAGANA_LEGACY[71]),
    FontUnicode('\u{3088}', HIRAGANA_LEGACY[72]),
    FontUnicode('\u{3089}', HIRAGANA_LEGACY[73]),
    FontUnicode('\u{308A}', HIRAGANA_LEGACY[74]),
    FontUnicode('\u{308B}', HIRAGANA_LEGACY[75]),
    FontUnicode('\u{308C}', HIRAGANA_LEGACY[76]),
    FontUnicode('\u{308D}', HIRAGANA_LEGACY[77]),
    FontUnicode('\u{308E}', HIRAGANA_LEGACY[78]),
    FontUnicode('\u{308F}', HIRAGANA_LEGACY[79]),
    FontUnicode('\u{3090}', HIRAGANA_LEGACY[80]),
    FontUnicode('\u{3091}', HIRAGANA_LEGACY[81]),
    FontUnicode('\u{3092}', HIRAGANA_LEGACY[82]),
    FontUnicode('\u{3093}', HIRAGANA_LEGACY[83]),
    FontUnicode('\u{3094}', HIRAGANA_LEGACY[84]),
    FontUnicode('\u{3095}', HIRAGANA_LEGACY[85]),
    FontUnicode('\u{3096}', HIRAGANA_LEGACY[86]),
    FontUnicode('\u{3097}', HIRAGANA_LEGACY[87]),
    FontUnicode('\u{3098}', HIRAGANA_LEGACY[88]),
    FontUnicode('\u{3099}', HIRAGANA_LEGACY[89]),
    FontUnicode('\u{309A}', HIRAGANA_LEGACY[90]),
    FontUnicode('\u{309B}', HIRAGANA_LEGACY[91]),
    FontUnicode('\u{309C}', HIRAGANA_LEGACY[92]),
    FontUnicode('\u{309D}', HIRAGANA_LEGACY[93]),
    FontUnicode('\u{309E}', HIRAGANA_LEGACY[94]),
    FontUnicode('\u{309F}', HIRAGANA_LEGACY[95]),
];

/// A convenient constant for Hiragana fonts (`U+3040` - `U+309F`), that implements the [`UnicodeFonts`](./char/trait.UnicodeFonts.html) trait.
pub const HIRAGANA_FONTS: HiraganaFonts = HiraganaFonts(HIRAGANA_UNICODE);

/// Strong-typed collection wrapper for [HIRAGANA_UNICODE](./constant.HIRAGANA_UNICODE.html).
pub struct HiraganaFonts([FontUnicode; 96]);

impl HiraganaFonts {
    pub fn new() -> Self {
        HiraganaFonts(HIRAGANA_UNICODE)
    }
}

impl fmt::Debug for HiraganaFonts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", stringify!(HIRAGANA_UNICODE))
    }
}

impl PartialEq for HiraganaFonts {
    fn eq(&self, other: &HiraganaFonts) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(true, |eq, (a, b)| eq && a == b)
    }
}

impl Default for HiraganaFonts {
    fn default() -> Self {
        HiraganaFonts::new()
    }
}

impl UnicodeFonts for HiraganaFonts {
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
        self.0.into_iter().fold(Vec::with_capacity(128), |mut v, font| {
            v.push((font.char(), *font));
            v
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hiragana_set_implements_default_trait_with_method_new() {
        let hiragana_set: HiraganaFonts = Default::default();
        assert_eq!(hiragana_set, HiraganaFonts::new());
    }

    #[test]
    fn hiragana_fonts_constant_is_equal_to_a_new_instance() {
        assert_eq!(HIRAGANA_FONTS, HiraganaFonts::new());
    }

    #[test]
    fn hiragana_fonts_constant_wraps_basic_unicode_constant() {
        let hiragana = HiraganaFonts::new();
        assert!(hiragana.0.len() == HIRAGANA_UNICODE.len());
        for (idx, font) in hiragana.0.iter().enumerate() {
            assert_eq!(font, &HIRAGANA_UNICODE[idx]);
        }
    }
}
