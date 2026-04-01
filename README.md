# DAli Editor

A high-performance, terminal text editor with a Rust-Python hybrid architecture. Optimized for sub-10ms latency and extensible intelligence.

## Command Reference

### Editor Hotkeys
- **`Ctrl + Q`**: Quick Exit
- **`Ctrl + S`**: Save Current Buffer
- **`Ctrl + F`**: Incremental Text Search
- **`Ctrl + P`**: Fuzzy File Finder
- **`Ctrl + W`**: Cycle Focus Between Windows
- **`Arrow Keys`**: Character-Atomic Navigation
- **`:`**: Enter DAli-Term Command Mode

### DAli-Term Commands
| Command | Action |
| :--- | :--- |
| **`h`**, **`help`** | Toggle Help Overlay |
| **`q`**, **`quit`** | Quit Editor |
| **`s`**, **`save`** | Save Current File |
| **`vsplit`** | Vertical Window Split |
| **`vsplit term`** | Integrated Terminal Split |
| **`pwd`** | Print Working Directory to Status Bar |
| **`ls`** | Directory Content List mode |
| **`tree`** | Recursive Directory Tree mode |
| **`build`** | Execute Project Build & Run Pipeline |

### View Modes
- **Editor Mode**: Main text editing viewport.
- **FileList Mode**: Interactive directory explorer.
- **Help Mode**: Real-time hotkey and command reference.

## Architecture
- **Core Engine**: Rust (Zero-Allocation UTF-8 GapBuffer)
- **UI Logic**: Viewport-Aware Multi-Window System
- **Intelligence**: Python Bridge via PyO3 (Non-Blocking FFI)
