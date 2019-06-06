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

/// A convenient constant for Hiragana fonts (`U+3040` - `U+309F`), that implements the `UnicodeFonts` trait.
///
/// ## `HIRAGANA_UNICODE[1]`: `0x3041` `"ぁ"`
///
/// ```text
/// ░░█░░░░░
/// ██████░░
/// ░░█░░░░░
/// ░░████░░
/// ░██░█░█░
/// █░██░░█░
/// ░██░░█░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[2]`: `0x3042` `"あ"`
///
/// ```text
/// ░░█░░░░░
/// ██████░░
/// ░░█░░░░░
/// ░░████░░
/// ░██░█░█░
/// █░██░░█░
/// ░██░░█░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[3]`: `0x3043` `"ぃ"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// █░░░█░░░
/// █░░░░█░░
/// █░█░░█░░
/// ░█░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[4]`: `0x3044` `"い"`
///
/// ```text
/// ░░░░░░░░
/// █░░░░░░░
/// █░░░█░░░
/// █░░░░█░░
/// █░░░░█░░
/// █░█░░█░░
/// ░█░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[5]`: `0x3045` `"ぅ"`
///
/// ```text
/// ░░░░░░░░
/// ░░███░░░
/// ░░░░░░░░
/// ░░███░░░
/// ░█░░░█░░
/// ░░░░░█░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[6]`: `0x3046` `"う"`
///
/// ```text
/// ░░████░░
/// ░░░░░░░░
/// ░░████░░
/// ░█░░░░█░
/// ░░░░░░█░
/// ░░░░░█░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[7]`: `0x3047` `"ぇ"`
///
/// ```text
/// ░░███░░░
/// ░░░░░░░░
/// ░█████░░
/// ░░░░█░░░
/// ░░░███░░
/// ░░█░░█░░
/// ░█░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[8]`: `0x3048` `"え"`
///
/// ```text
/// ░░███░░░
/// ░░░░░░░░
/// ░█████░░
/// ░░░░█░░░
/// ░░░███░░
/// ░░█░░█░░
/// ░█░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[9]`: `0x3049` `"ぉ"`
///
/// ```text
/// ░░█░░█░░
/// ████░░█░
/// ░░█░░░░░
/// ░░████░░
/// ░██░░░█░
/// █░█░░░█░
/// ░█░░░█░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[10]`: `0x304A` `"お"`
///
/// ```text
/// ░░█░░█░░
/// ████░░█░
/// ░░█░░░░░
/// ░░████░░
/// ░██░░░█░
/// █░█░░░█░
/// ░█░░░█░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[11]`: `0x304B` `"か"`
///
/// ```text
/// ░░█░░░░░
/// ░░█░░█░░
/// ████░░█░
/// ░░█░█░█░
/// ░█░░█░█░
/// ░█░░█░░░
/// █░░█░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[12]`: `0x304C` `"が"`
///
/// ```text
/// ░░█░░░█░
/// ░░█░░█░░
/// ████░░░░
/// ░░█░█░█░
/// ░█░░█░█░
/// ░█░░█░█░
/// █░░█░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[13]`: `0x304D` `"き"`
///
/// ```text
/// ░░░█░░░░
/// █████░░░
/// ░░░█░░░░
/// ██████░░
/// ░░███░░░
/// ░█░░░░░░
/// ░░████░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[14]`: `0x304E` `"ぎ"`
///
/// ```text
/// ░░█░░░█░
/// ████░█░░
/// ░░█░░░░░
/// █████░░░
/// ░███░░░░
/// █░░░░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[15]`: `0x304F` `"く"`
///
/// ```text
/// ░░░░█░░░
/// ░░░█░░░░
/// ░░█░░░░░
/// ░█░░░░░░
/// ░░█░░░░░
/// ░░░█░░░░
/// ░░░░█░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[16]`: `0x3050` `"ぐ"`
///
/// ```text
/// ░░░█░█░░
/// ░░█░░░█░
/// ░█░░█░░░
/// █░░░░█░░
/// ░█░░░░░░
/// ░░█░░░░░
/// ░░░█░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[17]`: `0x3051` `"け"`
///
/// ```text
/// ░░░░░░░░
/// ░█░░░█░░
/// █░░████░
/// █░░░░█░░
/// █░░░░█░░
/// ░█░░░█░░
/// ░░░░█░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[18]`: `0x3052` `"げ"`
///
/// ```text
/// ░░░░░░█░
/// ░█░░░█░░
/// █░░░█░░░
/// █░████░░
/// █░░░█░░░
/// ░█░░█░░░
/// ░░░█░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[19]`: `0x3053` `"こ"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░████░░
/// ░░░░░░░░
/// ░█░░░░░░
/// ░█░░░░░░
/// ░░████░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[20]`: `0x3054` `"ご"`
///
/// ```text
/// ░░░░░█░░
/// ░░░░░░█░
/// ░██░█░░░
/// ░░░░░█░░
/// █░░░░░░░
/// █░░░░░░░
/// ░███░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[21]`: `0x3055` `"さ"`
///
/// ```text
/// ░░░░█░░░
/// ░██████░
/// ░░░░█░░░
/// ░░████░░
/// ░█░░░░░░
/// ░█░░░░░░
/// ░░███░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[22]`: `0x3056` `"ざ"`
///
/// ```text
/// ░░█░░█░░
/// ████░░█░
/// ░░█░█░░░
/// ░███░█░░
/// █░░░░░░░
/// █░░░░░░░
/// ░███░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[23]`: `0x3057` `"し"`
///
/// ```text
/// ░░░░░░░░
/// ░█░░░░░░
/// ░█░░░░░░
/// ░█░░░░░░
/// ░█░░░░█░
/// ░█░░░█░░
/// ░░███░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[24]`: `0x3058` `"じ"`
///
/// ```text
/// ░░░░░█░░
/// ░█░░░░█░
/// ░█░░█░░░
/// ░█░░░█░░
/// ░█░░░░░░
/// ░█░░░█░░
/// ░░███░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[25]`: `0x3059` `"す"`
///
/// ```text
/// ░░░░█░░░
/// ░██████░
/// ░░░██░░░
/// ░░█░█░░░
/// ░░░██░░░
/// ░░░░█░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[26]`: `0x305A` `"ず"`
///
/// ```text
/// ░░█░░░█░
/// ████░█░░
/// ░██░░░░░
/// █░█░░░░░
/// ░██░░░░░
/// ░░█░░░░░
/// ██░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[27]`: `0x305B` `"せ"`
///
/// ```text
/// ░░░░░█░░
/// ░█░░███░
/// ████░█░░
/// ░█░░░█░░
/// ░█░██░░░
/// ░█░░░░░░
/// ░░███░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[28]`: `0x305C` `"ぜ"`
///
/// ```text
/// ░░░░░░░█
/// ░░░░█░█░
/// ░█░███░░
/// ███░█░░░
/// ░█░██░░░
/// ░█░░░░░░
/// ░░███░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[29]`: `0x305D` `"そ"`
///
/// ```text
/// ░████░░░
/// ░░░█░░░░
/// ░░█░░░░░
/// ███████░
/// ░░░█░░░░
/// ░░█░░░░░
/// ░░░███░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[30]`: `0x305E` `"ぞ"`
///
/// ```text
/// ████░░█░
/// ░░█░░█░░
/// ░█░░░░░░
/// ███████░
/// ░░░█░░░░
/// ░░█░░░░░
/// ░░░███░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[31]`: `0x305F` `"た"`
///
/// ```text
/// ░█░░░░░░
/// ████░░░░
/// ░█░░░░░░
/// ░█░░███░
/// ░█░░░░░░
/// █░░█░░░░
/// █░░░███░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[32]`: `0x3060` `"だ"`
///
/// ```text
/// ░█░░░░█░
/// ████░█░░
/// ░█░░░░░░
/// ░█░░███░
/// ░█░░░░░░
/// █░░█░░░░
/// █░░░███░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[33]`: `0x3061` `"ち"`
///
/// ```text
/// ░░░█░░░░
/// ░██████░
/// ░░░█░░░░
/// ░░████░░
/// ░░░░░░█░
/// ░░░░░░█░
/// ░░░███░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[34]`: `0x3062` `"ぢ"`
///
/// ```text
/// ░░█░░░█░
/// ████░█░░
/// ░░█░░░░░
/// ░████░░░
/// ░░░░░█░░
/// ░░░░░█░░
/// ░░███░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[35]`: `0x3063` `"っ"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░███░░░
/// ░█░░░█░░
/// ░░░░░█░░
/// ░░███░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[36]`: `0x3064` `"つ"`
///
/// ```text
/// ░░░░░░░░
/// ░░███░░░
/// ░█░░░█░░
/// █░░░░░█░
/// ░░░░░░█░
/// ░░░░░█░░
/// ░░███░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[37]`: `0x3065` `"づ"`
///
/// ```text
/// ░░░░░░█░
/// ░░░░░█░░
/// ░████░░░
/// █░░░░█░░
/// ░░░░░█░░
/// ░░░░░█░░
/// ░░███░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[38]`: `0x3066` `"て"`
///
/// ```text
/// ░░░░░░░░
/// ░█████░░
/// ░░░█░░░░
/// ░░█░░░░░
/// ░░█░░░░░
/// ░░█░░░░░
/// ░░░███░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[39]`: `0x3067` `"で"`
///
/// ```text
/// ░░░░░░░░
/// ░█████░░
/// ░░░█░░█░
/// ░░█░░█░░
/// ░░█░░░░░
/// ░░█░░░░░
/// ░░░███░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[40]`: `0x3068` `"と"`
///
/// ```text
/// ░░█░░░░░
/// ░░█░░░░░
/// ░░░█░░░░
/// ░░████░░
/// ░█░░░░░░
/// ░█░░░░░░
/// ░░████░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[41]`: `0x3069` `"ど"`
///
/// ```text
/// ░░█░░░█░
/// ░░█░░█░░
/// ░░░█░░░░
/// ░░████░░
/// ░█░░░░░░
/// ░█░░░░░░
/// ░░████░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[42]`: `0x306A` `"な"`
///
/// ```text
/// ░█░░██░░
/// ░█░░░░░░
/// ███░░█░░
/// ░█░░░█░░
/// ░█░░███░
/// █░░█░█░░
/// █░░░█░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[43]`: `0x306B` `"に"`
///
/// ```text
/// ░░░░░░░░
/// ░█░░░░░░
/// ░█░████░
/// ░█░░░░░░
/// ░█░█░░░░
/// ░█░░███░
/// ░█░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[44]`: `0x306C` `"ぬ"`
///
/// ```text
/// ░░░█░░░░
/// █░░█░░░░
/// ░█████░░
/// ██░█░░█░
/// █░█░░██░
/// █░█░█░█░
/// ░█░░░█░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[45]`: `0x306D` `"ね"`
///
/// ```text
/// ░░█░░░░░
/// ███░░░░░
/// ░░█░██░░
/// ░░██░░█░
/// ░██░░██░
/// ░░█░█░█░
/// ░░█░░█░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[46]`: `0x306E` `"の"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░████░░
/// ░█░█░░█░
/// █░░█░░█░
/// █░█░░░█░
/// ░█░░░█░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[47]`: `0x306F` `"は"`
///
/// ```text
/// ░░░░░░░░
/// ░█░░░█░░
/// ░█░████░
/// ░█░░░█░░
/// ░█░░███░
/// ░█░█░█░░
/// ░█░░█░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[48]`: `0x3070` `"ば"`
///
/// ```text
/// ░░░░░░░█
/// █░░░█░█░
/// █░███░░░
/// █░░░█░░░
/// █░░███░░
/// █░█░█░░░
/// █░░█░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[49]`: `0x3071` `"ぱ"`
///
/// ```text
/// ░░░░░░█░
/// █░░░██░█
/// █░███░█░
/// █░░░█░░░
/// █░░███░░
/// █░█░█░░░
/// █░░█░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[50]`: `0x3072` `"ひ"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░░█░░░
/// ░█░░██░░
/// █░░░█░█░
/// █░░░█░░░
/// ░███░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[51]`: `0x3073` `"び"`
///
/// ```text
/// ░░░░░░█░
/// ░░░░░█░░
/// ██░░░░░░
/// ░█░░██░░
/// █░░░█░█░
/// █░░░█░░░
/// ░███░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[52]`: `0x3074` `"ぴ"`
///
/// ```text
/// ░░░░░░█░
/// ░░░░░█░█
/// ██░░░░█░
/// ░█░░██░░
/// █░░░█░█░
/// █░░░█░░░
/// ░███░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[53]`: `0x3075` `"ふ"`
///
/// ```text
/// ░░███░░░
/// ░░░░░░░░
/// ░░░█░░░░
/// ░█░█░█░░
/// █░░█░░█░
/// ░░░░█░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[54]`: `0x3076` `"ぶ"`
///
/// ```text
/// ░░██░░█░
/// ░░░░░█░░
/// ░░░█░░░░
/// ░█░█░█░░
/// █░░█░░█░
/// ░░░░█░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[55]`: `0x3077` `"ぷ"`
///
/// ```text
/// ░░██░░█░
/// ░░░░░█░█
/// ░░░█░░█░
/// ░█░█░░░░
/// █░░█░█░░
/// ░░░█░░█░
/// ░░██░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[56]`: `0x3078` `"へ"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░█░░░░░
/// ░█░█░░░░
/// █░░░█░░░
/// ░░░░░█░░
/// ░░░░░░█░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[57]`: `0x3079` `"べ"`
///
/// ```text
/// ░░░░░█░░
/// ░░░░░░█░
/// ░░█░█░░░
/// ░█░█░█░░
/// █░░░█░░░
/// ░░░░░█░░
/// ░░░░░░█░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[58]`: `0x307A` `"ぺ"`
///
/// ```text
/// ░░░░░█░░
/// ░░░░█░█░
/// ░░█░░█░░
/// ░█░█░░░░
/// █░░░█░░░
/// ░░░░░█░░
/// ░░░░░░█░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[59]`: `0x307B` `"ほ"`
///
/// ```text
/// █░█████░
/// █░░░█░░░
/// █░█████░
/// █░░░█░░░
/// █░░███░░
/// █░█░█░█░
/// █░░█░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[60]`: `0x307C` `"ぼ"`
///
/// ```text
/// █░███░░█
/// █░░░█░█░
/// █░███░░░
/// █░░░█░░░
/// █░░███░░
/// █░█░█░█░
/// █░░█░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[61]`: `0x307D` `"ぽ"`
///
/// ```text
/// █░███░█░
/// █░░░██░█
/// █░███░█░
/// █░░░█░░░
/// █░░███░░
/// █░█░█░█░
/// █░░█░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[62]`: `0x307E` `"ま"`
///
/// ```text
/// ░██████░
/// ░░░█░░░░
/// ░█████░░
/// ░░░█░░░░
/// ░░███░░░
/// ░█░█░█░░
/// ░░█░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[63]`: `0x307F` `"み"`
///
/// ```text
/// ░░░░░░░░
/// ███░░░░░
/// ░░█░░█░░
/// ░░█░░█░░
/// ░██████░
/// █░█░░█░░
/// ░█░░█░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[64]`: `0x3080` `"む"`
///
/// ```text
/// ░░█░░░░░
/// ████░░░░
/// ░░█░░██░
/// ░██░░░░░
/// █░█░░░░░
/// ░██░░█░░
/// ░░████░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[65]`: `0x3081` `"め"`
///
/// ```text
/// ░░░░░░░░
/// █░░█░░░░
/// █░████░░
/// ░█░█░░█░
/// ██░█░░█░
/// █░█░░░█░
/// ░█░█░█░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[66]`: `0x3082` `"も"`
///
/// ```text
/// ░█░░░░░░
/// ████░░░░
/// ░█░░░░░░
/// ████░░░░
/// ░█░░░██░
/// ░█░░░░█░
/// ░░████░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[67]`: `0x3083` `"ゃ"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░█░░█░░░
/// █████░░░
/// ░█░░░█░░
/// ░█░░█░░░
/// ░░█░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[68]`: `0x3084` `"や"`
///
/// ```text
/// ░░░░░░░░
/// ░█░░█░░░
/// ██████░░
/// ░█░░░░█░
/// ░█░░░░█░
/// ░░█░██░░
/// ░░█░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[69]`: `0x3085` `"ゅ"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// █░░░█░░░
/// █░████░░
/// ██░░█░█░
/// █░░███░░
/// █░░░█░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[70]`: `0x3086` `"ゆ"`
///
/// ```text
/// ░░░░░░░░
/// █░░░█░░░
/// █░████░░
/// ██░░█░█░
/// █░░░█░█░
/// █░░███░░
/// █░░░█░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[71]`: `0x3087` `"ょ"`
///
/// ```text
/// ░░░░░░░░
/// ░░░█░░░░
/// ░░░███░░
/// ░░░█░░░░
/// ░░███░░░
/// ░█░█░█░░
/// ░░█░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[72]`: `0x3088` `"よ"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░███░░
/// ░░░█░░░░
/// ░░███░░░
/// ░█░█░█░░
/// ░░█░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[73]`: `0x3089` `"ら"`
///
/// ```text
/// ░████░░░
/// ░░░░░░░░
/// ░█░░░░░░
/// ░█░███░░
/// ░██░░░█░
/// ░█░░░░█░
/// ░░░░██░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[74]`: `0x308A` `"り"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░█░░
/// ░█░░░█░░
/// ░█░░░█░░
/// ░█░█░█░░
/// ░░█░░█░░
/// ░░░░█░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[75]`: `0x308B` `"る"`
///
/// ```text
/// █████░░░
/// ░░░█░░░░
/// ░░████░░
/// ░█░░░░█░
/// █░░█░░█░
/// ░░█░█░█░
/// ░░░███░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[76]`: `0x308C` `"れ"`
///
/// ```text
/// ░░█░░░░░
/// ███░░░░░
/// ░░█░░░░░
/// ░░██░░░░
/// ░██░█░░░
/// █░█░█░█░
/// ░░█░░█░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[77]`: `0x308D` `"ろ"`
///
/// ```text
/// ██████░░
/// ░░░░█░░░
/// ░░░█░░░░
/// ░░████░░
/// ░█░░░░█░
/// █░░░░░█░
/// ░░░░██░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[78]`: `0x308E` `"ゎ"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░█░░░░
/// ░███░░░░
/// ░░░███░░
/// ░░██░░█░
/// ░█░█░█░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[79]`: `0x308F` `"わ"`
///
/// ```text
/// ░░█░░░░░
/// ███░░░░░
/// ░░█░░░░░
/// ░░████░░
/// ░██░░░█░
/// █░█░░░█░
/// ░░█░░█░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[80]`: `0x3090` `"ゐ"`
///
/// ```text
/// ░███░░░░
/// ░░░█░░░░
/// ░░████░░
/// ░█░█░░█░
/// █░░█░██░
/// █░█░█░█░
/// ░█░░██░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[81]`: `0x3091` `"ゑ"`
///
/// ```text
/// ░██░░░░░
/// ░░████░░
/// ░█░░░░█░
/// █░░███░░
/// ░░█░░░░░
/// ░██░██░░
/// █░░█░░█░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[82]`: `0x3092` `"を"`
///
/// ```text
/// ░░█░░░░░
/// ████░░░░
/// ░░█░░░░░
/// ░███░██░
/// █░░░█░░░
/// ░░░█░░░░
/// ░░░░███░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[83]`: `0x3093` `"ん"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░█░░░░░
/// ░░██░░░░
/// ░██░█░█░
/// ░█░░█░█░
/// █░░░░█░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[84]`: `0x3094` `"ゔ"`
///
/// ```text
/// ░░░░░░█░
/// ░███░█░░
/// ░░░░░░░░
/// ░░████░░
/// ░█░░░░█░
/// ░░░░░░█░
/// ░░░███░░
/// ░░░░░░░░
/// ```
///
/// ##  85: 0x3095 " "
/// ##  86: 0x3096 " "
/// ##  87: 0x3097 " "
/// ##  88: 0x3098 " "
/// ##  89: 0x3099 " "
/// ##  90: 0x309A " "
/// ## `HIRAGANA_UNICODE[91]`: `0x309B` `"゛"`
///
/// ```text
/// ░░░░░░█░
/// ░░░░░░░█
/// ░░░░░█░░
/// ░░░░░░█░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[92]`: `0x309C` `"゜"`
///
/// ```text
/// ░░░░░░█░
/// ░░░░░█░█
/// ░░░░░░█░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[93]`: `0x309D` `"ゝ"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░░█░░░
/// ░░░░██░░
/// ░░██░░░░
/// ░░░░░░░░
/// ```
///
/// ## `HIRAGANA_UNICODE[94]`: `0x309E` `"ゞ"`
///
/// ```text
/// ░░░░░█░░
/// ░░░░░░█░
/// ░░█░█░░░
/// ░░█░░█░░
/// ░░░█░░░░
/// ░░░██░░░
/// ░██░░░░░
/// ░░░░░░░░
/// ```
///
/// ##  95: 0x309F " "
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
        write!(f, stringify!(HIRAGANA_UNICODE))
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
        self.0.into_iter()
            .fold(Vec::with_capacity(128), |mut v, font| {
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
