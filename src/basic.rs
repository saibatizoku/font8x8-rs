//! Basic Latin. `U+0000` - `U+007F`
use super::{legacy::BASIC_LEGACY, FontUnicode, UnicodeFonts};

use core::fmt;

/// A constant `[FontUnicode; 128]`, for Basic Latin fonts (`U+0000` - `U+007F`).
pub const BASIC_UNICODE: [FontUnicode; 128] = [
    FontUnicode('\u{0000}', BASIC_LEGACY[0]),
    FontUnicode('\u{0001}', BASIC_LEGACY[1]),
    FontUnicode('\u{0002}', BASIC_LEGACY[2]),
    FontUnicode('\u{0003}', BASIC_LEGACY[3]),
    FontUnicode('\u{0004}', BASIC_LEGACY[4]),
    FontUnicode('\u{0005}', BASIC_LEGACY[5]),
    FontUnicode('\u{0006}', BASIC_LEGACY[6]),
    FontUnicode('\u{0007}', BASIC_LEGACY[7]),
    FontUnicode('\u{0008}', BASIC_LEGACY[8]),
    FontUnicode('\u{0009}', BASIC_LEGACY[9]),
    FontUnicode('\u{000A}', BASIC_LEGACY[10]),
    FontUnicode('\u{000B}', BASIC_LEGACY[11]),
    FontUnicode('\u{000C}', BASIC_LEGACY[12]),
    FontUnicode('\u{000D}', BASIC_LEGACY[13]),
    FontUnicode('\u{000E}', BASIC_LEGACY[14]),
    FontUnicode('\u{000F}', BASIC_LEGACY[15]),
    FontUnicode('\u{0010}', BASIC_LEGACY[16]),
    FontUnicode('\u{0011}', BASIC_LEGACY[17]),
    FontUnicode('\u{0012}', BASIC_LEGACY[18]),
    FontUnicode('\u{0013}', BASIC_LEGACY[19]),
    FontUnicode('\u{0014}', BASIC_LEGACY[20]),
    FontUnicode('\u{0015}', BASIC_LEGACY[21]),
    FontUnicode('\u{0016}', BASIC_LEGACY[22]),
    FontUnicode('\u{0017}', BASIC_LEGACY[23]),
    FontUnicode('\u{0018}', BASIC_LEGACY[24]),
    FontUnicode('\u{0019}', BASIC_LEGACY[25]),
    FontUnicode('\u{001A}', BASIC_LEGACY[26]),
    FontUnicode('\u{001B}', BASIC_LEGACY[27]),
    FontUnicode('\u{001C}', BASIC_LEGACY[28]),
    FontUnicode('\u{001D}', BASIC_LEGACY[29]),
    FontUnicode('\u{001E}', BASIC_LEGACY[30]),
    FontUnicode('\u{001F}', BASIC_LEGACY[31]),
    FontUnicode('\u{0020}', BASIC_LEGACY[32]),
    FontUnicode('\u{0021}', BASIC_LEGACY[33]),
    FontUnicode('\u{0022}', BASIC_LEGACY[34]),
    FontUnicode('\u{0023}', BASIC_LEGACY[35]),
    FontUnicode('\u{0024}', BASIC_LEGACY[36]),
    FontUnicode('\u{0025}', BASIC_LEGACY[37]),
    FontUnicode('\u{0026}', BASIC_LEGACY[38]),
    FontUnicode('\u{0027}', BASIC_LEGACY[39]),
    FontUnicode('\u{0028}', BASIC_LEGACY[40]),
    FontUnicode('\u{0029}', BASIC_LEGACY[41]),
    FontUnicode('\u{002A}', BASIC_LEGACY[42]),
    FontUnicode('\u{002B}', BASIC_LEGACY[43]),
    FontUnicode('\u{002C}', BASIC_LEGACY[44]),
    FontUnicode('\u{002D}', BASIC_LEGACY[45]),
    FontUnicode('\u{002E}', BASIC_LEGACY[46]),
    FontUnicode('\u{002F}', BASIC_LEGACY[47]),
    FontUnicode('\u{0030}', BASIC_LEGACY[48]),
    FontUnicode('\u{0031}', BASIC_LEGACY[49]),
    FontUnicode('\u{0032}', BASIC_LEGACY[50]),
    FontUnicode('\u{0033}', BASIC_LEGACY[51]),
    FontUnicode('\u{0034}', BASIC_LEGACY[52]),
    FontUnicode('\u{0035}', BASIC_LEGACY[53]),
    FontUnicode('\u{0036}', BASIC_LEGACY[54]),
    FontUnicode('\u{0037}', BASIC_LEGACY[55]),
    FontUnicode('\u{0038}', BASIC_LEGACY[56]),
    FontUnicode('\u{0039}', BASIC_LEGACY[57]),
    FontUnicode('\u{003A}', BASIC_LEGACY[58]),
    FontUnicode('\u{003B}', BASIC_LEGACY[59]),
    FontUnicode('\u{003C}', BASIC_LEGACY[60]),
    FontUnicode('\u{003D}', BASIC_LEGACY[61]),
    FontUnicode('\u{003E}', BASIC_LEGACY[62]),
    FontUnicode('\u{003F}', BASIC_LEGACY[63]),
    FontUnicode('\u{0040}', BASIC_LEGACY[64]),
    FontUnicode('\u{0041}', BASIC_LEGACY[65]),
    FontUnicode('\u{0042}', BASIC_LEGACY[66]),
    FontUnicode('\u{0043}', BASIC_LEGACY[67]),
    FontUnicode('\u{0044}', BASIC_LEGACY[68]),
    FontUnicode('\u{0045}', BASIC_LEGACY[69]),
    FontUnicode('\u{0046}', BASIC_LEGACY[70]),
    FontUnicode('\u{0047}', BASIC_LEGACY[71]),
    FontUnicode('\u{0048}', BASIC_LEGACY[72]),
    FontUnicode('\u{0049}', BASIC_LEGACY[73]),
    FontUnicode('\u{004A}', BASIC_LEGACY[74]),
    FontUnicode('\u{004B}', BASIC_LEGACY[75]),
    FontUnicode('\u{004C}', BASIC_LEGACY[76]),
    FontUnicode('\u{004D}', BASIC_LEGACY[77]),
    FontUnicode('\u{004E}', BASIC_LEGACY[78]),
    FontUnicode('\u{004F}', BASIC_LEGACY[79]),
    FontUnicode('\u{0050}', BASIC_LEGACY[80]),
    FontUnicode('\u{0051}', BASIC_LEGACY[81]),
    FontUnicode('\u{0052}', BASIC_LEGACY[82]),
    FontUnicode('\u{0053}', BASIC_LEGACY[83]),
    FontUnicode('\u{0054}', BASIC_LEGACY[84]),
    FontUnicode('\u{0055}', BASIC_LEGACY[85]),
    FontUnicode('\u{0056}', BASIC_LEGACY[86]),
    FontUnicode('\u{0057}', BASIC_LEGACY[87]),
    FontUnicode('\u{0058}', BASIC_LEGACY[88]),
    FontUnicode('\u{0059}', BASIC_LEGACY[89]),
    FontUnicode('\u{005A}', BASIC_LEGACY[90]),
    FontUnicode('\u{005B}', BASIC_LEGACY[91]),
    FontUnicode('\u{005C}', BASIC_LEGACY[92]),
    FontUnicode('\u{005D}', BASIC_LEGACY[93]),
    FontUnicode('\u{005E}', BASIC_LEGACY[94]),
    FontUnicode('\u{005F}', BASIC_LEGACY[95]),
    FontUnicode('\u{0060}', BASIC_LEGACY[96]),
    FontUnicode('\u{0061}', BASIC_LEGACY[97]),
    FontUnicode('\u{0062}', BASIC_LEGACY[98]),
    FontUnicode('\u{0063}', BASIC_LEGACY[99]),
    FontUnicode('\u{0064}', BASIC_LEGACY[100]),
    FontUnicode('\u{0065}', BASIC_LEGACY[101]),
    FontUnicode('\u{0066}', BASIC_LEGACY[102]),
    FontUnicode('\u{0067}', BASIC_LEGACY[103]),
    FontUnicode('\u{0068}', BASIC_LEGACY[104]),
    FontUnicode('\u{0069}', BASIC_LEGACY[105]),
    FontUnicode('\u{006A}', BASIC_LEGACY[106]),
    FontUnicode('\u{006B}', BASIC_LEGACY[107]),
    FontUnicode('\u{006C}', BASIC_LEGACY[108]),
    FontUnicode('\u{006D}', BASIC_LEGACY[109]),
    FontUnicode('\u{006E}', BASIC_LEGACY[110]),
    FontUnicode('\u{006F}', BASIC_LEGACY[111]),
    FontUnicode('\u{0070}', BASIC_LEGACY[112]),
    FontUnicode('\u{0071}', BASIC_LEGACY[113]),
    FontUnicode('\u{0072}', BASIC_LEGACY[114]),
    FontUnicode('\u{0073}', BASIC_LEGACY[115]),
    FontUnicode('\u{0074}', BASIC_LEGACY[116]),
    FontUnicode('\u{0075}', BASIC_LEGACY[117]),
    FontUnicode('\u{0076}', BASIC_LEGACY[118]),
    FontUnicode('\u{0077}', BASIC_LEGACY[119]),
    FontUnicode('\u{0078}', BASIC_LEGACY[120]),
    FontUnicode('\u{0079}', BASIC_LEGACY[121]),
    FontUnicode('\u{007A}', BASIC_LEGACY[122]),
    FontUnicode('\u{007B}', BASIC_LEGACY[123]),
    FontUnicode('\u{007C}', BASIC_LEGACY[124]),
    FontUnicode('\u{007D}', BASIC_LEGACY[125]),
    FontUnicode('\u{007E}', BASIC_LEGACY[126]),
    FontUnicode('\u{007F}', BASIC_LEGACY[127]),
];

/// A convenient constant for Basic Latin fonts (`U+0000` - `U+007F`), that implements the `UnicodeFonts` trait.
///
/// ## `BASIC_UNICODE[0]: U+0000` `WHITESPACE`
/// ## `BASIC_UNICODE[1]: U+0001` `WHITESPACE`
/// ## `BASIC_UNICODE[2]: U+0002` `WHITESPACE`
/// ## `BASIC_UNICODE[3]: U+0003` `WHITESPACE`
/// ## `BASIC_UNICODE[4]: U+0004` `WHITESPACE`
/// ## `BASIC_UNICODE[5]: U+0005` `WHITESPACE`
/// ## `BASIC_UNICODE[6]: U+0006` `WHITESPACE`
/// ## `BASIC_UNICODE[7]: U+0007` `WHITESPACE`
/// ## `BASIC_UNICODE[8]: U+0008` `WHITESPACE`
/// ## `BASIC_UNICODE[9]: U+0009` `WHITESPACE`
/// ## `BASIC_UNICODE[10]: U+000A` `WHITESPACE`
/// ## `BASIC_UNICODE[11]: U+000B` `WHITESPACE`
/// ## `BASIC_UNICODE[12]: U+000C` `WHITESPACE`
/// ## `BASIC_UNICODE[13]: U+000D` `WHITESPACE`
/// ## `BASIC_UNICODE[14]: U+000E` `WHITESPACE`
/// ## `BASIC_UNICODE[15]: U+000F` `WHITESPACE`
/// ## `BASIC_UNICODE[16]: U+0010` `WHITESPACE`
/// ## `BASIC_UNICODE[17]: U+0011` `WHITESPACE`
/// ## `BASIC_UNICODE[18]: U+0012` `WHITESPACE`
/// ## `BASIC_UNICODE[19]: U+0013` `WHITESPACE`
/// ## `BASIC_UNICODE[20]: U+0014` `WHITESPACE`
/// ## `BASIC_UNICODE[21]: U+0015` `WHITESPACE`
/// ## `BASIC_UNICODE[22]: U+0016` `WHITESPACE`
/// ## `BASIC_UNICODE[23]: U+0017` `WHITESPACE`
/// ## `BASIC_UNICODE[24]: U+0018` `WHITESPACE`
/// ## `BASIC_UNICODE[25]: U+0019` `WHITESPACE`
/// ## `BASIC_UNICODE[26]: U+001A` `WHITESPACE`
/// ## `BASIC_UNICODE[27]: U+001B` `WHITESPACE`
/// ## `BASIC_UNICODE[28]: U+001C` `WHITESPACE`
/// ## `BASIC_UNICODE[29]: U+001D` `WHITESPACE`
/// ## `BASIC_UNICODE[30]: U+001E` `WHITESPACE`
/// ## `BASIC_UNICODE[31]: U+001F` `WHITESPACE`
/// ## `BASIC_UNICODE[32]: U+0020` `WHITESPACE`
/// ## `BASIC_UNICODE[33]: U+0021`
/// `"!"`
///
/// ```text
/// ░░░██░░░
/// ░░████░░
/// ░░████░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[34]: U+0022`
/// `"\"`
///
/// ```text
/// ░██░██░░
/// ░██░██░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[35]: U+0023`
/// `"#"`
///
/// ```text
/// ░██░██░░
/// ░██░██░░
/// ███████░
/// ░██░██░░
/// ███████░
/// ░██░██░░
/// ░██░██░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[36]: U+0024`
/// `"$"`
///
/// ```text
/// ░░██░░░░
/// ░█████░░
/// ██░░░░░░
/// ░████░░░
/// ░░░░██░░
/// █████░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[37]: U+0025`
/// `"%"`
///
/// ```text
/// ░░░░░░░░
/// ██░░░██░
/// ██░░██░░
/// ░░░██░░░
/// ░░██░░░░
/// ░██░░██░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[38]: U+0026`
/// `"&"`
///
/// ```text
/// ░░███░░░
/// ░██░██░░
/// ░░███░░░
/// ░███░██░
/// ██░███░░
/// ██░░██░░
/// ░███░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[39]: U+0027`
/// `"'"`
///
/// ```text
/// ░██░░░░░
/// ░██░░░░░
/// ██░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[40]: U+0028`
/// `"("`
///
/// ```text
/// ░░░██░░░
/// ░░██░░░░
/// ░██░░░░░
/// ░██░░░░░
/// ░██░░░░░
/// ░░██░░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[41]: U+0029`
/// `")"`
///
/// ```text
/// ░██░░░░░
/// ░░██░░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░██░░░░
/// ░██░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[42]: U+002A`
/// `"*"`
///
/// ```text
/// ░░░░░░░░
/// ░██░░██░
/// ░░████░░
/// ████████
/// ░░████░░
/// ░██░░██░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[43]: U+002B`
/// `"+"`
///
/// ```text
/// ░░░░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ██████░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[44]: U+002C`
/// `","`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░██░░░░░
/// ```
///
/// ## `BASIC_UNICODE[45]: U+002D`
/// `"-"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ██████░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[46]: U+002E`
/// `"."`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[47]: U+002F`
/// `"/"`
///
/// ```text
/// ░░░░░██░
/// ░░░░██░░
/// ░░░██░░░
/// ░░██░░░░
/// ░██░░░░░
/// ██░░░░░░
/// █░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[48]: U+0030`
/// `"0"`
///
/// ```text
/// ░█████░░
/// ██░░░██░
/// ██░░███░
/// ██░████░
/// ████░██░
/// ███░░██░
/// ░█████░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[49]: U+0031`
/// `"1"`
///
/// ```text
/// ░░██░░░░
/// ░███░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ██████░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[50]: U+0032`
/// `"2"`
///
/// ```text
/// ░████░░░
/// ██░░██░░
/// ░░░░██░░
/// ░░███░░░
/// ░██░░░░░
/// ██░░██░░
/// ██████░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[51]: U+0033`
/// `"3"`
///
/// ```text
/// ░████░░░
/// ██░░██░░
/// ░░░░██░░
/// ░░███░░░
/// ░░░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[52]: U+0034`
/// `"4"`
///
/// ```text
/// ░░░███░░
/// ░░████░░
/// ░██░██░░
/// ██░░██░░
/// ███████░
/// ░░░░██░░
/// ░░░████░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[53]: U+0035`
/// `"5"`
///
/// ```text
/// ██████░░
/// ██░░░░░░
/// █████░░░
/// ░░░░██░░
/// ░░░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[54]: U+0036`
/// `"6"`
///
/// ```text
/// ░░███░░░
/// ░██░░░░░
/// ██░░░░░░
/// █████░░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[55]: U+0037`
/// `"7"`
///
/// ```text
/// ██████░░
/// ██░░██░░
/// ░░░░██░░
/// ░░░██░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[56]: U+0038`
/// `"8"`
///
/// ```text
/// ░████░░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[57]: U+0039`
/// `"9"`
///
/// ```text
/// ░████░░░
/// ██░░██░░
/// ██░░██░░
/// ░█████░░
/// ░░░░██░░
/// ░░░██░░░
/// ░███░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[58]: U+003A`
/// `":"`
///
/// ```text
/// ░░░░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[59]: U+003B`
/// `";"`
///
/// ```text
/// ░░░░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░██░░░░░
/// ```
///
/// ## `BASIC_UNICODE[60]: U+003C`
/// `"<"`
///
/// ```text
/// ░░░██░░░
/// ░░██░░░░
/// ░██░░░░░
/// ██░░░░░░
/// ░██░░░░░
/// ░░██░░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[61]: U+003D`
/// `"="`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██████░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ██████░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[62]: U+003E`
/// `">"`
///
/// ```text
/// ░██░░░░░
/// ░░██░░░░
/// ░░░██░░░
/// ░░░░██░░
/// ░░░██░░░
/// ░░██░░░░
/// ░██░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[63]: U+003F`
/// `"?"`
///
/// ```text
/// ░████░░░
/// ██░░██░░
/// ░░░░██░░
/// ░░░██░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[64]: U+0040`
/// `"@"`
///
/// ```text
/// ░█████░░
/// ██░░░██░
/// ██░████░
/// ██░████░
/// ██░████░
/// ██░░░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[65]: U+0041`
/// `"A"`
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
/// ## `BASIC_UNICODE[66]: U+0042`
/// `"B"`
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
/// ## `BASIC_UNICODE[67]: U+0043`
/// `"C"`
///
/// ```text
/// ░░████░░
/// ░██░░██░
/// ██░░░░░░
/// ██░░░░░░
/// ██░░░░░░
/// ░██░░██░
/// ░░████░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[68]: U+0044`
/// `"D"`
///
/// ```text
/// █████░░░
/// ░██░██░░
/// ░██░░██░
/// ░██░░██░
/// ░██░░██░
/// ░██░██░░
/// █████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[69]: U+0045`
/// `"E"`
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
/// ## `BASIC_UNICODE[70]: U+0046`
/// `"F"`
///
/// ```text
/// ███████░
/// ░██░░░█░
/// ░██░█░░░
/// ░████░░░
/// ░██░█░░░
/// ░██░░░░░
/// ████░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[71]: U+0047`
/// `"G"`
///
/// ```text
/// ░░████░░
/// ░██░░██░
/// ██░░░░░░
/// ██░░░░░░
/// ██░░███░
/// ░██░░██░
/// ░░█████░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[72]: U+0048`
/// `"H"`
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
/// ## `BASIC_UNICODE[73]: U+0049`
/// `"I"`
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
/// ## `BASIC_UNICODE[74]: U+004A`
/// `"J"`
///
/// ```text
/// ░░░████░
/// ░░░░██░░
/// ░░░░██░░
/// ░░░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[75]: U+004B`
/// `"K"`
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
/// ## `BASIC_UNICODE[76]: U+004C`
/// `"L"`
///
/// ```text
/// ████░░░░
/// ░██░░░░░
/// ░██░░░░░
/// ░██░░░░░
/// ░██░░░█░
/// ░██░░██░
/// ███████░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[77]: U+004D`
/// `"M"`
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
/// ## `BASIC_UNICODE[78]: U+004E`
/// `"N"`
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
/// ## `BASIC_UNICODE[79]: U+004F`
/// `"O"`
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
/// ## `BASIC_UNICODE[80]: U+0050`
/// `"P"`
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
/// ## `BASIC_UNICODE[81]: U+0051`
/// `"Q"`
///
/// ```text
/// ░████░░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██░███░░
/// ░████░░░
/// ░░░███░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[82]: U+0052`
/// `"R"`
///
/// ```text
/// ██████░░
/// ░██░░██░
/// ░██░░██░
/// ░█████░░
/// ░██░██░░
/// ░██░░██░
/// ███░░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[83]: U+0053`
/// `"S"`
///
/// ```text
/// ░████░░░
/// ██░░██░░
/// ███░░░░░
/// ░███░░░░
/// ░░░███░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[84]: U+0054`
/// `"T"`
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
/// ## `BASIC_UNICODE[85]: U+0055`
/// `"U"`
///
/// ```text
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██████░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[86]: U+0056`
/// `"V"`
///
/// ```text
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ░░██░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[87]: U+0057`
/// `"W"`
///
/// ```text
/// ██░░░██░
/// ██░░░██░
/// ██░░░██░
/// ██░█░██░
/// ███████░
/// ███░███░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[88]: U+0058`
/// `"X"`
///
/// ```text
/// ██░░░██░
/// ██░░░██░
/// ░██░██░░
/// ░░███░░░
/// ░░███░░░
/// ░██░██░░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[89]: U+0059`
/// `"Y"`
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
/// ## `BASIC_UNICODE[90]: U+005A`
/// `"Z"`
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
/// ## `BASIC_UNICODE[91]: U+005B`
/// `"["`
///
/// ```text
/// ░████░░░
/// ░██░░░░░
/// ░██░░░░░
/// ░██░░░░░
/// ░██░░░░░
/// ░██░░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[92]: U+005C`
/// `"\"`
///
/// ```text
/// ██░░░░░░
/// ░██░░░░░
/// ░░██░░░░
/// ░░░██░░░
/// ░░░░██░░
/// ░░░░░██░
/// ░░░░░░█░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[93]: U+005D`
/// `"]"`
///
/// ```text
/// ░████░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[94]: U+005E`
/// `"^"`
///
/// ```text
/// ░░░█░░░░
/// ░░███░░░
/// ░██░██░░
/// ██░░░██░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[95]: U+005F`
/// `"_"`
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
/// ## `BASIC_UNICODE[96]: U+0060`
/// <pre>"`"</pre>
///
/// ```text
/// ░░██░░░░
/// ░░██░░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[97]: U+0061`
/// `"a"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░████░░░
/// ░░░░██░░
/// ░█████░░
/// ██░░██░░
/// ░███░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[98]: U+0062`
/// `"b"`
///
/// ```text
/// ███░░░░░
/// ░██░░░░░
/// ░██░░░░░
/// ░█████░░
/// ░██░░██░
/// ░██░░██░
/// ██░███░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[99]: U+0063`
/// `"c"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░████░░░
/// ██░░██░░
/// ██░░░░░░
/// ██░░██░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[100]: U+0064`
/// `"d"`
///
/// ```text
/// ░░░███░░
/// ░░░░██░░
/// ░░░░██░░
/// ░█████░░
/// ██░░██░░
/// ██░░██░░
/// ░███░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[101]: U+0065`
/// `"e"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░████░░░
/// ██░░██░░
/// ██████░░
/// ██░░░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[102]: U+0066`
/// `"f"`
///
/// ```text
/// ░░███░░░
/// ░██░██░░
/// ░██░░░░░
/// ████░░░░
/// ░██░░░░░
/// ░██░░░░░
/// ████░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[103]: U+0067`
/// `"g"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░███░██░
/// ██░░██░░
/// ██░░██░░
/// ░█████░░
/// ░░░░██░░
/// █████░░░
/// ```
///
/// ## `BASIC_UNICODE[104]: U+0068`
/// `"h"`
///
/// ```text
/// ███░░░░░
/// ░██░░░░░
/// ░██░██░░
/// ░███░██░
/// ░██░░██░
/// ░██░░██░
/// ███░░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[105]: U+0069`
/// `"i"`
///
/// ```text
/// ░░██░░░░
/// ░░░░░░░░
/// ░███░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[106]: U+006A`
/// `"j"`
///
/// ```text
/// ░░░░██░░
/// ░░░░░░░░
/// ░░░░██░░
/// ░░░░██░░
/// ░░░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░████░░░
/// ```
///
/// ## `BASIC_UNICODE[107]: U+006B`
/// `"k"`
///
/// ```text
/// ███░░░░░
/// ░██░░░░░
/// ░██░░██░
/// ░██░██░░
/// ░████░░░
/// ░██░██░░
/// ███░░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[108]: U+006C`
/// `"l"`
///
/// ```text
/// ░███░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[109]: U+006D`
/// `"m"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░░██░░
/// ███████░
/// ███████░
/// ██░█░██░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[110]: U+006E`
/// `"n"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// █████░░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[111]: U+006F`
/// `"o"`
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
/// ## `BASIC_UNICODE[112]: U+0070`
/// `"p"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░███░░
/// ░██░░██░
/// ░██░░██░
/// ░█████░░
/// ░██░░░░░
/// ████░░░░
/// ```
///
/// ## `BASIC_UNICODE[113]: U+0071`
/// `"q"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░███░██░
/// ██░░██░░
/// ██░░██░░
/// ░█████░░
/// ░░░░██░░
/// ░░░████░
/// ```
///
/// ## `BASIC_UNICODE[114]: U+0072`
/// `"r"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░███░░
/// ░███░██░
/// ░██░░██░
/// ░██░░░░░
/// ████░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[115]: U+0073`
/// `"s"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░█████░░
/// ██░░░░░░
/// ░████░░░
/// ░░░░██░░
/// █████░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[116]: U+0074`
/// `"t"`
///
/// ```text
/// ░░░█░░░░
/// ░░██░░░░
/// ░█████░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░██░█░░
/// ░░░██░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[117]: U+0075`
/// `"u"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░███░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[118]: U+0076`
/// `"v"`
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
/// ## `BASIC_UNICODE[119]: U+0077`
/// `"w"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░░░██░
/// ██░█░██░
/// ███████░
/// ███████░
/// ░██░██░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[120]: U+0078`
/// `"x"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░░░██░
/// ░██░██░░
/// ░░███░░░
/// ░██░██░░
/// ██░░░██░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[121]: U+0079`
/// `"y"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░░██░░
/// ██░░██░░
/// ██░░██░░
/// ░█████░░
/// ░░░░██░░
/// █████░░░
/// ```
///
/// ## `BASIC_UNICODE[122]: U+007A`
/// `"z"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ██████░░
/// █░░██░░░
/// ░░██░░░░
/// ░██░░█░░
/// ██████░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[123]: U+007B`
/// `"{"`
///
/// ```text
/// ░░░███░░
/// ░░██░░░░
/// ░░██░░░░
/// ███░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░░███░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[124]: U+007C`
/// `"|"`
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
/// ## `BASIC_UNICODE[125]: U+007D`
/// `"}"`
///
/// ```text
/// ███░░░░░
/// ░░██░░░░
/// ░░██░░░░
/// ░░░███░░
/// ░░██░░░░
/// ░░██░░░░
/// ███░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[126]: U+007E`
/// `"~"`
///
/// ```text
/// ░███░██░
/// ██░███░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BASIC_UNICODE[127]: U+007F` `WHITESPACE`
pub const BASIC_FONTS: BasicFonts = BasicFonts(BASIC_UNICODE);

/// Strong-typed collection wrapper for [BASIC_UNICODE](./constant.BASIC_UNICODE.html).
pub struct BasicFonts([FontUnicode; 128]);

impl fmt::Debug for BasicFonts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, stringify!(BASIC_UNICODE))
    }
}

impl PartialEq for BasicFonts {
    fn eq(&self, other: &BasicFonts) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(true, |eq, (a, b)| eq && a == b)
    }
}

impl BasicFonts {
    /// Create a new collection of `BASIC_UNICODE` fonts.
    pub fn new() -> Self {
        BasicFonts(BASIC_UNICODE)
    }
}

impl Default for BasicFonts {
    fn default() -> Self {
        BasicFonts::new()
    }
}

impl UnicodeFonts for BasicFonts {
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
    fn basic_fonts_inner_array_is_sorted_by_unicode_key() {
        let mut sorted = BASIC_UNICODE;
        sorted.sort_by_key(|f| f.char());
        for (idx, key) in sorted.iter().enumerate() {
            assert_eq!(BASIC_UNICODE[idx].char(), key.char());
        }
    }

    #[test]
    fn basic_set_implements_default_trait_with_method_new() {
        let basic_set: BasicFonts = Default::default();
        assert_eq!(basic_set, BasicFonts::new());
    }

    #[test]
    fn basic_fonts_constant_is_equal_to_a_new_instance() {
        assert_eq!(BASIC_FONTS, BasicFonts::new());
    }

    #[test]
    fn basic_fonts_constant_wraps_basic_unicode_constant() {
        let basic = BasicFonts::new();
        assert!(basic.0.len() == BASIC_UNICODE.len());
        for (idx, font) in basic.0.iter().enumerate() {
            assert_eq!(font, &BASIC_UNICODE[idx]);
        }
    }
}
