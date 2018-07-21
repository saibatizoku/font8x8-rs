extern crate font8x8;

use font8x8::legacy::GREEK_LEGACY;

// This example will print to the screen the first item on the GREEK_LEGACY array.
fn main() {
    for x in &GREEK_LEGACY[0] {
        for bit in 0..8 {
            match *x & 1 << bit {
                0 => print!(" "),
                _ => print!("█"),
            }
        }
        println!()
    }
}
