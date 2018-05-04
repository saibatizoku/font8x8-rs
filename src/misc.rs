//! A miscellanous set of characters.
use super::{legacy::MISC_LEGACY, FontUtf16, Utf16Fonts};
use std::fmt;

pub const MISC_UTF16: [FontUtf16; 10] = [
    FontUtf16(0x20A7 as u16, MISC_LEGACY[0]),
    FontUtf16(0x0192 as u16, MISC_LEGACY[1]),
    FontUtf16(0x00AA as u16, MISC_LEGACY[2]),
    FontUtf16(0x00BA as u16, MISC_LEGACY[3]),
    FontUtf16(0x2310 as u16, MISC_LEGACY[4]),
    FontUtf16(0x2264 as u16, MISC_LEGACY[5]),
    FontUtf16(0x2265 as u16, MISC_LEGACY[6]),
    FontUtf16(0x0060 as u16, MISC_LEGACY[7]),
    FontUtf16(0x1EF2 as u16, MISC_LEGACY[8]),
    FontUtf16(0x1EF3 as u16, MISC_LEGACY[9]),
];

#[cfg(feature = "unicode")]
pub const MISC_UNICODE: [(u16, [u8; 8]); 10] = [
    (0x20A7, MISC[0]),
    (0x0192, MISC[1]),
    (0x00AA, MISC[2]),
    (0x00BA, MISC[3]),
    (0x2310, MISC[4]),
    (0x2264, MISC[5]),
    (0x2265, MISC[6]),
    (0x0060, MISC[7]),
    (0x1EF2, MISC[8]),
    (0x1EF3, MISC[9]),
];
