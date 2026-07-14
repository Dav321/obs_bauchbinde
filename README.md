# Bauchbinde
A small Rust web app for managing and displaying lower-third title graphics, built for OBS.

## Features
- Control page for adding, editing, deleting, and selecting titles
- Separate display view for browser sources, e.g. OBS
- Configurable display duration
- Local SQLite storage

## Requirements
- Rust and Cargo
- A `logo.png` file in the working directory

## Run
Build and run:
```bash
cargo run
```

## Data
Title presets are stored in a local SQLite database named ```bauchbinde.db```.
The database is created automatically in the working directory.
