//! Hiragana. `U+3040 - U+309F`
use super::{legacy::HIRAGANA_LEGACY, FontUtf16, Utf16Fonts};
use std::fmt;

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

#[cfg(feature = "unicode")]
pub const HIRAGANA_UNICODE: [(u16, [u8; 8]); 96] = [
    (0x3040, HIRAGANA[0]),
    (0x3041, HIRAGANA[1]),
    (0x3042, HIRAGANA[2]),
    (0x3043, HIRAGANA[3]),
    (0x3044, HIRAGANA[4]),
    (0x3045, HIRAGANA[5]),
    (0x3046, HIRAGANA[6]),
    (0x3047, HIRAGANA[7]),
    (0x3048, HIRAGANA[8]),
    (0x3049, HIRAGANA[9]),
    (0x304A, HIRAGANA[10]),
    (0x304B, HIRAGANA[11]),
    (0x304C, HIRAGANA[12]),
    (0x304D, HIRAGANA[13]),
    (0x304E, HIRAGANA[14]),
    (0x304F, HIRAGANA[15]),
    (0x3050, HIRAGANA[16]),
    (0x3051, HIRAGANA[17]),
    (0x3052, HIRAGANA[18]),
    (0x3053, HIRAGANA[19]),
    (0x3054, HIRAGANA[20]),
    (0x3055, HIRAGANA[21]),
    (0x3056, HIRAGANA[22]),
    (0x3057, HIRAGANA[23]),
    (0x3058, HIRAGANA[24]),
    (0x3059, HIRAGANA[25]),
    (0x305A, HIRAGANA[26]),
    (0x305B, HIRAGANA[27]),
    (0x305C, HIRAGANA[28]),
    (0x305D, HIRAGANA[29]),
    (0x305E, HIRAGANA[30]),
    (0x305F, HIRAGANA[31]),
    (0x3060, HIRAGANA[32]),
    (0x3061, HIRAGANA[33]),
    (0x3062, HIRAGANA[34]),
    (0x3063, HIRAGANA[35]),
    (0x3064, HIRAGANA[36]),
    (0x3065, HIRAGANA[37]),
    (0x3066, HIRAGANA[38]),
    (0x3067, HIRAGANA[39]),
    (0x3068, HIRAGANA[40]),
    (0x3069, HIRAGANA[41]),
    (0x306A, HIRAGANA[42]),
    (0x306B, HIRAGANA[43]),
    (0x306C, HIRAGANA[44]),
    (0x306D, HIRAGANA[45]),
    (0x306E, HIRAGANA[46]),
    (0x306F, HIRAGANA[47]),
    (0x3070, HIRAGANA[48]),
    (0x3071, HIRAGANA[49]),
    (0x3072, HIRAGANA[50]),
    (0x3073, HIRAGANA[51]),
    (0x3074, HIRAGANA[52]),
    (0x3075, HIRAGANA[53]),
    (0x3076, HIRAGANA[54]),
    (0x3077, HIRAGANA[55]),
    (0x3078, HIRAGANA[56]),
    (0x3079, HIRAGANA[57]),
    (0x307A, HIRAGANA[58]),
    (0x307B, HIRAGANA[59]),
    (0x307C, HIRAGANA[60]),
    (0x307D, HIRAGANA[61]),
    (0x307E, HIRAGANA[62]),
    (0x307F, HIRAGANA[63]),
    (0x3080, HIRAGANA[64]),
    (0x3081, HIRAGANA[65]),
    (0x3082, HIRAGANA[66]),
    (0x3083, HIRAGANA[67]),
    (0x3084, HIRAGANA[68]),
    (0x3085, HIRAGANA[69]),
    (0x3086, HIRAGANA[70]),
    (0x3087, HIRAGANA[71]),
    (0x3088, HIRAGANA[72]),
    (0x3089, HIRAGANA[73]),
    (0x308A, HIRAGANA[74]),
    (0x308B, HIRAGANA[75]),
    (0x308C, HIRAGANA[76]),
    (0x308D, HIRAGANA[77]),
    (0x308E, HIRAGANA[78]),
    (0x308F, HIRAGANA[79]),
    (0x3090, HIRAGANA[80]),
    (0x3091, HIRAGANA[81]),
    (0x3092, HIRAGANA[82]),
    (0x3093, HIRAGANA[83]),
    (0x3094, HIRAGANA[84]),
    (0x3095, HIRAGANA[85]),
    (0x3096, HIRAGANA[86]),
    (0x3097, HIRAGANA[87]),
    (0x3098, HIRAGANA[88]),
    (0x3099, HIRAGANA[89]),
    (0x309A, HIRAGANA[90]),
    (0x309B, HIRAGANA[91]),
    (0x309C, HIRAGANA[92]),
    (0x309D, HIRAGANA[93]),
    (0x309E, HIRAGANA[94]),
    (0x309F, HIRAGANA[95]),
];

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
