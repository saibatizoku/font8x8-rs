extern crate font8x8;

use font8x8::{GREEK_FONTS, UnicodeFonts};

// This example will print to the screen the same glyph
// if it is found by its `char` within GREEK_FONTS.
fn main() {
    if let Some(glyph) = GREEK_FONTS.get('ΐ') {
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
