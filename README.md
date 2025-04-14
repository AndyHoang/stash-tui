# Stash TUI

A terminal user interface for [Stash](https://github.com/stashapp/stash) built with Rust and [Ratatui](https://github.com/ratatui-org/ratatui).

## Features

- Browse scenes, performers, and tags in your Stash collection
- Navigate through pages of content
- Terminal-based interface for quick access to your media

## Requirements

- Rust and Cargo
- A running Stash server

## Installation

1. Clone this repository:
```bash
git clone https://github.com/yourusername/stash-tui.git
cd stash-tui
```

2. Configure your Stash server URL:
```bash
# Edit the .env file with your Stash server URL
echo "STASH_URL=http://your-stash-server:9999" > .env
```

3. Build and run the application:
```bash
cargo build --release
cargo run --release
```

## Configuration

The application can be configured using:

1. **Environment variables**:
   - `STASH_URL`: The URL of your Stash server

2. **.env file**:
   - Create a `.env` file in the project root
   - Add `STASH_URL=http://your-stash-server:9999`

If not provided, it will use a default URL of `http://localhost:9999`.

## Usage

- `Tab` / `Shift+Tab`: Switch between tabs
- `↑` / `↓`: Navigate through items
- `PageUp` / `PageDown`: Change pages
- `r`: Refresh current view
- `q` or `Ctrl+C`: Quit

## Development

This is a work in progress. Future features may include:

- Detailed view of scenes, performers, and tags
- Search functionality
- Filtering options
- Configuration file for server settings
- Authentication support

## License

MIT
