//!  Box Drawing. `U+2500 - U+257F`
use super::{legacy::BOX_LEGACY, FontUnicode, UnicodeFonts};
use core::fmt;

/// A constant `[FontUnicode; 128]`, for Box Element fonts (`U+2500` - `U+257F`).
pub const BOX_UNICODE: [FontUnicode; 128] = [
    FontUnicode('\u{2500}', BOX_LEGACY[0]),
    FontUnicode('\u{2501}', BOX_LEGACY[1]),
    FontUnicode('\u{2502}', BOX_LEGACY[2]),
    FontUnicode('\u{2503}', BOX_LEGACY[3]),
    FontUnicode('\u{2504}', BOX_LEGACY[4]),
    FontUnicode('\u{2505}', BOX_LEGACY[5]),
    FontUnicode('\u{2506}', BOX_LEGACY[6]),
    FontUnicode('\u{2507}', BOX_LEGACY[7]),
    FontUnicode('\u{2508}', BOX_LEGACY[8]),
    FontUnicode('\u{2509}', BOX_LEGACY[9]),
    FontUnicode('\u{250A}', BOX_LEGACY[10]),
    FontUnicode('\u{250B}', BOX_LEGACY[11]),
    FontUnicode('\u{250C}', BOX_LEGACY[12]),
    FontUnicode('\u{250D}', BOX_LEGACY[13]),
    FontUnicode('\u{250E}', BOX_LEGACY[14]),
    FontUnicode('\u{250F}', BOX_LEGACY[15]),
    FontUnicode('\u{2510}', BOX_LEGACY[16]),
    FontUnicode('\u{2511}', BOX_LEGACY[17]),
    FontUnicode('\u{2512}', BOX_LEGACY[18]),
    FontUnicode('\u{2513}', BOX_LEGACY[19]),
    FontUnicode('\u{2514}', BOX_LEGACY[20]),
    FontUnicode('\u{2515}', BOX_LEGACY[21]),
    FontUnicode('\u{2516}', BOX_LEGACY[22]),
    FontUnicode('\u{2517}', BOX_LEGACY[23]),
    FontUnicode('\u{2518}', BOX_LEGACY[24]),
    FontUnicode('\u{2519}', BOX_LEGACY[25]),
    FontUnicode('\u{251A}', BOX_LEGACY[26]),
    FontUnicode('\u{251B}', BOX_LEGACY[27]),
    FontUnicode('\u{251C}', BOX_LEGACY[28]),
    FontUnicode('\u{251D}', BOX_LEGACY[29]),
    FontUnicode('\u{251E}', BOX_LEGACY[30]),
    FontUnicode('\u{251F}', BOX_LEGACY[31]),
    FontUnicode('\u{2520}', BOX_LEGACY[32]),
    FontUnicode('\u{2521}', BOX_LEGACY[33]),
    FontUnicode('\u{2522}', BOX_LEGACY[34]),
    FontUnicode('\u{2523}', BOX_LEGACY[35]),
    FontUnicode('\u{2524}', BOX_LEGACY[36]),
    FontUnicode('\u{2525}', BOX_LEGACY[37]),
    FontUnicode('\u{2526}', BOX_LEGACY[38]),
    FontUnicode('\u{2527}', BOX_LEGACY[39]),
    FontUnicode('\u{2528}', BOX_LEGACY[40]),
    FontUnicode('\u{2529}', BOX_LEGACY[41]),
    FontUnicode('\u{252A}', BOX_LEGACY[42]),
    FontUnicode('\u{252B}', BOX_LEGACY[43]),
    FontUnicode('\u{252C}', BOX_LEGACY[44]),
    FontUnicode('\u{252D}', BOX_LEGACY[45]),
    FontUnicode('\u{252E}', BOX_LEGACY[46]),
    FontUnicode('\u{252F}', BOX_LEGACY[47]),
    FontUnicode('\u{2530}', BOX_LEGACY[48]),
    FontUnicode('\u{2531}', BOX_LEGACY[49]),
    FontUnicode('\u{2532}', BOX_LEGACY[50]),
    FontUnicode('\u{2533}', BOX_LEGACY[51]),
    FontUnicode('\u{2534}', BOX_LEGACY[52]),
    FontUnicode('\u{2535}', BOX_LEGACY[53]),
    FontUnicode('\u{2536}', BOX_LEGACY[54]),
    FontUnicode('\u{2537}', BOX_LEGACY[55]),
    FontUnicode('\u{2538}', BOX_LEGACY[56]),
    FontUnicode('\u{2539}', BOX_LEGACY[57]),
    FontUnicode('\u{253A}', BOX_LEGACY[58]),
    FontUnicode('\u{253B}', BOX_LEGACY[59]),
    FontUnicode('\u{253C}', BOX_LEGACY[60]),
    FontUnicode('\u{253D}', BOX_LEGACY[61]),
    FontUnicode('\u{253E}', BOX_LEGACY[62]),
    FontUnicode('\u{253F}', BOX_LEGACY[63]),
    FontUnicode('\u{2540}', BOX_LEGACY[64]),
    FontUnicode('\u{2541}', BOX_LEGACY[65]),
    FontUnicode('\u{2542}', BOX_LEGACY[66]),
    FontUnicode('\u{2543}', BOX_LEGACY[67]),
    FontUnicode('\u{2544}', BOX_LEGACY[68]),
    FontUnicode('\u{2545}', BOX_LEGACY[69]),
    FontUnicode('\u{2546}', BOX_LEGACY[70]),
    FontUnicode('\u{2547}', BOX_LEGACY[71]),
    FontUnicode('\u{254B}', BOX_LEGACY[72]),
    FontUnicode('\u{254A}', BOX_LEGACY[73]),
    FontUnicode('\u{2549}', BOX_LEGACY[74]),
    FontUnicode('\u{254B}', BOX_LEGACY[75]),
    FontUnicode('\u{254C}', BOX_LEGACY[76]),
    FontUnicode('\u{254D}', BOX_LEGACY[77]),
    FontUnicode('\u{254E}', BOX_LEGACY[78]),
    FontUnicode('\u{254F}', BOX_LEGACY[79]),
    FontUnicode('\u{2550}', BOX_LEGACY[80]),
    FontUnicode('\u{2551}', BOX_LEGACY[81]),
    FontUnicode('\u{2552}', BOX_LEGACY[82]),
    FontUnicode('\u{2553}', BOX_LEGACY[83]),
    FontUnicode('\u{2554}', BOX_LEGACY[84]),
    FontUnicode('\u{2555}', BOX_LEGACY[85]),
    FontUnicode('\u{2556}', BOX_LEGACY[86]),
    FontUnicode('\u{2557}', BOX_LEGACY[87]),
    FontUnicode('\u{2558}', BOX_LEGACY[88]),
    FontUnicode('\u{2559}', BOX_LEGACY[89]),
    FontUnicode('\u{255A}', BOX_LEGACY[90]),
    FontUnicode('\u{255B}', BOX_LEGACY[91]),
    FontUnicode('\u{255C}', BOX_LEGACY[92]),
    FontUnicode('\u{255D}', BOX_LEGACY[93]),
    FontUnicode('\u{255E}', BOX_LEGACY[94]),
    FontUnicode('\u{255F}', BOX_LEGACY[95]),
    FontUnicode('\u{2560}', BOX_LEGACY[96]),
    FontUnicode('\u{2561}', BOX_LEGACY[97]),
    FontUnicode('\u{2562}', BOX_LEGACY[98]),
    FontUnicode('\u{2563}', BOX_LEGACY[99]),
    FontUnicode('\u{2564}', BOX_LEGACY[100]),
    FontUnicode('\u{2565}', BOX_LEGACY[101]),
    FontUnicode('\u{2566}', BOX_LEGACY[102]),
    FontUnicode('\u{2567}', BOX_LEGACY[103]),
    FontUnicode('\u{2568}', BOX_LEGACY[104]),
    FontUnicode('\u{2569}', BOX_LEGACY[105]),
    FontUnicode('\u{256A}', BOX_LEGACY[106]),
    FontUnicode('\u{256B}', BOX_LEGACY[107]),
    FontUnicode('\u{256C}', BOX_LEGACY[108]),
    FontUnicode('\u{256D}', BOX_LEGACY[109]),
    FontUnicode('\u{256E}', BOX_LEGACY[110]),
    FontUnicode('\u{256F}', BOX_LEGACY[111]),
    FontUnicode('\u{2570}', BOX_LEGACY[112]),
    FontUnicode('\u{2571}', BOX_LEGACY[113]),
    FontUnicode('\u{2572}', BOX_LEGACY[114]),
    FontUnicode('\u{2573}', BOX_LEGACY[115]),
    FontUnicode('\u{2574}', BOX_LEGACY[116]),
    FontUnicode('\u{2575}', BOX_LEGACY[117]),
    FontUnicode('\u{2576}', BOX_LEGACY[118]),
    FontUnicode('\u{2577}', BOX_LEGACY[119]),
    FontUnicode('\u{2578}', BOX_LEGACY[120]),
    FontUnicode('\u{2579}', BOX_LEGACY[121]),
    FontUnicode('\u{257A}', BOX_LEGACY[122]),
    FontUnicode('\u{257B}', BOX_LEGACY[123]),
    FontUnicode('\u{257C}', BOX_LEGACY[124]),
    FontUnicode('\u{257D}', BOX_LEGACY[125]),
    FontUnicode('\u{257E}', BOX_LEGACY[126]),
    FontUnicode('\u{257F}', BOX_LEGACY[127]),
];

/// A convenient constant for Box Element fonts (`U+2500` - `U+257F`), that implements the `UnicodeFonts` trait.
///
/// ## `BOX_UNICODE[0]: U+2500`
/// `"─"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[1]: U+2501`
/// `"━"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████████
/// ████████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[2]: U+2502`
/// `"│"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[3]: U+2503`
/// `"┃"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[4]: U+2504`
/// `"┄"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░███░█
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[5]: U+2505`
/// `"┅"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░███░█
/// ██░███░█
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[6]: U+2506`
/// `"┆"`
///
/// ```text
/// ░░░█░░░░
/// ░░░░░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░░░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[7]: U+2507`
/// `"┇"`
///
/// ```text
/// ░░░██░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[8]: U+2508`
/// `"┈"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// █░█░█░█░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[9]: U+2509`
/// `"┉"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// █░█░█░█░
/// █░█░█░█░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[10]: U+250A`
/// `"┊"`
///
/// ```text
/// ░░░░░░░░
/// ░░░█░░░░
/// ░░░░░░░░
/// ░░░█░░░░
/// ░░░░░░░░
/// ░░░█░░░░
/// ░░░░░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[11]: U+250B`
/// `"┋"`
///
/// ```text
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[12]: U+250C`
/// `"┌"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░█████
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[13]: U+250D`
/// `"┍"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░█████
/// ░░░█████
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[14]: U+250E`
/// `"┎"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░█████
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[15]: U+250F`
/// `"┏"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░█████
/// ░░░█████
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[16]: U+2510`
/// `"┐"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[17]: U+2511`
/// `"┑"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████░░░░
/// ████░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[18]: U+2512`
/// `"┒"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// █████░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[19]: U+2513`
/// `"┓"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// █████░░░
/// █████░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[20]: U+2514`
/// `"└"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[21]: U+2515`
/// `"┕"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█████
/// ░░░█████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[22]: U+2516`
/// `"┖"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░█████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[23]: U+2517`
/// `"┗"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░█████
/// ░░░█████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[24]: U+2518`
/// `"┘"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ████░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[25]: U+2519`
/// `"┙"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ████░░░░
/// ████░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[26]: U+251A`
/// `"┚"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// █████░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[27]: U+251B`
/// `"┛"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// █████░░░
/// █████░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[28]: U+251C`
/// `"├"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█████
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[29]: U+251D`
/// `"┝"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█████
/// ░░░█████
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[30]: U+251E`
/// `"┞"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░█████
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[31]: U+251F`
/// `"┟"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█████
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[32]: U+2520`
/// `"┠"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░█████
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[33]: U+2521`
/// `"┡"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░█████
/// ░░░█████
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[34]: U+2522`
/// `"┢"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█████
/// ░░░█████
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[35]: U+2523`
/// `"┣"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░█████
/// ░░░█████
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[36]: U+2524`
/// `"┤"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ████░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[37]: U+2525`
/// `"┥"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ████░░░░
/// ████░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[38]: U+2526`
/// `"┦"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// █████░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[39]: U+2527`
/// `"┧"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// █████░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[40]: U+2528`
/// `"┨"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// █████░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[41]: U+2529`
/// `"┩"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// █████░░░
/// █████░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[42]: U+252A`
/// `"┪"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// █████░░░
/// █████░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[43]: U+252B`
/// `"┫"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// █████░░░
/// █████░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[44]: U+252C`
/// `"┬"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████████
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[45]: U+252D`
/// `"┭"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████░░░░
/// ████████
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[46]: U+252E`
/// `"┮"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░█████
/// ████████
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[47]: U+252F`
/// `"┯"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████████
/// ████████
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[48]: U+2530`
/// `"┰"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████████
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[49]: U+2531`
/// `"┱"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// █████░░░
/// ████████
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[50]: U+2532`
/// `"┲"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░█████
/// ████████
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[51]: U+2533`
/// `"┳"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████████
/// ████████
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[52]: U+2534`
/// `"┴"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ████████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[53]: U+2535`
/// `"┵"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ████░░░░
/// ████████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[54]: U+2536`
/// `"┶"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█████
/// ████████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[55]: U+2537`
/// `"┷"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ████████
/// ████████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[56]: U+2538`
/// `"┸"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ████████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[57]: U+2539`
/// `"┹"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// █████░░░
/// ████████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[58]: U+253A`
/// `"┺"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░█████
/// ████████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[59]: U+253B`
/// `"┻"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ████████
/// ████████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[60]: U+253C`
/// `"┼"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ████████
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[61]: U+253D`
/// `"┽"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ████░░░░
/// ████████
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[62]: U+253E`
/// `"┾"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█████
/// ████████
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[63]: U+253F`
/// `"┿"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ████████
/// ████████
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[64]: U+2540`
/// `"╀"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ████████
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[65]: U+2541`
/// `"╁"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ████████
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[66]: U+2542`
/// `"╂"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ████████
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[67]: U+2543`
/// `"╃"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// █████░░░
/// ████████
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[68]: U+2544`
/// `"╄"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░█████
/// ████████
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[69]: U+2545`
/// `"╅"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// █████░░░
/// ████████
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[70]: U+2546`
/// `"╆"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█████
/// ████████
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[71]: U+2547`
/// `"╇"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ████████
/// ████████
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[72]: U+254B`
/// `"╋"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ████████
/// ████████
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[73]: U+254A`
/// `"╊"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░█████
/// ████████
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[74]: U+2549`
/// `"╉"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// █████░░░
/// ████████
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[75]: U+254B`
/// `"╋"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ████████
/// ████████
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[76]: U+254C`
/// `"╌"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ███░░███
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[77]: U+254D`
/// `"╍"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ███░░███
/// ███░░███
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[78]: U+254E`
/// `"╎"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[79]: U+254F`
/// `"╏"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[80]: U+2550`
/// `"═"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████████
/// ░░░░░░░░
/// ████████
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[81]: U+2551`
/// `"║"`
///
/// ```text
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ```
///
/// ## `BOX_UNICODE[82]: U+2552`
/// `"╒"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░█████
/// ░░░█░░░░
/// ░░░█████
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[83]: U+2553`
/// `"╓"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░██████
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ```
///
/// ## `BOX_UNICODE[84]: U+2554`
/// `"╔"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░██████
/// ░░█░░░░░
/// ░░█░████
/// ░░█░█░░░
/// ░░█░█░░░
/// ```
///
/// ## `BOX_UNICODE[85]: U+2555`
/// `"╕"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████░░░░
/// ░░░█░░░░
/// ████░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[86]: U+2556`
/// `"╖"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// █████░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ```
///
/// ## `BOX_UNICODE[87]: U+2557`
/// `"╗"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// █████░░░
/// ░░░░█░░░
/// ███░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ```
///
/// ## `BOX_UNICODE[88]: U+2558`
/// `"╘"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█████
/// ░░░█░░░░
/// ░░░█████
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[89]: U+2559`
/// `"╙"`
///
/// ```text
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░██████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[90]: U+255A`
/// `"╚"`
///
/// ```text
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░████
/// ░░█░░░░░
/// ░░██████
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[91]: U+255B`
/// `"╛"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ████░░░░
/// ░░░█░░░░
/// ████░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[92]: U+255C`
/// `"╜"`
///
/// ```text
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// █████░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[93]: U+255D`
/// `"╝"`
///
/// ```text
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ███░█░░░
/// ░░░░█░░░
/// █████░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[94]: U+255E`
/// `"╞"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█████
/// ░░░█░░░░
/// ░░░█████
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[95]: U+255F`
/// `"╟"`
///
/// ```text
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░████
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ```
///
/// ## `BOX_UNICODE[96]: U+2560`
/// `"╠"`
///
/// ```text
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░████
/// ░░█░░░░░
/// ░░█░████
/// ░░█░█░░░
/// ░░█░█░░░
/// ```
///
/// ## `BOX_UNICODE[97]: U+2561`
/// `"╡"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ████░░░░
/// ░░░█░░░░
/// ████░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[98]: U+2562`
/// `"╢"`
///
/// ```text
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ███░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ```
///
/// ## `BOX_UNICODE[99]: U+2563`
/// `"╣"`
///
/// ```text
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ███░█░░░
/// ░░░░█░░░
/// ███░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ```
///
/// ## `BOX_UNICODE[100]: U+2564`
/// `"╤"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████████
/// ░░░░░░░░
/// ████████
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[101]: U+2565`
/// `"╥"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████████
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ```
///
/// ## `BOX_UNICODE[102]: U+2566`
/// `"╦"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████████
/// ░░░░░░░░
/// ███░████
/// ░░█░█░░░
/// ░░█░█░░░
/// ```
///
/// ## `BOX_UNICODE[103]: U+2567`
/// `"╧"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ████████
/// ░░░░░░░░
/// ████████
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[104]: U+2568`
/// `"╨"`
///
/// ```text
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ████████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[105]: U+2569`
/// `"╩"`
///
/// ```text
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ███░████
/// ░░░░░░░░
/// ████████
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[106]: U+256A`
/// `"╪"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ████████
/// ░░░█░░░░
/// ████████
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[107]: U+256B`
/// `"╫"`
///
/// ```text
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ████████
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ```
///
/// ## `BOX_UNICODE[108]: U+256C`
/// `"╬"`
///
/// ```text
/// ░░█░█░░░
/// ░░█░█░░░
/// ░░█░█░░░
/// ███░████
/// ░░░░░░░░
/// ███░████
/// ░░█░█░░░
/// ░░█░█░░░
/// ```
///
/// ## `BOX_UNICODE[109]: U+256D`
/// `"╭"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░███
/// ░░░░█░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[110]: U+256E`
/// `"╮"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ██░░░░░░
/// ░░█░░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[111]: U+256F`
/// `"╯"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░█░░░░░
/// ██░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[112]: U+2570`
/// `"╰"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░░█░░░
/// ░░░░░███
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[113]: U+2571`
/// `"╱"`
///
/// ```text
/// ░░░░░░░█
/// ░░░░░░█░
/// ░░░░░█░░
/// ░░░░█░░░
/// ░░░█░░░░
/// ░░█░░░░░
/// ░█░░░░░░
/// █░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[114]: U+2572`
/// `"╲"`
///
/// ```text
/// █░░░░░░░
/// ░█░░░░░░
/// ░░█░░░░░
/// ░░░█░░░░
/// ░░░░█░░░
/// ░░░░░█░░
/// ░░░░░░█░
/// ░░░░░░░█
/// ```
///
/// ## `BOX_UNICODE[115]: U+2573`
/// `"╳"`
///
/// ```text
/// █░░░░░░█
/// ░█░░░░█░
/// ░░█░░█░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░█░░█░░
/// ░█░░░░█░
/// █░░░░░░█
/// ```
///
/// ## `BOX_UNICODE[116]: U+2574`
/// `"╴"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[117]: U+2575`
/// `"╵"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[118]: U+2576`
/// `"╶"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░█████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[119]: U+2577`
/// `"╷"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
///
/// ## `BOX_UNICODE[120]: U+2578`
/// `"╸"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████░░░░
/// ████░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[121]: U+2579`
/// `"╹"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[122]: U+257A`
/// `"╺"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░█████
/// ░░░█████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[123]: U+257B`
/// `"╻"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[124]: U+257C`
/// `"╼"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░█████
/// ████████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[125]: U+257D`
/// `"╽"`
///
/// ```text
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ```
///
/// ## `BOX_UNICODE[126]: U+257E`
/// `"╾"`
///
/// ```text
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ████░░░░
/// ████████
/// ░░░░░░░░
/// ░░░░░░░░
/// ░░░░░░░░
/// ```
///
/// ## `BOX_UNICODE[127]: U+257F`
/// `"╿"`
///
/// ```text
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░██░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ░░░█░░░░
/// ```
pub const BOX_FONTS: BoxFonts = BoxFonts(BOX_UNICODE);

/// Strong-typed collection wrapper for [BOX_UNICODE](./constant.BOX_UNICODE.html).
pub struct BoxFonts([FontUnicode; 128]);

impl BoxFonts {
    pub fn new() -> Self {
        BoxFonts(BOX_UNICODE)
    }
}

impl fmt::Debug for BoxFonts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, stringify!(BOX_UNICODE))
    }
}

impl PartialEq for BoxFonts {
    fn eq(&self, other: &BoxFonts) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(true, |eq, (a, b)| eq && a == b)
    }
}

impl Default for BoxFonts {
    fn default() -> Self {
        BoxFonts::new()
    }
}

impl UnicodeFonts for BoxFonts {
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
    fn box_fonts_inner_array_is_sorted_by_unicode_key() {
        let mut sorted = BOX_UNICODE;
        sorted.sort_by_key(|f| f.char());
        for (idx, key) in sorted.iter().enumerate() {
            println!("{} {:04X}", idx, key.char() as u32);
            assert_eq!(BOX_UNICODE[idx].char(), key.char());
        }
    }

    #[test]
    fn box_set_implements_default_trait_with_method_new() {
        let box_set: BoxFonts = Default::default();
        assert_eq!(box_set, BoxFonts::new());
    }

    #[test]
    fn box_fonts_constant_is_equal_to_a_new_instance() {
        assert_eq!(BOX_FONTS, BoxFonts::new());
    }

    #[test]
    fn box_fonts_constant_wraps_basic_unicode_constant() {
        let box_font = BoxFonts::new();
        assert!(box_font.0.len() == BOX_UNICODE.len());
        for (idx, font) in box_font.0.iter().enumerate() {
            assert_eq!(font, &BOX_UNICODE[idx]);
        }
    }
}
