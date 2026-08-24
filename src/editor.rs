use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::terminal::{enable_raw_mode,disable_raw_mode};

pub struct Editor {
    to_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor {to_quit: false}
    }

    pub fn run(&mut self) {
        if let Err(error) = self.repl() {
            panic!("{error:#?}")
        }
    }
    fn repl(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode().unwrap();
        loop {
            if let Key(KeyEvent{code, modifiers, kind, state}) = read()? {
                println!("{code:?} {modifiers:?} {kind:?} {state:?}\r");
                if let Char('c') = code && modifiers == KeyModifiers::CONTROL {
                        self.to_quit = true;
                }
            }
            if self.to_quit {
                break;
            }
        }
        disable_raw_mode().unwrap();
        Ok(())
    }
}
