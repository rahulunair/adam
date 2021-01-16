use std::io;
use std::io::Read;

use termion::raw::IntoRawMode;

// convert character to a control byte
fn to_ctr_byte(c: char) -> u8 {
    let b = c as u8;
    b & 0b0001_1111
}

// error printing
fn die(e: std::io::Error) {
    panic!(e);
}

fn main() {
    let stdin = io::stdin();
    let _stdout = io::stdout().into_raw_mode().unwrap();
    for b in stdin.bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    println!("{:?}\r", b);
                } else {
                    println!("{:?} ({})\r", b, c);
                }
                if b == to_ctr_byte('q') {
                    break;
                }
            }
            Err(b) => {
                die(b);
            }
        }
    }
}
