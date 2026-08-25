mod editor;
use editor::Editor;

fn main() {
    if let Err(err) = Editor::default().run() {
        panic!("{err:?}");
    }
}
