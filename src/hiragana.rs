//! Hiragana. `U+3040 - U+309F`
/// Contains an 8x8 font map for unicode points `U+3040 - U+309F` (Hiragana)
///
/// * `U+3040`
/// * `U+3041` (Hiragana a)
/// * `U+3042` (Hiragana A)
/// * `U+3043` (Hiragana i)
/// * `U+3044` (Hiragana I)
/// * `U+3045` (Hiragana u)
/// * `U+3046` (Hiragana U)
/// * `U+3047` (Hiragana e)
/// * `U+3048` (Hiragana E)
/// * `U+3049` (Hiragana o)
/// * `U+304A` (Hiragana O)
/// * `U+304B` (Hiragana KA)
/// * `U+304C` (Hiragana GA)
/// * `U+304D` (Hiragana KI)
/// * `U+304E` (Hiragana GI)
/// * `U+304F` (Hiragana KU)
/// * `U+3050` (Hiragana GU)
/// * `U+3051` (Hiragana KE)
/// * `U+3052` (Hiragana GE)
/// * `U+3053` (Hiragana KO)
/// * `U+3054` (Hiragana GO)
/// * `U+3055` (Hiragana SA)
/// * `U+3056` (Hiragana ZA)
/// * `U+3057` (Hiragana SI)
/// * `U+3058` (Hiragana ZI)
/// * `U+3059` (Hiragana SU)
/// * `U+305A` (Hiragana ZU)
/// * `U+305B` (Hiragana SE)
/// * `U+305C` (Hiragana ZE)
/// * `U+305D` (Hiragana SO)
/// * `U+305E` (Hiragana ZO)
/// * `U+305F` (Hiragana TA)
/// * `U+3060` (Hiragana DA)
/// * `U+3061` (Hiragana TI)
/// * `U+3062` (Hiragana DI)
/// * `U+3063` (Hiragana tu)
/// * `U+3064` (Hiragana TU)
/// * `U+3065` (Hiragana DU)
/// * `U+3066` (Hiragana TE)
/// * `U+3067` (Hiragana DE)
/// * `U+3068` (Hiragana TO)
/// * `U+3069` (Hiragana DO)
/// * `U+306A` (Hiragana NA)
/// * `U+306B` (Hiragana NI)
/// * `U+306C` (Hiragana NU)
/// * `U+306D` (Hiragana NE)
/// * `U+306E` (Hiragana NO)
/// * `U+306F` (Hiragana HA)
/// * `U+3070` (Hiragana BA)
/// * `U+3071` (Hiragana PA)
/// * `U+3072` (Hiragana HI)
/// * `U+3073` (Hiragana BI)
/// * `U+3074` (Hiragana PI)
/// * `U+3075` (Hiragana HU)
/// * `U+3076` (Hiragana BU)
/// * `U+3077` (Hiragana PU)
/// * `U+3078` (Hiragana HE)
/// * `U+3079` (Hiragana BE)
/// * `U+307A` (Hiragana PE)
/// * `U+307B` (Hiragana HO)
/// * `U+307C` (Hiragana BO)
/// * `U+307D` (Hiragana PO)
/// * `U+307E` (Hiragana MA)
/// * `U+307F` (Hiragana MI)
/// * `U+3080` (Hiragana MU)
/// * `U+3081` (Hiragana ME)
/// * `U+3082` (Hiragana MO)
/// * `U+3083` (Hiragana ya)
/// * `U+3084` (Hiragana YA)
/// * `U+3085` (Hiragana yu)
/// * `U+3086` (Hiragana YU)
/// * `U+3087` (Hiragana yo)
/// * `U+3088` (Hiragana YO)
/// * `U+3089` (Hiragana RA)
/// * `U+308A` (Hiragana RI)
/// * `U+308B` (Hiragana RU)
/// * `U+308C` (Hiragana RE)
/// * `U+308D` (Hiragana RO)
/// * `U+308E` (Hiragana wa)
/// * `U+308F` (Hiragana WA)
/// * `U+3090` (Hiragana WI)
/// * `U+3091` (Hiragana WE)
/// * `U+3092` (Hiragana WO)
/// * `U+3093` (Hiragana N)
/// * `U+3094` (Hiragana VU)
/// * `U+3095`
/// * `U+3096`
/// * `U+3097`
/// * `U+3098`
/// * `U+3099` (voiced combinator mark)
/// * `U+309A` (semivoiced combinator mark)
/// * `U+309B` (Hiragana voiced mark)
/// * `U+309C` (Hiragana semivoiced mark)
/// * `U+309D` (Hiragana iteration mark)
/// * `U+309E` (Hiragana voiced iteration mark)
/// * `U+309F`
pub const HIRAGANA: [[u8; 8]; 96] = [
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x04, 0x3F, 0x04, 0x3C, 0x56, 0x4D, 0x26, 0x00],
    [0x04, 0x3F, 0x04, 0x3C, 0x56, 0x4D, 0x26, 0x00],
    [0x00, 0x00, 0x00, 0x11, 0x21, 0x25, 0x02, 0x00],
    [0x00, 0x01, 0x11, 0x21, 0x21, 0x25, 0x02, 0x00],
    [0x00, 0x1C, 0x00, 0x1C, 0x22, 0x20, 0x18, 0x00],
    [0x3C, 0x00, 0x3C, 0x42, 0x40, 0x20, 0x18, 0x00],
    [0x1C, 0x00, 0x3E, 0x10, 0x38, 0x24, 0x62, 0x00],
    [0x1C, 0x00, 0x3E, 0x10, 0x38, 0x24, 0x62, 0x00],
    [0x24, 0x4F, 0x04, 0x3C, 0x46, 0x45, 0x22, 0x00],
    [0x24, 0x4F, 0x04, 0x3C, 0x46, 0x45, 0x22, 0x00],
    [0x04, 0x24, 0x4F, 0x54, 0x52, 0x12, 0x09, 0x00],
    [0x44, 0x24, 0x0F, 0x54, 0x52, 0x52, 0x09, 0x00],
    [0x08, 0x1F, 0x08, 0x3F, 0x1C, 0x02, 0x3C, 0x00],
    [0x44, 0x2F, 0x04, 0x1F, 0x0E, 0x01, 0x1E, 0x00],
    [0x10, 0x08, 0x04, 0x02, 0x04, 0x08, 0x10, 0x00],
    [0x28, 0x44, 0x12, 0x21, 0x02, 0x04, 0x08, 0x00],
    [0x00, 0x22, 0x79, 0x21, 0x21, 0x22, 0x10, 0x00],
    [0x40, 0x22, 0x11, 0x3D, 0x11, 0x12, 0x08, 0x00],
    [0x00, 0x00, 0x3C, 0x00, 0x02, 0x02, 0x3C, 0x00],
    [0x20, 0x40, 0x16, 0x20, 0x01, 0x01, 0x0E, 0x00],
    [0x10, 0x7E, 0x10, 0x3C, 0x02, 0x02, 0x1C, 0x00],
    [0x24, 0x4F, 0x14, 0x2E, 0x01, 0x01, 0x0E, 0x00],
    [0x00, 0x02, 0x02, 0x02, 0x42, 0x22, 0x1C, 0x00],
    [0x20, 0x42, 0x12, 0x22, 0x02, 0x22, 0x1C, 0x00],
    [0x10, 0x7E, 0x18, 0x14, 0x18, 0x10, 0x0C, 0x00],
    [0x44, 0x2F, 0x06, 0x05, 0x06, 0x04, 0x03, 0x00],
    [0x20, 0x72, 0x2F, 0x22, 0x1A, 0x02, 0x1C, 0x00],
    [0x80, 0x50, 0x3A, 0x17, 0x1A, 0x02, 0x1C, 0x00],
    [0x1E, 0x08, 0x04, 0x7F, 0x08, 0x04, 0x38, 0x00],
    [0x4F, 0x24, 0x02, 0x7F, 0x08, 0x04, 0x38, 0x00],
    [0x02, 0x0F, 0x02, 0x72, 0x02, 0x09, 0x71, 0x00],
    [0x42, 0x2F, 0x02, 0x72, 0x02, 0x09, 0x71, 0x00],
    [0x08, 0x7E, 0x08, 0x3C, 0x40, 0x40, 0x38, 0x00],
    [0x44, 0x2F, 0x04, 0x1E, 0x20, 0x20, 0x1C, 0x00],
    [0x00, 0x00, 0x00, 0x1C, 0x22, 0x20, 0x1C, 0x00],
    [0x00, 0x1C, 0x22, 0x41, 0x40, 0x20, 0x1C, 0x00],
    [0x40, 0x20, 0x1E, 0x21, 0x20, 0x20, 0x1C, 0x00],
    [0x00, 0x3E, 0x08, 0x04, 0x04, 0x04, 0x38, 0x00],
    [0x00, 0x3E, 0x48, 0x24, 0x04, 0x04, 0x38, 0x00],
    [0x04, 0x04, 0x08, 0x3C, 0x02, 0x02, 0x3C, 0x00],
    [0x44, 0x24, 0x08, 0x3C, 0x02, 0x02, 0x3C, 0x00],
    [0x32, 0x02, 0x27, 0x22, 0x72, 0x29, 0x11, 0x00],
    [0x00, 0x02, 0x7A, 0x02, 0x0A, 0x72, 0x02, 0x00],
    [0x08, 0x09, 0x3E, 0x4B, 0x65, 0x55, 0x22, 0x00],
    [0x04, 0x07, 0x34, 0x4C, 0x66, 0x54, 0x24, 0x00],
    [0x00, 0x00, 0x3C, 0x4A, 0x49, 0x45, 0x22, 0x00],
    [0x00, 0x22, 0x7A, 0x22, 0x72, 0x2A, 0x12, 0x00],
    [0x80, 0x51, 0x1D, 0x11, 0x39, 0x15, 0x09, 0x00],
    [0x40, 0xB1, 0x5D, 0x11, 0x39, 0x15, 0x09, 0x00],
    [0x00, 0x00, 0x13, 0x32, 0x51, 0x11, 0x0E, 0x00],
    [0x40, 0x20, 0x03, 0x32, 0x51, 0x11, 0x0E, 0x00],
    [0x40, 0xA0, 0x43, 0x32, 0x51, 0x11, 0x0E, 0x00],
    [0x1C, 0x00, 0x08, 0x2A, 0x49, 0x10, 0x0C, 0x00],
    [0x4C, 0x20, 0x08, 0x2A, 0x49, 0x10, 0x0C, 0x00],
    [0x4C, 0xA0, 0x48, 0x0A, 0x29, 0x48, 0x0C, 0x00],
    [0x00, 0x00, 0x04, 0x0A, 0x11, 0x20, 0x40, 0x00],
    [0x20, 0x40, 0x14, 0x2A, 0x11, 0x20, 0x40, 0x00],
    [0x20, 0x50, 0x24, 0x0A, 0x11, 0x20, 0x40, 0x00],
    [0x7D, 0x11, 0x7D, 0x11, 0x39, 0x55, 0x09, 0x00],
    [0x9D, 0x51, 0x1D, 0x11, 0x39, 0x55, 0x09, 0x00],
    [0x5D, 0xB1, 0x5D, 0x11, 0x39, 0x55, 0x09, 0x00],
    [0x7E, 0x08, 0x3E, 0x08, 0x1C, 0x2A, 0x04, 0x00],
    [0x00, 0x07, 0x24, 0x24, 0x7E, 0x25, 0x12, 0x00],
    [0x04, 0x0F, 0x64, 0x06, 0x05, 0x26, 0x3C, 0x00],
    [0x00, 0x09, 0x3D, 0x4A, 0x4B, 0x45, 0x2A, 0x00],
    [0x02, 0x0F, 0x02, 0x0F, 0x62, 0x42, 0x3C, 0x00],
    [0x00, 0x00, 0x12, 0x1F, 0x22, 0x12, 0x04, 0x00],
    [0x00, 0x12, 0x3F, 0x42, 0x42, 0x34, 0x04, 0x00],
    [0x00, 0x00, 0x11, 0x3D, 0x53, 0x39, 0x11, 0x00],
    [0x00, 0x11, 0x3D, 0x53, 0x51, 0x39, 0x11, 0x00],
    [0x00, 0x08, 0x38, 0x08, 0x1C, 0x2A, 0x04, 0x00],
    [0x08, 0x08, 0x38, 0x08, 0x1C, 0x2A, 0x04, 0x00],
    [0x1E, 0x00, 0x02, 0x3A, 0x46, 0x42, 0x30, 0x00],
    [0x00, 0x20, 0x22, 0x22, 0x2A, 0x24, 0x10, 0x00],
    [0x1F, 0x08, 0x3C, 0x42, 0x49, 0x54, 0x38, 0x00],
    [0x04, 0x07, 0x04, 0x0C, 0x16, 0x55, 0x24, 0x00],
    [0x3F, 0x10, 0x08, 0x3C, 0x42, 0x41, 0x30, 0x00],
    [0x00, 0x00, 0x08, 0x0E, 0x38, 0x4C, 0x2A, 0x00],
    [0x04, 0x07, 0x04, 0x3C, 0x46, 0x45, 0x24, 0x00],
    [0x0E, 0x08, 0x3C, 0x4A, 0x69, 0x55, 0x32, 0x00],
    [0x06, 0x3C, 0x42, 0x39, 0x04, 0x36, 0x49, 0x00],
    [0x04, 0x0F, 0x04, 0x6E, 0x11, 0x08, 0x70, 0x00],
    [0x08, 0x08, 0x04, 0x0C, 0x56, 0x52, 0x21, 0x00],
    [0x40, 0x2E, 0x00, 0x3C, 0x42, 0x40, 0x38, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x40, 0x80, 0x20, 0x40, 0x00, 0x00, 0x00, 0x00],
    [0x40, 0xA0, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x08, 0x08, 0x10, 0x30, 0x0C, 0x00],
    [0x20, 0x40, 0x14, 0x24, 0x08, 0x18, 0x06, 0x00],
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
];