use termion::event::Key;

use crate::Terminal;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position { x: 0, y: 0 },
        }
    }
}

impl Editor {
    pub fn run(&mut self) {
        // for key in io::stdin().keys() {
        //     match key {
        //         Ok(key) => match key {
        //             Key::Char(c) => {
        //                 if c.is_control() {
        //                     println!("{:3?} ({:#08b}) \r", c as u8, c as u8);
        //                 } else {
        //                     println!("{:3?} ({:#08b}) ({})\r", c as u8, c as u8,c);
        //                 }
        //             }
        //             Key::Ctrl('q') => break,
        //             _ => println!("{:?}\r", key),
        //         }
        //         Err(err) => die(err),
        //     }
        // }
        loop {
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

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position { x: 0, y: 0 });

        if self.should_quit {
            Terminal::clear_screen();
            println!("bye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }

        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("editor -- version {} (quit: Ctrl-q)", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
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

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up | Key::Down | Key::Left | Key::Right => self.move_cursor(pressed_key),
            // Do not support Page{Up,Down}, End, Home
            _ => (),
        }
        Ok(())
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;

        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1)
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1)
                }
            }
            _ => (),
        }
        self.cursor_position = Position { x, y };
    }
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!(e)
}
