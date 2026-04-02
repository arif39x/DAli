# DAli Editor

A high-performance, terminal text editor with a Rust-Python hybrid architecture.

## Command Reference

### Editor Hotkeys
- **`Ctrl + Q`**: Quick Exit
- **`Ctrl + S`**: Save Current Buffer
- **`Ctrl + F`**: Incremental Text Search
- **`Ctrl + P`**: Fuzzy File Finder
- **`Ctrl + W`**: Cycle Focus Between Windows
- **`Ctrl + C`**: Copy selection to clipboard
- **`Ctrl + X`**: Cut selection to clipboard
- **`Ctrl + V`**: Paste from clipboard
- **`Shift + Arrow Keys`**: Text Selection
- **`Tab`**: Indent selection or Expand Snippet
- **`Shift + Tab`**: Outdent selection or current line
- **`Ctrl + E`**: Enter DAli-Term Command Mode
- **`Arrow Keys`**: Character-Atomic Navigation

### DAli-Term Commands
| Command | Action |
| :--- | :--- |
| **`h`**, **`help`** | Toggle Help Overlay |
| **`q`**, **`quit`** | Quit Editor |
| **`s`**, **`save`** | Save Current File |
| **`vsplit`** | Vertical Window Split |
| **`vsplit term`**, **`term`** | Integrated Terminal Split |
| **`1`**, **`2`**, **`...`** | Jump to specific window number |
| **`pwd`** | Print Working Directory to Status Bar |
| **`ls`** | Directory Content List mode |
| **`tree`** | Recursive Directory Tree mode |
| **`build`** | Execute Project Build & Run Pipeline |

### View Modes
- **Editor Mode**: Main text editing viewport.
- **FileList Mode**: Interactive directory explorer.
- **Help Mode**: Real-time hotkey and command reference.

