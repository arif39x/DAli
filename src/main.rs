mod terminal;
mod buffer;
mod events;
mod bridge;
mod highlight;
mod fuzzy;
mod editor;

use editor::Editor;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    
    let mut editor = Editor::new()?;
    editor.run()?;

    Ok(())
}
