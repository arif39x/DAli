# DAli Editor

A high-performance, Kilo-inspired terminal text editor with a Rust-Python hybrid intelligence engine.

## Command Reference

### Editor Keys
- **Ctrl + Q**: Quit
- **Ctrl + F**: Search
- **Ctrl + P**: Fuzzy Search
- **Ctrl + S**: Save
- **Arrow Keys**: Move Cursor
- **:**: Enter Command Mode

### DAli-Term Commands (After pressing `:`)
- **h**: Toggle Help Overlay
- **s**: Save File
- **q**: Quit Editor
- **build**: Run Cargo Build / Run
- **Esc**: Close Command Mode
- **Enter**: Execute & Close

## Architecture
- **Core**: Rust (crossterm + custom Gap Buffer)
- **Intelligence**: Python Bridge (PyO3)
- **Modularity**: All files strictly < 121 lines.
