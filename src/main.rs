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
    // DAli: The Hybrid Rust-Python Terminal Editor
    // Optimized for sub-10ms latency and extensible intelligence.
    
    let mut editor = Editor::new()?;
    editor.run()?;

    Ok(())
}
