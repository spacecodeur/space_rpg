# Project-Specific Instructions for Claude

## Project Overview

Space RPG is a terminal-based RPG built with Rust and ratatui. The project features a modular component architecture with a book-style UI for displaying story content.

## Current Architecture

### Component Structure
- **App**: Main application orchestrator (`src/app.rs`)
- **Home**: Main layout component (`src/components/home.rs`)
- **Book**: Story display with pagination (`src/components/book/`)
  - `mod.rs`: Main book component
  - `paginator.rs`: Pagination logic
  - `text_processor.rs`: Text wrapping and formatting
- **ActionInput**: User input component (`src/components/action_input.rs`)

### Key Features Implemented
- **Adaptive Book UI**: Two-page layout (left/right) with responsive text flow
- **Smart Pagination**: Text automatically flows from left page to right page, then to next pages
- **Responsive Design**: Text width adapts to terminal size
- **Action System**: NextPage/PreviousPage actions for navigation

## Development Guidelines

### Component Development
- Follow the `Component` trait pattern defined in `src/components.rs`
- All components must implement: `register_action_handler`, `register_config_handler`, `update`, and `draw`
- Use modular architecture - separate logic into focused modules when components grow large

### Action System
- Add new actions to `src/action.rs` enum
- Handle actions in relevant component's `update` method
- Actions flow through the component hierarchy automatically

### Code Quality
- **Always run** `cargo check` and `cargo build` before committing
- Keep methods and files focused and well-named
- Extract complex logic into separate modules when functions exceed ~50 lines

## Ratatui Integration

This project uses ratatui version 0.29. When working on ratatui features:

1. **Consult local documentation** in `docs/ratatui/guide/` for concepts and patterns
2. **Reference API docs** in `docs/ratatui/code/` for specific implementations
3. **Follow ratatui best practices**:
   - Use Layout for responsive design
   - Implement proper widget rendering with `frame.render_widget()`
   - Handle area calculations correctly for nested layouts

## Testing & Building

- **Check compilation**: `cargo check`
- **Build project**: `cargo build` 
- **Run application**: `cargo run`
- The app should compile without errors and display a functional book UI

## Commit Guidelines

Follow the established commit format:
- Use types: `feature`, `fix`, `refactor`, `docs`, `chore`, etc.
- Write descriptive messages focusing on the "why" not just the "what"
- Keep commits focused and atomic

## Known Components for Reference

- **Book Component**: Fully implemented with adaptive pagination
- **ActionInput**: Basic input handling component  
- **FPS Component**: Available but commented out in `src/components/fps.rs`

This architecture supports easy extension - new game features can be added as new components following the established patterns.