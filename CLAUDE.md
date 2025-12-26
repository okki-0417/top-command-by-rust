# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust implementation of a `top` command-like TUI application for macOS/Linux. Displays process list with CPU/memory usage as bar graphs, with real-time updates and keyboard controls.

## Build Commands

```bash
cargo build           # Debug build
cargo build --release # Release build
cargo run             # Run application
cargo test            # Run tests
cargo clippy          # Lint
cargo fmt             # Format code
```

## Architecture

```
src/
├── main.rs      # Entry point, main loop, keyboard event handling
├── process.rs   # ProcessInfo struct, fetch_processes(), SortKey enum
└── ui.rs        # Screen rendering, bar graphs, colors
```

### Key Components

- `ProcessInfo` - Struct holding pid, name, cpu_percent, memory_mb
- `SortKey` - Enum for sorting (Cpu, Memory)
- `fetch_processes()` - Gets process list from sysinfo, sorts by key
- `display_rows()` - Renders process list with colored bar graphs

### Terminal Handling

- Uses `crossterm` for raw mode and keyboard input
- Raw mode requires `\r\n` instead of `\n` for line breaks
- `event::poll()` for non-blocking key input

## Dependencies

- `sysinfo` - Process information retrieval
- `crossterm` - Terminal handling, keyboard events

## UI Constants (ui.rs)

- `BAR_WIDTH` - Width of CPU/MEM bar graphs
- `NAME_WIDTH` - Max width of process name column
