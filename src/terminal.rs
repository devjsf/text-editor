use crossterm::terminal::{enable_raw_mode,disable_raw_mode, size, Clear, ClearType};
use crossterm::cursor::MoveTo;
use crossterm::execute;
use std::io::stdout;
use std::io;

pub struct Terminal{}

impl Terminal {
    pub fn initialize() -> io::Result<()> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor(0,0)?;
        Ok(())
    }
    pub fn terminate() -> io::Result<()> {
        disable_raw_mode()?;
        Ok(())
    }    

    pub fn clear_screen() -> io::Result<()> {
        execute!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn move_cursor(x: u16, y: u16) -> io::Result<()> {
        execute!(stdout(), MoveTo(x,y))?;
        Ok(())
    }
    pub fn size() -> io::Result<(u16, u16)> {
        size()
    }
}
