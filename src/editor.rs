pub struct Editor {}

use std::io;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

// error printing
fn die(e: std::io::Error) {
    panic!(e);
}

impl Editor {
    pub fn run(&self) {
        let stdin = io::stdin();
        let _stdout = io::stdout().into_raw_mode().unwrap();
        for key in stdin.keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        if c.is_control() {
                            println!("{:?}\r", c as u8);
                        } else {
                            println!("{:?} ({})\r", c as u8, c);
                        }
                    }
                    Key::Ctrl('q') => break,
                    _ => println!("{:?}", key),
                },
                Err(b) => {
                    die(b);
                }
            }
        }
    }

    pub fn default() -> Self {
        Self {}
    }
}
