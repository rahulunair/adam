use std::io::{self, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

// error printing
fn die(e: std::io::Error) {
    print!("{}", termion::clear::All);
    panic!(e);
}

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        let _stdout = io::stdout().into_raw_mode().unwrap();
        loop {
            // process_keypress is wrapped in a Result
            if let Err(error) = self.referesh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    pub fn default() -> Self {
        Self { should_quit: false }
    }

    fn referesh_screen(&self) -> Result<(), std::io::Error> {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        if self.should_quit {
            println!("Good bye!\r");
        } else {
            self.draw_rows();
            print!("{}", termion::cursor::Goto(1, 1));
        }
        io::stdout().flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        // wait for keypress to read key
        // read_key is wrapped in a Result
        // either a valid key or an Error
        let processed_key = read_key()?;
        match processed_key {
            // if key is ctrl + q, end program
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        // if everything goes well without error
        // everything is Ok, return nothing - ()
        Ok(())
    }
    fn draw_rows(&self) {
        for _ in 0..24 {
            println!("~\r");
        }
    }
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        // below expression is wrapped in an Option which can be
        // None or a value
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}
