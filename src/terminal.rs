use crossterm::terminal::{enable_raw_mode,disable_raw_mode, size, Clear, ClearType};
use crossterm::caret::{Hide, Show, MoveTo};
use crossterm::{execute, queue};
use crossterm::style::Print;
use std::io::{self, stdout, Write};

pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal{}

impl Terminal {
    pub fn initialize() -> io::Result<()> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_caret(Position{x:0, y:0})?;
        Ok(())
    }
    pub fn terminate() -> io::Result<()> {
        disable_raw_mode()?;
        Self::move_caret(Position{x:0, y:0})?;
        Ok(())
    }    

    pub fn flush() -> io::Result<()> {
        io::stdout().flush()?;
        Ok(())
    }

    pub fn clear_screen() -> io::Result<()> {
        execute!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> io::Result<()> {
        execute!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn move_caret(position: Position) -> io::Result<()> {
        execute!(stdout(), MoveTo(position.x, position.y))?;
        Ok(())
    }

    pub fn show_caret() -> io::Result<()> {
        queue!(stdout(), Show)?;
        Ok(())
    }

    pub fn hide_caret() -> io::Result<()> {
        queue!(stdout(), Hide)?;
        Ok(())
    }
    
    pub fn print(string: &str) -> io::Result<()> {
        queue!(stdout(), Print(string))?;
        Ok(())
    }

    pub fn size() -> io::Result<Size> {
        let (width, height) = size()?;
        Ok(Size{width, height})
    }
}
