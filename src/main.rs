#![warn(clippy::all, clippy::pedantic)]
mod editor;

use editor::Editor;
fn main() {
    let ed = Editor::default();
    ed.run();
}
