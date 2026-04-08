# DAli Editor

A high-performance, memory-safe, deterministic terminal text editor built in pure Rust.

## Principles
- **Speed**: Minimal latency, GapBuffer-backed text engine.
- **Safety**: Built on Rust's strong safety guarantees.
- **Privacy**: **STRICT NO-AI POLICY**. All operations are local and deterministic.
- **Polish**: Professional-grade developer ergonomics.

## Configuration
DAli is highly configurable via TOML. Create your configuration at `~/.config/dali/dali.toml`:

```toml
[keybindings]
quit = "ctrl-q"
save = "ctrl-s"
find = "ctrl-f"
command = "ctrl-e"
fuzzy = "ctrl-p"

[theme]
status_bar_bg = [30, 30, 60]
status_bar_fg = [255, 255, 255]
gutter_fg = [100, 100, 120]
selection_bg = [80, 80, 150]

[editor]
tab_size = 4
line_numbers = true
```

## Features
- **Deterministic Indentation**: Context-aware auto-indentation in pure Rust.
- **Native Snippets**: High-speed snippet expansion via `Tab`.
- **Fuzzy Finder**: Blazing fast file search with fuzzy matching.
- **Multi-Window**: Professional split-view support.
- **Syntax Highlighting**: Real-time language-aware coloring.
- **Git Integration**: Background branch and modified status tracking.

## Usage
### Hotkeys (Default)
- **`Ctrl + Q`**: Quit
- **`Ctrl + E` / `:`**: Command Mode
- **`Ctrl + P`**: Fuzzy File Finder
- **`Ctrl + F`**: Search in Buffer
- **`Ctrl + S`**: Save
- **`Tab`**: Expand Snippet or Indent

### Command Mode
| Command | Action |
| :--- | :--- |
| `:h` | Help Overlay |
| `:vsplit` | Vertical Split |
| `:ls` | File Explorer |
| `:tree` | Tree Explorer |
| `:1`, `:2` | Switch Window |
