use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io;
use super::terminal::{Terminal, Size, Position};

pub struct Editor {
    to_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self {to_quit: false}
    }

    pub fn run(&mut self) -> io::Result<()> {
        Terminal::initialize()?;
        self.repl()?;
        Terminal::terminate()?;
        Ok(())
    }


    pub fn refresh_screen(&self) -> io::Result<()> {
        if self.to_quit {
            Terminal::clear_screen()?;
        }else {
            Terminal::move_cursor(Position{x:0, y:0})?;
            Terminal::clear_screen()?;
            Self::draw_row()?;
        }
        Ok(())
    }

    pub fn draw_row() -> io::Result<()> {
        let Size{height,..} = Terminal::size()?;
        for row in 0..height {
            let row_display = row + 1;
            print!("{row_display}");
            if row + 1 < height {
                print!("\r\n");
            }
        Terminal::flush()?;
        }
        Ok(())
    }
    
    fn repl(&mut self) -> io::Result<()> {
        loop {
            self.refresh_screen()?;
            Terminal::move_cursor(Position{x:0, y:0})?;
            let event = read()?;
            self.read_event(&event);
            if self.to_quit {
                break;
            }
        }
        Ok(())
    }
    fn read_event(&mut self, event: &Event) {
        if let Key(KeyEvent{code, modifiers,..}) = event &&
            let Char('c') = code && *modifiers == KeyModifiers::CONTROL {
                self.to_quit = true;
        }
    }
}
