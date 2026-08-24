use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{enable_raw_mode,disable_raw_mode};

pub struct Editor {

}
impl Editor {
    pub fn run() {
        enable_raw_mode().unwrap();
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{event:?}\r");
                    if let Char(c) = event.code && c == 'q' {
                            break;
                    }
                },
                Err(error) => println!("Error {error}"),
                _ => (),
            }
        }
        disable_raw_mode().unwrap();
    }
}
