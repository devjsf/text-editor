use crossterm::event::{read, Event::{self, Key}, KeyCode::{self, Char}, KeyEvent, KeyEventKind, KeyModifiers};
use std::io;
use super::terminal::{Terminal, Size, Position};

pub enum Mode {
   Normal,
   Insert,
}
 
pub struct Location {
    x: usize,
    y: usize,
}
pub struct Editor {
    to_quit: bool,
    location: Location,
    mode: Mode,
}

impl Editor {
    pub const fn default() -> Self {
        Self {to_quit: false, location: Location{x:0, y:0}, mode: Mode::Normal}
    }

    pub fn run(&mut self) -> io::Result<()> {
        Terminal::initialize()?;
        self.repl()?;
        Terminal::terminate()?;
        Ok(())
    }

    pub fn refresh_screen(&self) -> io::Result<()> {
        Terminal::hide_cursor()?;
        if self.to_quit {
            Terminal::clear_screen()?;
        }else {
            Self::draw_row()?;
        }
        Terminal::show_cursor()?;
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
            Terminal::move_cursor(Position{col:0, row:0})?;
            let event = read()?;
            self.read_event(&event);
            if self.to_quit {
                break;
            }
        }
        Ok(())
    }
    
    fn move_caret(&mut self, key_code: KeyCode) -> io::Result<()> {
        let Location {mut x, mut y} = self.location;
        let Size {height, width} = Terminal::size()?;
        let max_x = usize::from(width.min(80).saturating_sub(1));
        let max_y = usize::from(height.saturating_sub(1));
        match key_code {
            KeyCode::Char('k') => y = y.saturating_sub(1),
            KeyCode::Char('j') => y = y.saturating_add(1).min(max_y),
            KeyCode::Char('h') => x = x.saturating_sub(1),
            KeyCode::Char('l') => x = x.saturating_add(1).min(max_x),
            _ => return Ok(())
        }
    self.location = Location{x,y};
    Ok(())
    }

    fn read_event(&mut self, event: &Event) -> io::Result<()> {
        if let Key(KeyEvent{code, modifiers, kind:KeyEventKind::Press, ..}) = event {
            match code {
                Char('c') if *modifiers == KeyModifiers::CONTROL => {
                    self.to_quit = true;
                }
                _ => {
                    match self.mode {
                        Mode::Normal => {
                            match code {
                                Char('k')
                                    | Char('j')
                                    | Char('h')
                                    | Char('l') => {
                                        self.move_caret(*code)?;
                                    }
                                Char('i') => {self.mode = Mode::Insert;
                                }
                                _ => ()
                            }
                        }
                        Mode::Insert => {
                            match code {
                                KeyCode::Esc => {
                                    self.mode = Mode::Normal;
                                }
                                _ => ()
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
