#[cfg(feature = "default")]
extern crate font8x8;

#[cfg(feature = "default")]
use font8x8::legacy::GREEK_LEGACY;

// This example will print to the screen the first item on the GREEK_LEGACY array.
#[cfg(feature = "default")]
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

#[cfg(not(feature = "default"))]
fn main() {}
