use crate::Terminal;

use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");
// error printing
fn die(e: std::io::Error) {
    print!("{}", termion::clear::All);
    panic!(e);
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            // process_keypress is wrapped in a Result
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
            if self.should_quit {
                break;
            }
        }
    }

    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().unwrap(),
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor();
        if self.should_quit {
            Terminal::clear_screen();
            println!("Good bye!\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }
        Terminal::show_cursor();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        // wait for keypress to read key
        // read_key is wrapped in a Result
        // either a valid key or an Error
        let processed_key = Terminal::read_key()?;
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
        for _ in 0..self.terminal.size().height - 1 {
            Terminal::clear_current_line();
            println!("~\r");
        }
    }
}
