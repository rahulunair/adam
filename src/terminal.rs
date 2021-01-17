// terminal data structures sit here
use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use termion::terminal_size;

pub struct Size {
    pub height: u16,
    pub width: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = terminal_size()?;
        Ok(Self {
            size: Size {
                height: size.1,
                width: size.0,
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }
    pub fn cursor_position(x: u16, y: u16) {
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
        print!("{}", termion::cursor::Goto(x, y));
    }
    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }
    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
    pub fn hide_cursor() {
        print!("{}", termion::cursor::Hide);
    }
    pub fn show_cursor() {
        print!("{}", termion::cursor::Show);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }
}
