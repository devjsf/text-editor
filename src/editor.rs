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
        Terminal::hide_caret()?;
        if self.to_quit {
            Terminal::clear_screen()?;
        }else {
            Terminal::move_caret(Position{x:0, y:0})?;
            Self::draw_row()?;
        }
        Terminal::show_caret()?;
        Ok(())
    }

    pub fn draw_row() -> io::Result<()> {
        let Size{height,..} = Terminal::size()?;
        for row in 0..height {
            Terminal::clear_line()?;
            let row_display: String = (row + 1).to_string();
            Terminal::print(&row_display)?;
            if row + 1 < height {
                Terminal::print("\r\n")?;
            }
        Terminal::flush()?;
        }
        Ok(())
    }
    
    fn repl(&mut self) -> io::Result<()> {
        loop {
            self.refresh_screen()?;
            Terminal::move_caret(Position{x:0, y:0})?;
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
