//! Extended Latin. `U+00A0 - U+00FF`
use super::{
    legacy::LATIN_LEGACY,
    unicode::{FontUnicode, UnicodeFonts},
};
use core::fmt;

/// A constant `[FontUnicode; 96]`, for Extended Latin fonts (`U+00A0` - `U+00FF`).
pub const LATIN_UNICODE: [FontUnicode; 96] = [
    FontUnicode('\u{00A0}', LATIN_LEGACY[0]),
    FontUnicode('\u{00A1}', LATIN_LEGACY[1]),
    FontUnicode('\u{00A2}', LATIN_LEGACY[2]),
    FontUnicode('\u{00A3}', LATIN_LEGACY[3]),
    FontUnicode('\u{00A4}', LATIN_LEGACY[4]),
    FontUnicode('\u{00A5}', LATIN_LEGACY[5]),
    FontUnicode('\u{00A6}', LATIN_LEGACY[6]),
    FontUnicode('\u{00A7}', LATIN_LEGACY[7]),
    FontUnicode('\u{00A8}', LATIN_LEGACY[8]),
    FontUnicode('\u{00A9}', LATIN_LEGACY[9]),
    FontUnicode('\u{00AA}', LATIN_LEGACY[10]),
    FontUnicode('\u{00AB}', LATIN_LEGACY[11]),
    FontUnicode('\u{00AC}', LATIN_LEGACY[12]),
    FontUnicode('\u{00AD}', LATIN_LEGACY[13]),
    FontUnicode('\u{00AE}', LATIN_LEGACY[14]),
    FontUnicode('\u{00AF}', LATIN_LEGACY[15]),
    FontUnicode('\u{00B0}', LATIN_LEGACY[16]),
    FontUnicode('\u{00B1}', LATIN_LEGACY[17]),
    FontUnicode('\u{00B2}', LATIN_LEGACY[18]),
    FontUnicode('\u{00B2}', LATIN_LEGACY[19]),
    FontUnicode('\u{00B2}', LATIN_LEGACY[20]),
    FontUnicode('\u{00B5}', LATIN_LEGACY[21]),
    FontUnicode('\u{00B6}', LATIN_LEGACY[22]),
    FontUnicode('\u{00B7}', LATIN_LEGACY[23]),
    FontUnicode('\u{00B8}', LATIN_LEGACY[24]),
    FontUnicode('\u{00B9}', LATIN_LEGACY[25]),
    FontUnicode('\u{00BA}', LATIN_LEGACY[26]),
    FontUnicode('\u{00BB}', LATIN_LEGACY[27]),
    FontUnicode('\u{00BC}', LATIN_LEGACY[28]),
    FontUnicode('\u{00BD}', LATIN_LEGACY[29]),
    FontUnicode('\u{00BE}', LATIN_LEGACY[30]),
    FontUnicode('\u{00BF}', LATIN_LEGACY[31]),
    FontUnicode('\u{00C0}', LATIN_LEGACY[32]),
    FontUnicode('\u{00C1}', LATIN_LEGACY[33]),
    FontUnicode('\u{00C2}', LATIN_LEGACY[34]),
    FontUnicode('\u{00C3}', LATIN_LEGACY[35]),
    FontUnicode('\u{00C4}', LATIN_LEGACY[36]),
    FontUnicode('\u{00C5}', LATIN_LEGACY[37]),
    FontUnicode('\u{00C6}', LATIN_LEGACY[38]),
    FontUnicode('\u{00C7}', LATIN_LEGACY[39]),
    FontUnicode('\u{00C8}', LATIN_LEGACY[40]),
    FontUnicode('\u{00C9}', LATIN_LEGACY[41]),
    FontUnicode('\u{00CA}', LATIN_LEGACY[42]),
    FontUnicode('\u{00CB}', LATIN_LEGACY[43]),
    FontUnicode('\u{00CC}', LATIN_LEGACY[44]),
    FontUnicode('\u{00CD}', LATIN_LEGACY[45]),
    FontUnicode('\u{00CE}', LATIN_LEGACY[46]),
    FontUnicode('\u{00CF}', LATIN_LEGACY[47]),
    FontUnicode('\u{00D0}', LATIN_LEGACY[48]),
    FontUnicode('\u{00D1}', LATIN_LEGACY[49]),
    FontUnicode('\u{00D2}', LATIN_LEGACY[50]),
    FontUnicode('\u{00D3}', LATIN_LEGACY[51]),
    FontUnicode('\u{00D4}', LATIN_LEGACY[52]),
    FontUnicode('\u{00D5}', LATIN_LEGACY[53]),
    FontUnicode('\u{00D6}', LATIN_LEGACY[54]),
    FontUnicode('\u{00D7}', LATIN_LEGACY[55]),
    FontUnicode('\u{00D8}', LATIN_LEGACY[56]),
    FontUnicode('\u{00D9}', LATIN_LEGACY[57]),
    FontUnicode('\u{00DA}', LATIN_LEGACY[58]),
    FontUnicode('\u{00DB}', LATIN_LEGACY[59]),
    FontUnicode('\u{00DC}', LATIN_LEGACY[60]),
    FontUnicode('\u{00DD}', LATIN_LEGACY[61]),
    FontUnicode('\u{00DE}', LATIN_LEGACY[62]),
    FontUnicode('\u{00DF}', LATIN_LEGACY[63]),
    FontUnicode('\u{00E0}', LATIN_LEGACY[64]),
    FontUnicode('\u{00E1}', LATIN_LEGACY[65]),
    FontUnicode('\u{00E2}', LATIN_LEGACY[66]),
    FontUnicode('\u{00E3}', LATIN_LEGACY[67]),
    FontUnicode('\u{00E4}', LATIN_LEGACY[68]),
    FontUnicode('\u{00E5}', LATIN_LEGACY[69]),
    FontUnicode('\u{00E6}', LATIN_LEGACY[70]),
    FontUnicode('\u{00E7}', LATIN_LEGACY[71]),
    FontUnicode('\u{00E8}', LATIN_LEGACY[72]),
    FontUnicode('\u{00E9}', LATIN_LEGACY[73]),
    FontUnicode('\u{00EA}', LATIN_LEGACY[74]),
    FontUnicode('\u{00EB}', LATIN_LEGACY[75]),
    FontUnicode('\u{00EC}', LATIN_LEGACY[76]),
    FontUnicode('\u{00ED}', LATIN_LEGACY[77]),
    FontUnicode('\u{00EE}', LATIN_LEGACY[78]),
    FontUnicode('\u{00EF}', LATIN_LEGACY[79]),
    FontUnicode('\u{00F0}', LATIN_LEGACY[80]),
    FontUnicode('\u{00F1}', LATIN_LEGACY[81]),
    FontUnicode('\u{00F2}', LATIN_LEGACY[82]),
    FontUnicode('\u{00F3}', LATIN_LEGACY[83]),
    FontUnicode('\u{00F4}', LATIN_LEGACY[84]),
    FontUnicode('\u{00F5}', LATIN_LEGACY[85]),
    FontUnicode('\u{00F6}', LATIN_LEGACY[86]),
    FontUnicode('\u{00F7}', LATIN_LEGACY[87]),
    FontUnicode('\u{00F8}', LATIN_LEGACY[88]),
    FontUnicode('\u{00F9}', LATIN_LEGACY[89]),
    FontUnicode('\u{00FA}', LATIN_LEGACY[90]),
    FontUnicode('\u{00FB}', LATIN_LEGACY[91]),
    FontUnicode('\u{00FC}', LATIN_LEGACY[92]),
    FontUnicode('\u{00FD}', LATIN_LEGACY[93]),
    FontUnicode('\u{00FE}', LATIN_LEGACY[94]),
    FontUnicode('\u{00FF}', LATIN_LEGACY[95]),
];

/// A convenient constant for Extended Latin fonts (`U+00A0` - `U+00FF`), that implements the `UnicodeFonts` trait.
///
/// ## `LATIN_UNICODE[0]: U+00A0` `WHITESPACE`
/// ## `LATIN_UNICODE[1]: U+00A1`
/// `"¡"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[2]: U+00A2`
/// `"¢"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░██████░
/// ██░░░░░░
/// ██░░░░░░
/// ░██████░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `LATIN_UNICODE[3]: U+00A3`
/// `"£"`
///
/// ```text
/// ░░███░░░
/// ░██░██░░
/// ░██░░█░░
/// ████░░░░
/// ░██░░░░░
/// ███░░██░
/// ██████░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[4]: U+00A4`
/// `"¤"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░░░██░
/// ░█████░░
/// ░██░██░░
/// ░█████░░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[5]: U+00A5`
/// `"¥"`
///
/// ```text
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ██████░░
/// ░░██░░░░
/// ██████░░
/// ░░██░░░░
/// ░░██░░░░
/// ```
///
/// ## `LATIN_UNICODE[6]: U+00A6`
/// `"¦"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[7]: U+00A7`
/// `"§"`
///
/// ```text
/// ░░█████░
/// ░██░░░██
/// ░░███░░░
/// ░██░██░░
/// ░██░██░░
/// ░░███░░░
/// ██░░██░░
/// ░████░░░
/// ```
///
/// ## `LATIN_UNICODE[8]: U+00A8`
/// `"¨"`
///
/// ```text
/// ██░░██░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[9]: U+00A9`
/// `"©"`
///
/// ```text
/// ░░████░░
/// ░█░░░░█░
/// █░░██░░█
/// █░█░░░░█
/// █░█░░░░█
/// █░░██░░█
/// ░█░░░░█░
/// ░░████░░
/// ```
///
/// ## `LATIN_UNICODE[10]: U+00AA`
/// `"ª"`
///
/// ```text
/// ░░████░░
/// ░██░██░░
/// ░██░██░░
/// ░░█████░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[11]: U+00AB`
/// `"«"`
///
/// ```text
/// ░░░░░░░░
/// ░░██░░██
/// ░██░░██░
/// ██░░██░░
/// ░██░░██░
/// ░░██░░██
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[12]: U+00AC`
/// `"¬"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ██████░░
/// ░░░░██░░
/// ░░░░██░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[13]: U+00AD` `WHITESPACE`
/// ## `LATIN_UNICODE[14]: U+00AE`
/// `"®"`
///
/// ```text
/// ░░████░░
/// ░█░░░░█░
/// █░███░░█
/// █░█░░█░█
/// █░███░░█
/// █░█░░█░█
/// ░█░░░░█░
/// ░░████░░
/// ```
///
/// ## `LATIN_UNICODE[15]: U+00AF`
/// `"¯"`
///
/// ```text
/// ░██████░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[16]: U+00B0`
/// `"°"`
///
/// ```text
/// ░░███░░░
/// ░██░██░░
/// ░██░██░░
/// ░░███░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[17]: U+00B1`
/// `"±"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░██████░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ░██████░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[18]: U+00B2`
/// `"²"`
///
/// ```text
/// ░░███░░░
/// ░░░░██░░
/// ░░░██░░░
/// ░░██░░░░
/// ░░████░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[19]: U+00B2`
/// `"²"`
///
/// ```text
/// ░░███░░░
/// ░░░░██░░
/// ░░░██░░░
/// ░░░░██░░
/// ░░███░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[20]: U+00B2`
/// `"²"`
///
/// ```text
/// ░░░██░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[21]: U+00B5`
/// `"µ"`
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
/// ## `LATIN_UNICODE[22]: U+00B6`
/// `"¶"`
///
/// ```text
/// ░███████
/// ██░██░██
/// ██░██░██
/// ░████░██
/// ░░░██░██
/// ░░░██░██
/// ░░░██░██
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[23]: U+00B7`
/// `"·"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[24]: U+00B8`
/// `"¸"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░░██░░
/// ░████░░░
/// ```
///
/// ## `LATIN_UNICODE[25]: U+00B9`
/// `"¹"`
///
/// ```text
/// ░░░█░░░░
/// ░░██░░░░
/// ░░░█░░░░
/// ░░███░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[26]: U+00BA`
/// `"º"`
///
/// ```text
/// ░░███░░░
/// ░██░██░░
/// ░██░██░░
/// ░░███░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[27]: U+00BB`
/// `"»"`
///
/// ```text
/// ░░░░░░░░
/// ██░░██░░
/// ░██░░██░
/// ░░██░░██
/// ░██░░██░
/// ██░░██░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[28]: U+00BC`
/// `"¼"`
///
/// ```text
/// ██░░░░██
/// ██░░░██░
/// ██░░██░░
/// █░████░█
/// ░░██░███
/// ░██░████
/// ██░░████
/// ██░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[29]: U+00BD`
/// `"½"`
///
/// ```text
/// ██░░░░██
/// ██░░░██░
/// ██░░██░░
/// ██░████░
/// ░░██░░██
/// ░██░░██░
/// ██░░██░░
/// ░░░░████
/// ```
///
/// ## `LATIN_UNICODE[30]: U+00BE`
/// `"¾"`
///
/// ```text
/// ██░░░░░░
/// ░░█░░░██
/// ██░░░██░
/// ░░█░██░█
/// ██░██░██
/// ░░██░█░█
/// ░██░░███
/// ░░░░░░░█
/// ```
///
/// ## `LATIN_UNICODE[31]: U+00BF`
/// `"¿"`
///
/// ```text
/// ░░██░░░░
/// ░░░░░░░░
/// ░░██░░░░
/// ░██░░░░░
/// ██░░░░░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[32]: U+00C0`
/// `"À"`
///
/// ```text
/// ███░░░░░
/// ░░░░░░░░
/// ░░███░░░
/// ░██░██░░
/// ██░░░██░
/// ███████░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[33]: U+00C1`
/// `"Á"`
///
/// ```text
/// ░░░░███░
/// ░░░░░░░░
/// ░░███░░░
/// ░██░██░░
/// ██░░░██░
/// ███████░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[34]: U+00C2`
/// `"Â"`
///
/// ```text
/// ░░███░░░
/// ░██░██░░
/// ░░░░░░░░
/// ░█████░░
/// ██░░░██░
/// ███████░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[35]: U+00C3`
/// `"Ã"`
///
/// ```text
/// ░███░██░
/// ██░███░░
/// ░░░░░░░░
/// ░█████░░
/// ██░░░██░
/// ███████░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[36]: U+00C4`
/// `"Ä"`
///
/// ```text
/// ██░░░██░
/// ░░███░░░
/// ░██░██░░
/// ██░░░██░
/// ███████░
/// ██░░░██░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[37]: U+00C5`
/// `"Å"`
///
/// ```text
/// ░░██░░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ░████░░░
/// ██░░██░░
/// ██████░░
/// ██░░██░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[38]: U+00C6`
/// `"Æ"`
///
/// ```text
/// ░░█████░
/// ░██░██░░
/// ██░░██░░
/// ███████░
/// ██░░██░░
/// ██░░██░░
/// ██░░███░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[39]: U+00C7`
/// `"Ç"`
///
/// ```text
/// ░████░░░
/// ██░░██░░
/// ██░░░░░░
/// ██░░██░░
/// ░████░░░
/// ░░░██░░░
/// ░░░░██░░
/// ░████░░░
/// ```
///
/// ## `LATIN_UNICODE[40]: U+00C8`
/// `"È"`
///
/// ```text
/// ███░░░░░
/// ░░░░░░░░
/// ██████░░
/// ░██░░░░░
/// ░████░░░
/// ░██░░░░░
/// ██████░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[41]: U+00C9`
/// `"É"`
///
/// ```text
/// ░░░███░░
/// ░░░░░░░░
/// ██████░░
/// ░██░░░░░
/// ░████░░░
/// ░██░░░░░
/// ██████░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[42]: U+00CA`
/// `"Ê"`
///
/// ```text
/// ░░██░░░░
/// ░█░░█░░░
/// ██████░░
/// ░██░░░░░
/// ░████░░░
/// ░██░░░░░
/// ██████░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[43]: U+00CB`
/// `"Ë"`
///
/// ```text
/// ░██░██░░
/// ░░░░░░░░
/// ██████░░
/// ░██░░░░░
/// ░████░░░
/// ░██░░░░░
/// ██████░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[44]: U+00CC`
/// `"Ì"`
///
/// ```text
/// ███░░░░░
/// ░░░░░░░░
/// ░████░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[45]: U+00CD`
/// `"Í"`
///
/// ```text
/// ░░░███░░
/// ░░░░░░░░
/// ░████░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[46]: U+00CE`
/// `"Î"`
///
/// ```text
/// ░░██░░░░
/// ░█░░█░░░
/// ░░░░░░░░
/// ░████░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[47]: U+00CF`
/// `"Ï"`
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
/// ## `LATIN_UNICODE[48]: U+00D0`
/// `"Ð"`
///
/// ```text
/// ██████░░
/// ░██░░██░
/// ████░██░
/// ████░██░
/// ░██░░██░
/// ░██░░██░
/// ██████░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[49]: U+00D1`
/// `"Ñ"`
///
/// ```text
/// ██████░░
/// ░░░░░░░░
/// ██░░██░░
/// ███░██░░
/// ██████░░
/// ██░███░░
/// ██░░██░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[50]: U+00D2`
/// `"Ò"`
///
/// ```text
/// ░███░░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░████░░
/// ░██░░██░
/// ░░████░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[51]: U+00D3`
/// `"Ó"`
///
/// ```text
/// ░░░░███░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░████░░
/// ░██░░██░
/// ░░████░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[52]: U+00D4`
/// `"Ô"`
///
/// ```text
/// ░░████░░
/// ░██░░██░
/// ░░░██░░░
/// ░░████░░
/// ░██░░██░
/// ░░████░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[53]: U+00D5`
/// `"Õ"`
///
/// ```text
/// ░███░██░
/// ██░███░░
/// ░░░░░░░░
/// ░█████░░
/// ██░░░██░
/// ██░░░██░
/// ░█████░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[54]: U+00D6`
/// `"Ö"`
///
/// ```text
/// ██░░░░██
/// ░░░██░░░
/// ░░████░░
/// ░██░░██░
/// ░██░░██░
/// ░░████░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[55]: U+00D7`
/// `"×"`
///
/// ```text
/// ░░░░░░░░
/// ░██░██░░
/// ░░███░░░
/// ░░░█░░░░
/// ░░███░░░
/// ░██░██░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[56]: U+00D8`
/// `"Ø"`
///
/// ```text
/// ░░███░█░
/// ░██░██░░
/// ██░░███░
/// ██░████░
/// ████░██░
/// ░██░██░░
/// █░███░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[57]: U+00D9`
/// `"Ù"`
///
/// ```text
/// ░███░░░░
/// ░░░░░░░░
/// ░██░░██░
/// ░██░░██░
/// ░██░░██░
/// ░██░░██░
/// ░░████░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[58]: U+00DA`
/// `"Ú"`
///
/// ```text
/// ░░░░███░
/// ░░░░░░░░
/// ░██░░██░
/// ░██░░██░
/// ░██░░██░
/// ░██░░██░
/// ░░████░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[59]: U+00DB`
/// `"Û"`
///
/// ```text
/// ░░████░░
/// ░██░░██░
/// ░░░░░░░░
/// ░██░░██░
/// ░██░░██░
/// ░██░░██░
/// ░░████░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[60]: U+00DC`
/// `"Ü"`
///
/// ```text
/// ██░░██░░
/// ░░░░░░░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[61]: U+00DD`
/// `"Ý"`
///
/// ```text
/// ░░░░███░
/// ░░░░░░░░
/// ░██░░██░
/// ░██░░██░
/// ░░████░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[62]: U+00DE`
/// `"Þ"`
///
/// ```text
/// ████░░░░
/// ░██░░░░░
/// ░█████░░
/// ░██░░██░
/// ░██░░██░
/// ░█████░░
/// ░██░░░░░
/// ████░░░░
/// ```
///
/// ## `LATIN_UNICODE[63]: U+00DF`
/// `"ß"`
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
/// ## `LATIN_UNICODE[64]: U+00E0`
/// `"à"`
///
/// ```text
/// ███░░░░░
/// ░░░░░░░░
/// ░████░░░
/// ░░░░██░░
/// ░█████░░
/// ██░░██░░
/// ░██████░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[65]: U+00E1`
/// `"á"`
///
/// ```text
/// ░░░███░░
/// ░░░░░░░░
/// ░████░░░
/// ░░░░██░░
/// ░█████░░
/// ██░░██░░
/// ░██████░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[66]: U+00E2`
/// `"â"`
///
/// ```text
/// ░██████░
/// ██░░░░██
/// ░░████░░
/// ░░░░░██░
/// ░░█████░
/// ░██░░██░
/// ░░██████
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[67]: U+00E3`
/// `"ã"`
///
/// ```text
/// ░███░██░
/// ██░███░░
/// ░████░░░
/// ░░░░██░░
/// ░█████░░
/// ██░░██░░
/// ░██████░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[68]: U+00E4`
/// `"ä"`
///
/// ```text
/// ██░░██░░
/// ░░░░░░░░
/// ░████░░░
/// ░░░░██░░
/// ░█████░░
/// ██░░██░░
/// ░██████░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[69]: U+00E5`
/// `"å"`
///
/// ```text
/// ░░██░░░░
/// ░░██░░░░
/// ░████░░░
/// ░░░░██░░
/// ░█████░░
/// ██░░██░░
/// ░██████░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[70]: U+00E6`
/// `"æ"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░███████
/// ░░░░██░░
/// ░███████
/// ██░░██░░
/// ░███████
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[71]: U+00E7`
/// `"ç"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░████░░░
/// ██░░░░░░
/// ██░░░░░░
/// ░████░░░
/// ░░░░██░░
/// ░░███░░░
/// ```
///
/// ## `LATIN_UNICODE[72]: U+00E8`
/// `"è"`
///
/// ```text
/// ███░░░░░
/// ░░░░░░░░
/// ░████░░░
/// ██░░██░░
/// ██████░░
/// ██░░░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[73]: U+00E9`
/// `"é"`
///
/// ```text
/// ░░░███░░
/// ░░░░░░░░
/// ░████░░░
/// ██░░██░░
/// ██████░░
/// ██░░░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[74]: U+00EA`
/// `"ê"`
///
/// ```text
/// ░██████░
/// ██░░░░██
/// ░░████░░
/// ░██░░██░
/// ░██████░
/// ░██░░░░░
/// ░░████░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[75]: U+00EB`
/// `"ë"`
///
/// ```text
/// ██░░██░░
/// ░░░░░░░░
/// ░████░░░
/// ██░░██░░
/// ██████░░
/// ██░░░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[76]: U+00EC`
/// `"ì"`
///
/// ```text
/// ███░░░░░
/// ░░░░░░░░
/// ░███░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[77]: U+00ED`
/// `"í"`
///
/// ```text
/// ░░███░░░
/// ░░░░░░░░
/// ░███░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[78]: U+00EE`
/// `"î"`
///
/// ```text
/// ░█████░░
/// ██░░░██░
/// ░░███░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░████░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[79]: U+00EF`
/// `"ï"`
///
/// ```text
/// ██░░██░░
/// ░░░░░░░░
/// ░███░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[80]: U+00F0`
/// `"ð"`
///
/// ```text
/// ██░██░░░
/// ░███░░░░
/// ██░██░░░
/// ░░░░██░░
/// ░█████░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[81]: U+00F1`
/// `"ñ"`
///
/// ```text
/// ░░░░░░░░
/// █████░░░
/// ░░░░░░░░
/// █████░░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[82]: U+00F2`
/// `"ò"`
///
/// ```text
/// ░░░░░░░░
/// ███░░░░░
/// ░░░░░░░░
/// ░████░░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[83]: U+00F3`
/// `"ó"`
///
/// ```text
/// ░░░░░░░░
/// ░░░███░░
/// ░░░░░░░░
/// ░████░░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[84]: U+00F4`
/// `"ô"`
///
/// ```text
/// ░████░░░
/// ██░░██░░
/// ░░░░░░░░
/// ░████░░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[85]: U+00F5`
/// `"õ"`
///
/// ```text
/// ░███░██░
/// ██░███░░
/// ░░░░░░░░
/// ░████░░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[86]: U+00F6`
/// `"ö"`
///
/// ```text
/// ░░░░░░░░
/// ██░░██░░
/// ░░░░░░░░
/// ░████░░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[87]: U+00F7`
/// `"÷"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ░██████░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[88]: U+00F8`
/// `"ø"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░██░
/// ░░████░░
/// ░██░███░
/// ░██████░
/// ░███░██░
/// ░░████░░
/// ░██░░░░░
/// ```
///
/// ## `LATIN_UNICODE[89]: U+00F9`
/// `"ù"`
///
/// ```text
/// ░░░░░░░░
/// ███░░░░░
/// ░░░░░░░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░██████░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[90]: U+00FA`
/// `"ú"`
///
/// ```text
/// ░░░░░░░░
/// ░░░███░░
/// ░░░░░░░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░██████░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[91]: U+00FB`
/// `"û"`
///
/// ```text
/// ░████░░░
/// ██░░██░░
/// ░░░░░░░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░██████░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[92]: U+00FC`
/// `"ü"`
///
/// ```text
/// ░░░░░░░░
/// ██░░██░░
/// ░░░░░░░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░██████░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[93]: U+00FD`
/// `"ý"`
///
/// ```text
/// ░░░░░░░░
/// ░░░███░░
/// ░░░░░░░░
/// ██░░██░░
/// ██░░██░░
/// ░█████░░
/// ░░░░██░░
/// █████░░░
/// ```
///
/// ## `LATIN_UNICODE[94]: U+00FE`
/// `"þ"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░██░░░░░
/// ░█████░░
/// ░██░░██░
/// ░█████░░
/// ░██░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `LATIN_UNICODE[95]: U+00FF`
/// `"ÿ"`
///
/// ```text
/// ░░░░░░░░
/// ██░░██░░
/// ░░░░░░░░
/// ██░░██░░
/// ██░░██░░
/// ░█████░░
/// ░░░░██░░
/// █████░░░
/// ```
pub const LATIN_FONTS: LatinFonts = LatinFonts(LATIN_UNICODE);

/// Strong-typed collection wrapper for [LATIN_UNICODE](./constant.LATIN_UNICODE.html).
pub struct LatinFonts([FontUnicode; 96]);

impl LatinFonts {
    pub fn new() -> Self {
        LatinFonts(LATIN_UNICODE)
    }
}

impl fmt::Debug for LatinFonts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, stringify!(LATIN_UNICODE))
    }
}

impl PartialEq for LatinFonts {
    fn eq(&self, other: &LatinFonts) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(true, |eq, (a, b)| eq && a == b)
    }
}

impl Default for LatinFonts {
    fn default() -> Self {
        LatinFonts::new()
    }
}

impl UnicodeFonts for LatinFonts {
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
    fn latin_fonts_inner_array_is_sorted_by_unicode_key() {
        let mut sorted = LATIN_UNICODE;
        sorted.sort_by_key(|f| f.char());
        for (idx, key) in sorted.iter().enumerate() {
            println!("{} {:?}", idx, key.char());
            assert_eq!(LATIN_UNICODE[idx].char(), key.char());
        }
    }

    #[test]
    fn latin_set_implements_default_trait_with_method_new() {
        let latin_set: LatinFonts = Default::default();
        assert_eq!(latin_set, LatinFonts::new());
    }

    #[test]
    fn latin_fonts_constant_is_equal_to_a_new_instance() {
        assert_eq!(LATIN_FONTS, LatinFonts::new());
    }

    #[test]
    fn latin_fonts_constant_wraps_basic_unicode_constant() {
        let latin = LatinFonts::new();
        assert!(latin.0.len() == LATIN_UNICODE.len());
        for (idx, font) in latin.0.iter().enumerate() {
            assert_eq!(font, &LATIN_UNICODE[idx]);
        }
    }
}
