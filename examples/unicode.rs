#[cfg(feature = "default")]
extern crate font8x8;

#[cfg(feature = "default")]
use font8x8::{UnicodeFonts, GREEK_FONTS};

// This example will print to the screen the same glyph
// if it is found by its `char` within GREEK_FONTS.
#[cfg(feature = "default")]
fn main() {
    let unicode = '\u{03A9}';
    println!("This should be displayed below: {:?}", unicode);
    if let Some(glyph) = GREEK_FONTS.get(unicode) {
        for x in &glyph {
            for bit in 0..8 {
                match *x & 1 << bit {
                    0 => print!(" "),
                    _ => print!("█"),
                }
            }
            println!()
        }
    }
}

#[cfg(not(feature = "default"))]
fn main() {}
