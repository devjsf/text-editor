mod editor;
mod terminal;
use editor::Editor;
use terminal::Terminal;

fn main() {
    if let Err(err) = Editor::default().run() {
        panic!("{err:?}");
    }
}
