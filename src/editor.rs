use crate::Terminal;

use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");
// error printing
fn die(e: std::io::Error) {
    print!("{}", termion::clear::All);
    panic!(e);
}

pub struct Position {
    pub x: usize,
    pub y: usize,
}
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
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
            cursor_position: Position { x: 0, y: 0 },
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor();
        Terminal::cursor_position(&Position { x: 0, y: 0 });
        if self.should_quit {
            Terminal::clear_screen();
            println!("Good bye!\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&Position { x: 0, y: 0 });
        }
        Terminal::show_cursor();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        // wait for keypress to read key
        // read_key is wrapped in a Result
        // either a valid key or an Error
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            // if key is ctrl + q, end program
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        // if everything goes well without error
        // everything is Ok, return nothing - ()
        Ok(())
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut y, mut x } = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,
            Key::Left => x = x.saturating_sub(1),
            _ => (),
        }
        self.cursor_position = Position { x, y }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Adam editor -- version {}\r", VERSION);
        let terminal_width = self.terminal.size().width as usize;
        let message_len = welcome_message.len();
        let padding = terminal_width.saturating_sub(message_len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(terminal_width);
        println!("{}\r", welcome_message);
    }
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }
}
