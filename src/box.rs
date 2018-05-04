//!  Box Drawing. `U+2500 - U+257F`
use super::{legacy::BOX_LEGACY, FontUtf16, Utf16Fonts};
use std::fmt;

/// `BOX_UTF16` description and ASCII-art representation.
///
/// ## `BOX_UTF16[0]`: `U+2500` `"─"`
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
/// ## `BOX_UTF16[1]`: `U+2501` `"━"`
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
/// ## `BOX_UTF16[2]`: `U+2502` `"│"`
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
/// ## `BOX_UTF16[3]`: `U+2503` `"┃"`
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
/// ## `BOX_UTF16[4]`: `U+2504` `"┄"`
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
/// ## `BOX_UTF16[5]`: `U+2505` `"┅"`
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
/// ## `BOX_UTF16[6]`: `U+2506` `"┆"`
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
/// ## `BOX_UTF16[7]`: `U+2507` `"┇"`
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
/// ## `BOX_UTF16[8]`: `U+2508` `"┈"`
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
/// ## `BOX_UTF16[9]`: `U+2509` `"┉"`
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
/// ## `BOX_UTF16[10]`: `U+250A` `"┊"`
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
/// ## `BOX_UTF16[11]`: `U+250B` `"┋"`
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
/// ## `BOX_UTF16[12]`: `U+250C` `"┌"`
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
/// ## `BOX_UTF16[13]`: `U+250D` `"┍"`
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
/// ## `BOX_UTF16[14]`: `U+250E` `"┎"`
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
/// ## `BOX_UTF16[15]`: `U+250F` `"┏"`
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
/// ## `BOX_UTF16[16]`: `U+2510` `"┐"`
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
/// ## `BOX_UTF16[17]`: `U+2511` `"┑"`
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
/// ## `BOX_UTF16[18]`: `U+2512` `"┒"`
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
/// ## `BOX_UTF16[19]`: `U+2513` `"┓"`
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
/// ## `BOX_UTF16[20]`: `U+2514` `"└"`
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
/// ## `BOX_UTF16[21]`: `U+2515` `"┕"`
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
/// ## `BOX_UTF16[22]`: `U+2516` `"┖"`
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
/// ## `BOX_UTF16[23]`: `U+2517` `"┗"`
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
/// ## `BOX_UTF16[24]`: `U+2518` `"┘"`
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
/// ## `BOX_UTF16[25]`: `U+2519` `"┙"`
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
/// ## `BOX_UTF16[26]`: `U+251A` `"┚"`
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
/// ## `BOX_UTF16[27]`: `U+251B` `"┛"`
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
/// ## `BOX_UTF16[28]`: `U+251C` `"├"`
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
/// ## `BOX_UTF16[29]`: `U+251D` `"┝"`
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
/// ## `BOX_UTF16[30]`: `U+251E` `"┞"`
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
/// ## `BOX_UTF16[31]`: `U+251F` `"┟"`
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
/// ## `BOX_UTF16[32]`: `U+2520` `"┠"`
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
/// ## `BOX_UTF16[33]`: `U+2521` `"┡"`
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
/// ## `BOX_UTF16[34]`: `U+2522` `"┢"`
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
/// ## `BOX_UTF16[35]`: `U+2523` `"┣"`
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
/// ## `BOX_UTF16[36]`: `U+2524` `"┤"`
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
/// ## `BOX_UTF16[37]`: `U+2525` `"┥"`
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
/// ## `BOX_UTF16[38]`: `U+2526` `"┦"`
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
/// ## `BOX_UTF16[39]`: `U+2527` `"┧"`
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
/// ## `BOX_UTF16[40]`: `U+2528` `"┨"`
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
/// ## `BOX_UTF16[41]`: `U+2529` `"┩"`
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
/// ## `BOX_UTF16[42]`: `U+252A` `"┪"`
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
/// ## `BOX_UTF16[43]`: `U+252B` `"┫"`
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
/// ## `BOX_UTF16[44]`: `U+252C` `"┬"`
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
/// ## `BOX_UTF16[45]`: `U+252D` `"┭"`
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
/// ## `BOX_UTF16[46]`: `U+252E` `"┮"`
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
/// ## `BOX_UTF16[47]`: `U+252F` `"┯"`
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
/// ## `BOX_UTF16[48]`: `U+2530` `"┰"`
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
/// ## `BOX_UTF16[49]`: `U+2531` `"┱"`
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
/// ## `BOX_UTF16[50]`: `U+2532` `"┲"`
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
/// ## `BOX_UTF16[51]`: `U+2533` `"┳"`
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
/// ## `BOX_UTF16[52]`: `U+2534` `"┴"`
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
/// ## `BOX_UTF16[53]`: `U+2535` `"┵"`
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
/// ## `BOX_UTF16[54]`: `U+2536` `"┶"`
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
/// ## `BOX_UTF16[55]`: `U+2537` `"┷"`
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
/// ## `BOX_UTF16[56]`: `U+2538` `"┸"`
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
/// ## `BOX_UTF16[57]`: `U+2539` `"┹"`
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
/// ## `BOX_UTF16[58]`: `U+253A` `"┺"`
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
/// ## `BOX_UTF16[59]`: `U+253B` `"┻"`
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
/// ## `BOX_UTF16[60]`: `U+253C` `"┼"`
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
/// ## `BOX_UTF16[61]`: `U+253D` `"┽"`
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
/// ## `BOX_UTF16[62]`: `U+253E` `"┾"`
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
/// ## `BOX_UTF16[63]`: `U+253F` `"┿"`
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
/// ## `BOX_UTF16[64]`: `U+2540` `"╀"`
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
/// ## `BOX_UTF16[65]`: `U+2541` `"╁"`
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
/// ## `BOX_UTF16[66]`: `U+2542` `"╂"`
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
/// ## `BOX_UTF16[67]`: `U+2543` `"╃"`
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
/// ## `BOX_UTF16[68]`: `U+2544` `"╄"`
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
/// ## `BOX_UTF16[69]`: `U+2545` `"╅"`
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
/// ## `BOX_UTF16[70]`: `U+2546` `"╆"`
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
/// ## `BOX_UTF16[71]`: `U+2547` `"╇"`
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
/// ## `BOX_UTF16[72]`: `U+254B` `"╋"`
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
/// ## `BOX_UTF16[73]`: `U+254A` `"╊"`
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
/// ## `BOX_UTF16[74]`: `U+2549` `"╉"`
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
/// ## `BOX_UTF16[75]`: `U+254B` `"╋"`
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
/// ## `BOX_UTF16[76]`: `U+254C` `"╌"`
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
/// ## `BOX_UTF16[77]`: `U+254D` `"╍"`
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
/// ## `BOX_UTF16[78]`: `U+254E` `"╎"`
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
/// ## `BOX_UTF16[79]`: `U+254F` `"╏"`
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
/// ## `BOX_UTF16[80]`: `U+2550` `"═"`
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
/// ## `BOX_UTF16[81]`: `U+2551` `"║"`
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
/// ## `BOX_UTF16[82]`: `U+2552` `"╒"`
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
/// ## `BOX_UTF16[83]`: `U+2553` `"╓"`
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
/// ## `BOX_UTF16[84]`: `U+2554` `"╔"`
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
/// ## `BOX_UTF16[85]`: `U+2555` `"╕"`
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
/// ## `BOX_UTF16[86]`: `U+2556` `"╖"`
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
/// ## `BOX_UTF16[87]`: `U+2557` `"╗"`
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
/// ## `BOX_UTF16[88]`: `U+2558` `"╘"`
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
/// ## `BOX_UTF16[89]`: `U+2559` `"╙"`
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
/// ## `BOX_UTF16[90]`: `U+255A` `"╚"`
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
/// ## `BOX_UTF16[91]`: `U+255B` `"╛"`
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
/// ## `BOX_UTF16[92]`: `U+255C` `"╜"`
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
/// ## `BOX_UTF16[93]`: `U+255D` `"╝"`
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
/// ## `BOX_UTF16[94]`: `U+255E` `"╞"`
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
/// ## `BOX_UTF16[95]`: `U+255F` `"╟"`
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
/// ## `BOX_UTF16[96]`: `U+2560` `"╠"`
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
/// ## `BOX_UTF16[97]`: `U+2561` `"╡"`
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
/// ## `BOX_UTF16[98]`: `U+2562` `"╢"`
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
/// ## `BOX_UTF16[99]`: `U+2563` `"╣"`
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
/// ## `BOX_UTF16[100]`: `U+2564` `"╤"`
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
/// ## `BOX_UTF16[101]`: `U+2565` `"╥"`
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
/// ## `BOX_UTF16[102]`: `U+2566` `"╦"`
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
/// ## `BOX_UTF16[103]`: `U+2567` `"╧"`
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
/// ## `BOX_UTF16[104]`: `U+2568` `"╨"`
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
/// ## `BOX_UTF16[105]`: `U+2569` `"╩"`
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
/// ## `BOX_UTF16[106]`: `U+256A` `"╪"`
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
/// ## `BOX_UTF16[107]`: `U+256B` `"╫"`
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
/// ## `BOX_UTF16[108]`: `U+256C` `"╬"`
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
/// ## `BOX_UTF16[109]`: `U+256D` `"╭"`
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
/// ## `BOX_UTF16[110]`: `U+256E` `"╮"`
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
/// ## `BOX_UTF16[111]`: `U+256F` `"╯"`
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
/// ## `BOX_UTF16[112]`: `U+2570` `"╰"`
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
/// ## `BOX_UTF16[113]`: `U+2571` `"╱"`
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
/// ## `BOX_UTF16[114]`: `U+2572` `"╲"`
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
/// ## `BOX_UTF16[115]`: `U+2573` `"╳"`
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
/// ## `BOX_UTF16[116]`: `U+2574` `"╴"`
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
/// ## `BOX_UTF16[117]`: `U+2575` `"╵"`
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
/// ## `BOX_UTF16[118]`: `U+2576` `"╶"`
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
/// ## `BOX_UTF16[119]`: `U+2577` `"╷"`
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
/// ## `BOX_UTF16[120]`: `U+2578` `"╸"`
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
/// ## `BOX_UTF16[121]`: `U+2579` `"╹"`
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
/// ## `BOX_UTF16[122]`: `U+257A` `"╺"`
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
/// ## `BOX_UTF16[123]`: `U+257B` `"╻"`
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
/// ## `BOX_UTF16[124]`: `U+257C` `"╼"`
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
/// ## `BOX_UTF16[125]`: `U+257D` `"╽"`
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
/// ## `BOX_UTF16[126]`: `U+257E` `"╾"`
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
/// ## `BOX_UTF16[127]`: `U+257F` `"╿"`
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
pub const BOX_UTF16: [FontUtf16; 128] = [
    FontUtf16(0x2500 as u16, BOX_LEGACY[0]),
    FontUtf16(0x2501 as u16, BOX_LEGACY[1]),
    FontUtf16(0x2502 as u16, BOX_LEGACY[2]),
    FontUtf16(0x2503 as u16, BOX_LEGACY[3]),
    FontUtf16(0x2504 as u16, BOX_LEGACY[4]),
    FontUtf16(0x2505 as u16, BOX_LEGACY[5]),
    FontUtf16(0x2506 as u16, BOX_LEGACY[6]),
    FontUtf16(0x2507 as u16, BOX_LEGACY[7]),
    FontUtf16(0x2508 as u16, BOX_LEGACY[8]),
    FontUtf16(0x2509 as u16, BOX_LEGACY[9]),
    FontUtf16(0x250A as u16, BOX_LEGACY[10]),
    FontUtf16(0x250B as u16, BOX_LEGACY[11]),
    FontUtf16(0x250C as u16, BOX_LEGACY[12]),
    FontUtf16(0x250D as u16, BOX_LEGACY[13]),
    FontUtf16(0x250E as u16, BOX_LEGACY[14]),
    FontUtf16(0x250F as u16, BOX_LEGACY[15]),
    FontUtf16(0x2510 as u16, BOX_LEGACY[16]),
    FontUtf16(0x2511 as u16, BOX_LEGACY[17]),
    FontUtf16(0x2512 as u16, BOX_LEGACY[18]),
    FontUtf16(0x2513 as u16, BOX_LEGACY[19]),
    FontUtf16(0x2514 as u16, BOX_LEGACY[20]),
    FontUtf16(0x2515 as u16, BOX_LEGACY[21]),
    FontUtf16(0x2516 as u16, BOX_LEGACY[22]),
    FontUtf16(0x2517 as u16, BOX_LEGACY[23]),
    FontUtf16(0x2518 as u16, BOX_LEGACY[24]),
    FontUtf16(0x2519 as u16, BOX_LEGACY[25]),
    FontUtf16(0x251A as u16, BOX_LEGACY[26]),
    FontUtf16(0x251B as u16, BOX_LEGACY[27]),
    FontUtf16(0x251C as u16, BOX_LEGACY[28]),
    FontUtf16(0x251D as u16, BOX_LEGACY[29]),
    FontUtf16(0x251E as u16, BOX_LEGACY[30]),
    FontUtf16(0x251F as u16, BOX_LEGACY[31]),
    FontUtf16(0x2520 as u16, BOX_LEGACY[32]),
    FontUtf16(0x2521 as u16, BOX_LEGACY[33]),
    FontUtf16(0x2522 as u16, BOX_LEGACY[34]),
    FontUtf16(0x2523 as u16, BOX_LEGACY[35]),
    FontUtf16(0x2524 as u16, BOX_LEGACY[36]),
    FontUtf16(0x2525 as u16, BOX_LEGACY[37]),
    FontUtf16(0x2526 as u16, BOX_LEGACY[38]),
    FontUtf16(0x2527 as u16, BOX_LEGACY[39]),
    FontUtf16(0x2528 as u16, BOX_LEGACY[40]),
    FontUtf16(0x2529 as u16, BOX_LEGACY[41]),
    FontUtf16(0x252A as u16, BOX_LEGACY[42]),
    FontUtf16(0x252B as u16, BOX_LEGACY[43]),
    FontUtf16(0x252C as u16, BOX_LEGACY[44]),
    FontUtf16(0x252D as u16, BOX_LEGACY[45]),
    FontUtf16(0x252E as u16, BOX_LEGACY[46]),
    FontUtf16(0x252F as u16, BOX_LEGACY[47]),
    FontUtf16(0x2530 as u16, BOX_LEGACY[48]),
    FontUtf16(0x2531 as u16, BOX_LEGACY[49]),
    FontUtf16(0x2532 as u16, BOX_LEGACY[50]),
    FontUtf16(0x2533 as u16, BOX_LEGACY[51]),
    FontUtf16(0x2534 as u16, BOX_LEGACY[52]),
    FontUtf16(0x2535 as u16, BOX_LEGACY[53]),
    FontUtf16(0x2536 as u16, BOX_LEGACY[54]),
    FontUtf16(0x2537 as u16, BOX_LEGACY[55]),
    FontUtf16(0x2538 as u16, BOX_LEGACY[56]),
    FontUtf16(0x2539 as u16, BOX_LEGACY[57]),
    FontUtf16(0x253A as u16, BOX_LEGACY[58]),
    FontUtf16(0x253B as u16, BOX_LEGACY[59]),
    FontUtf16(0x253C as u16, BOX_LEGACY[60]),
    FontUtf16(0x253D as u16, BOX_LEGACY[61]),
    FontUtf16(0x253E as u16, BOX_LEGACY[62]),
    FontUtf16(0x253F as u16, BOX_LEGACY[63]),
    FontUtf16(0x2540 as u16, BOX_LEGACY[64]),
    FontUtf16(0x2541 as u16, BOX_LEGACY[65]),
    FontUtf16(0x2542 as u16, BOX_LEGACY[66]),
    FontUtf16(0x2543 as u16, BOX_LEGACY[67]),
    FontUtf16(0x2544 as u16, BOX_LEGACY[68]),
    FontUtf16(0x2545 as u16, BOX_LEGACY[69]),
    FontUtf16(0x2546 as u16, BOX_LEGACY[70]),
    FontUtf16(0x2547 as u16, BOX_LEGACY[71]),
    FontUtf16(0x254B as u16, BOX_LEGACY[72]),
    FontUtf16(0x254A as u16, BOX_LEGACY[73]),
    FontUtf16(0x2549 as u16, BOX_LEGACY[74]),
    FontUtf16(0x254B as u16, BOX_LEGACY[75]),
    FontUtf16(0x254C as u16, BOX_LEGACY[76]),
    FontUtf16(0x254D as u16, BOX_LEGACY[77]),
    FontUtf16(0x254E as u16, BOX_LEGACY[78]),
    FontUtf16(0x254F as u16, BOX_LEGACY[79]),
    FontUtf16(0x2550 as u16, BOX_LEGACY[80]),
    FontUtf16(0x2551 as u16, BOX_LEGACY[81]),
    FontUtf16(0x2552 as u16, BOX_LEGACY[82]),
    FontUtf16(0x2553 as u16, BOX_LEGACY[83]),
    FontUtf16(0x2554 as u16, BOX_LEGACY[84]),
    FontUtf16(0x2555 as u16, BOX_LEGACY[85]),
    FontUtf16(0x2556 as u16, BOX_LEGACY[86]),
    FontUtf16(0x2557 as u16, BOX_LEGACY[87]),
    FontUtf16(0x2558 as u16, BOX_LEGACY[88]),
    FontUtf16(0x2559 as u16, BOX_LEGACY[89]),
    FontUtf16(0x255A as u16, BOX_LEGACY[90]),
    FontUtf16(0x255B as u16, BOX_LEGACY[91]),
    FontUtf16(0x255C as u16, BOX_LEGACY[92]),
    FontUtf16(0x255D as u16, BOX_LEGACY[93]),
    FontUtf16(0x255E as u16, BOX_LEGACY[94]),
    FontUtf16(0x255F as u16, BOX_LEGACY[95]),
    FontUtf16(0x2560 as u16, BOX_LEGACY[96]),
    FontUtf16(0x2561 as u16, BOX_LEGACY[97]),
    FontUtf16(0x2562 as u16, BOX_LEGACY[98]),
    FontUtf16(0x2563 as u16, BOX_LEGACY[99]),
    FontUtf16(0x2564 as u16, BOX_LEGACY[100]),
    FontUtf16(0x2565 as u16, BOX_LEGACY[101]),
    FontUtf16(0x2566 as u16, BOX_LEGACY[102]),
    FontUtf16(0x2567 as u16, BOX_LEGACY[103]),
    FontUtf16(0x2568 as u16, BOX_LEGACY[104]),
    FontUtf16(0x2569 as u16, BOX_LEGACY[105]),
    FontUtf16(0x256A as u16, BOX_LEGACY[106]),
    FontUtf16(0x256B as u16, BOX_LEGACY[107]),
    FontUtf16(0x256C as u16, BOX_LEGACY[108]),
    FontUtf16(0x256D as u16, BOX_LEGACY[109]),
    FontUtf16(0x256E as u16, BOX_LEGACY[110]),
    FontUtf16(0x256F as u16, BOX_LEGACY[111]),
    FontUtf16(0x2570 as u16, BOX_LEGACY[112]),
    FontUtf16(0x2571 as u16, BOX_LEGACY[113]),
    FontUtf16(0x2572 as u16, BOX_LEGACY[114]),
    FontUtf16(0x2573 as u16, BOX_LEGACY[115]),
    FontUtf16(0x2574 as u16, BOX_LEGACY[116]),
    FontUtf16(0x2575 as u16, BOX_LEGACY[117]),
    FontUtf16(0x2576 as u16, BOX_LEGACY[118]),
    FontUtf16(0x2577 as u16, BOX_LEGACY[119]),
    FontUtf16(0x2578 as u16, BOX_LEGACY[120]),
    FontUtf16(0x2579 as u16, BOX_LEGACY[121]),
    FontUtf16(0x257A as u16, BOX_LEGACY[122]),
    FontUtf16(0x257B as u16, BOX_LEGACY[123]),
    FontUtf16(0x257C as u16, BOX_LEGACY[124]),
    FontUtf16(0x257D as u16, BOX_LEGACY[125]),
    FontUtf16(0x257E as u16, BOX_LEGACY[126]),
    FontUtf16(0x257F as u16, BOX_LEGACY[127]),
];
