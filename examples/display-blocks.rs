#[cfg(feature = "default")]
extern crate font8x8;

#[cfg(feature = "default")]
use font8x8::*; //{BLOCK_FONTS, UnicodeFonts};

#[cfg(feature = "default")]
fn main() {
    // Use the convenience `print_set` method to view all the
    // Block Fonts on your screen.
    BLOCK_FONTS.print_set();
}

#[cfg(not(feature = "default"))]
fn main() {}
