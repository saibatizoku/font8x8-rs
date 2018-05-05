//! Hiragana. `U+3040 - U+309F`
use super::{legacy::HIRAGANA_LEGACY, FontUtf16, Utf16Fonts};
use std::fmt;

/// A constant `[FontUtf16; 96]`, for Hiragana fonts (`U+3040` - `U+309F`).
pub const HIRAGANA_UTF16: [FontUtf16; 96] = [
    FontUtf16(0x3040 as u16, HIRAGANA_LEGACY[0]),
    FontUtf16(0x3041 as u16, HIRAGANA_LEGACY[1]),
    FontUtf16(0x3042 as u16, HIRAGANA_LEGACY[2]),
    FontUtf16(0x3043 as u16, HIRAGANA_LEGACY[3]),
    FontUtf16(0x3044 as u16, HIRAGANA_LEGACY[4]),
    FontUtf16(0x3045 as u16, HIRAGANA_LEGACY[5]),
    FontUtf16(0x3046 as u16, HIRAGANA_LEGACY[6]),
    FontUtf16(0x3047 as u16, HIRAGANA_LEGACY[7]),
    FontUtf16(0x3048 as u16, HIRAGANA_LEGACY[8]),
    FontUtf16(0x3049 as u16, HIRAGANA_LEGACY[9]),
    FontUtf16(0x304A as u16, HIRAGANA_LEGACY[10]),
    FontUtf16(0x304B as u16, HIRAGANA_LEGACY[11]),
    FontUtf16(0x304C as u16, HIRAGANA_LEGACY[12]),
    FontUtf16(0x304D as u16, HIRAGANA_LEGACY[13]),
    FontUtf16(0x304E as u16, HIRAGANA_LEGACY[14]),
    FontUtf16(0x304F as u16, HIRAGANA_LEGACY[15]),
    FontUtf16(0x3050 as u16, HIRAGANA_LEGACY[16]),
    FontUtf16(0x3051 as u16, HIRAGANA_LEGACY[17]),
    FontUtf16(0x3052 as u16, HIRAGANA_LEGACY[18]),
    FontUtf16(0x3053 as u16, HIRAGANA_LEGACY[19]),
    FontUtf16(0x3054 as u16, HIRAGANA_LEGACY[20]),
    FontUtf16(0x3055 as u16, HIRAGANA_LEGACY[21]),
    FontUtf16(0x3056 as u16, HIRAGANA_LEGACY[22]),
    FontUtf16(0x3057 as u16, HIRAGANA_LEGACY[23]),
    FontUtf16(0x3058 as u16, HIRAGANA_LEGACY[24]),
    FontUtf16(0x3059 as u16, HIRAGANA_LEGACY[25]),
    FontUtf16(0x305A as u16, HIRAGANA_LEGACY[26]),
    FontUtf16(0x305B as u16, HIRAGANA_LEGACY[27]),
    FontUtf16(0x305C as u16, HIRAGANA_LEGACY[28]),
    FontUtf16(0x305D as u16, HIRAGANA_LEGACY[29]),
    FontUtf16(0x305E as u16, HIRAGANA_LEGACY[30]),
    FontUtf16(0x305F as u16, HIRAGANA_LEGACY[31]),
    FontUtf16(0x3060 as u16, HIRAGANA_LEGACY[32]),
    FontUtf16(0x3061 as u16, HIRAGANA_LEGACY[33]),
    FontUtf16(0x3062 as u16, HIRAGANA_LEGACY[34]),
    FontUtf16(0x3063 as u16, HIRAGANA_LEGACY[35]),
    FontUtf16(0x3064 as u16, HIRAGANA_LEGACY[36]),
    FontUtf16(0x3065 as u16, HIRAGANA_LEGACY[37]),
    FontUtf16(0x3066 as u16, HIRAGANA_LEGACY[38]),
    FontUtf16(0x3067 as u16, HIRAGANA_LEGACY[39]),
    FontUtf16(0x3068 as u16, HIRAGANA_LEGACY[40]),
    FontUtf16(0x3069 as u16, HIRAGANA_LEGACY[41]),
    FontUtf16(0x306A as u16, HIRAGANA_LEGACY[42]),
    FontUtf16(0x306B as u16, HIRAGANA_LEGACY[43]),
    FontUtf16(0x306C as u16, HIRAGANA_LEGACY[44]),
    FontUtf16(0x306D as u16, HIRAGANA_LEGACY[45]),
    FontUtf16(0x306E as u16, HIRAGANA_LEGACY[46]),
    FontUtf16(0x306F as u16, HIRAGANA_LEGACY[47]),
    FontUtf16(0x3070 as u16, HIRAGANA_LEGACY[48]),
    FontUtf16(0x3071 as u16, HIRAGANA_LEGACY[49]),
    FontUtf16(0x3072 as u16, HIRAGANA_LEGACY[50]),
    FontUtf16(0x3073 as u16, HIRAGANA_LEGACY[51]),
    FontUtf16(0x3074 as u16, HIRAGANA_LEGACY[52]),
    FontUtf16(0x3075 as u16, HIRAGANA_LEGACY[53]),
    FontUtf16(0x3076 as u16, HIRAGANA_LEGACY[54]),
    FontUtf16(0x3077 as u16, HIRAGANA_LEGACY[55]),
    FontUtf16(0x3078 as u16, HIRAGANA_LEGACY[56]),
    FontUtf16(0x3079 as u16, HIRAGANA_LEGACY[57]),
    FontUtf16(0x307A as u16, HIRAGANA_LEGACY[58]),
    FontUtf16(0x307B as u16, HIRAGANA_LEGACY[59]),
    FontUtf16(0x307C as u16, HIRAGANA_LEGACY[60]),
    FontUtf16(0x307D as u16, HIRAGANA_LEGACY[61]),
    FontUtf16(0x307E as u16, HIRAGANA_LEGACY[62]),
    FontUtf16(0x307F as u16, HIRAGANA_LEGACY[63]),
    FontUtf16(0x3080 as u16, HIRAGANA_LEGACY[64]),
    FontUtf16(0x3081 as u16, HIRAGANA_LEGACY[65]),
    FontUtf16(0x3082 as u16, HIRAGANA_LEGACY[66]),
    FontUtf16(0x3083 as u16, HIRAGANA_LEGACY[67]),
    FontUtf16(0x3084 as u16, HIRAGANA_LEGACY[68]),
    FontUtf16(0x3085 as u16, HIRAGANA_LEGACY[69]),
    FontUtf16(0x3086 as u16, HIRAGANA_LEGACY[70]),
    FontUtf16(0x3087 as u16, HIRAGANA_LEGACY[71]),
    FontUtf16(0x3088 as u16, HIRAGANA_LEGACY[72]),
    FontUtf16(0x3089 as u16, HIRAGANA_LEGACY[73]),
    FontUtf16(0x308A as u16, HIRAGANA_LEGACY[74]),
    FontUtf16(0x308B as u16, HIRAGANA_LEGACY[75]),
    FontUtf16(0x308C as u16, HIRAGANA_LEGACY[76]),
    FontUtf16(0x308D as u16, HIRAGANA_LEGACY[77]),
    FontUtf16(0x308E as u16, HIRAGANA_LEGACY[78]),
    FontUtf16(0x308F as u16, HIRAGANA_LEGACY[79]),
    FontUtf16(0x3090 as u16, HIRAGANA_LEGACY[80]),
    FontUtf16(0x3091 as u16, HIRAGANA_LEGACY[81]),
    FontUtf16(0x3092 as u16, HIRAGANA_LEGACY[82]),
    FontUtf16(0x3093 as u16, HIRAGANA_LEGACY[83]),
    FontUtf16(0x3094 as u16, HIRAGANA_LEGACY[84]),
    FontUtf16(0x3095 as u16, HIRAGANA_LEGACY[85]),
    FontUtf16(0x3096 as u16, HIRAGANA_LEGACY[86]),
    FontUtf16(0x3097 as u16, HIRAGANA_LEGACY[87]),
    FontUtf16(0x3098 as u16, HIRAGANA_LEGACY[88]),
    FontUtf16(0x3099 as u16, HIRAGANA_LEGACY[89]),
    FontUtf16(0x309A as u16, HIRAGANA_LEGACY[90]),
    FontUtf16(0x309B as u16, HIRAGANA_LEGACY[91]),
    FontUtf16(0x309C as u16, HIRAGANA_LEGACY[92]),
    FontUtf16(0x309D as u16, HIRAGANA_LEGACY[93]),
    FontUtf16(0x309E as u16, HIRAGANA_LEGACY[94]),
    FontUtf16(0x309F as u16, HIRAGANA_LEGACY[95]),
];

/// A convenient constant for Hiragana fonts (`U+3040` - `U+309F`), that implements the [`Utf16Fonts`](./utf16/trait.Utf16Fonts.html) trait.
pub const HIRAGANA_FONTS: HiraganaFonts = HiraganaFonts(HIRAGANA_UTF16);

/// Strong-typed collection wrapper for [HIRAGANA_UTF16](./constant.HIRAGANA_UTF16.html).
pub struct HiraganaFonts([FontUtf16; 96]);

impl HiraganaFonts {
    pub fn new() -> Self {
        HiraganaFonts(HIRAGANA_UTF16)
    }
}

impl fmt::Debug for HiraganaFonts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", stringify!(HIRAGANA_UTF16))
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

impl Utf16Fonts for HiraganaFonts {
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
    fn hiragana_fonts_constant_wraps_basic_utf16_constant() {
        let hiragana = HiraganaFonts::new();
        assert!(hiragana.0.len() == HIRAGANA_UTF16.len());
        for (idx, font) in hiragana.0.iter().enumerate() {
            assert_eq!(font, &HIRAGANA_UTF16[idx]);
        }
    }
}
