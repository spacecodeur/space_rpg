# Space RPG

[![CI](https://github.com//space_rpg/workflows/CI/badge.svg)](https://github.com//space_rpg/actions)

A terminal-based RPG built with Rust and ratatui for learning and experimentation.

## Features

### Currently Implemented
- **Book-style UI**: Two-page layout with left and right pages
- **Smart Pagination**: Text automatically flows between pages and handles overflow
- **Responsive Design**: Adapts to terminal size with dynamic text wrapping
- **Modular Architecture**: Clean component-based structure for easy extension

### UI Components
- **Book Component**: Displays story content with adaptive pagination
- **Action Input**: Handles user input and commands
- **Component System**: Extensible architecture following ratatui best practices

## Quick Start

```bash
# Clone the repository
git clone <repository-url>
cd space_rpg

# Run the application
cargo run

# Or build first
cargo build
cargo run
```

## Architecture

The project follows a modular component architecture:

```
src/
├── app.rs              # Main application orchestrator
├── action.rs           # Action definitions (NextPage, PreviousPage, etc.)
├── components/
│   ├── home.rs         # Main layout component
│   ├── book/           # Book component with pagination
│   │   ├── mod.rs      # Main book logic
│   │   ├── paginator.rs    # Pagination handling
│   │   └── text_processor.rs  # Text wrapping and formatting
│   └── action_input.rs # User input handling
└── ...
```

## Development

### Prerequisites
- Rust 1.70+ 
- Terminal with good Unicode support

### Building
```bash
cargo check    # Quick syntax check
cargo build    # Full build
cargo run      # Build and run
```

### Adding Features

1. **New Components**: Follow the `Component` trait pattern in `src/components.rs`
2. **New Actions**: Add to `src/action.rs` enum and handle in relevant components
3. **UI Changes**: Use ratatui layouts and widgets, consult local docs in `docs/ratatui/`

### Code Style
- Modular design: extract logic when files/functions grow large
- Follow Rust conventions and run `cargo fmt`
- Always test compilation with `cargo check` before committing

## Documentation

- **Project-specific**: See `CLAUDE.md` for detailed development guidelines
- **Ratatui docs**: Local documentation available in `docs/ratatui/guide/`
- **API reference**: Available in `docs/ratatui/code/`

## Current Status

This is an experimental project for learning ratatui and terminal UI development. The book UI system is fully functional and serves as a foundation for future RPG features.

## Future Features

Potential additions could include:
- Game mechanics (combat, inventory, character stats)
- Interactive story elements
- Save/load system  
- Multiple UI themes
- Sound/music integration
