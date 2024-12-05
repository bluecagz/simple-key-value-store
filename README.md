# Simple Key-Value Store

A lightweight, in-memory key-value store implemented in Rust, with support for persistence and compaction.

## Features

- Basic Operations: `set`, `get`, and `delete` for managing key-value pairs.
- Persistence: Data is stored persistently in a binary file using `bincode`.
- Compaction: Periodically rewrites the data file to reclaim space from deleted keys.
- CLI Interface: Interactively manage the key-value store through the command line.

## Usage

Run the key-value store CLI:
```
cargo run
```

### Commands

- **Set a key-value pair**:
  ```
  > set <key> <value>
  ```

- **Get a value by key**:
  ```
  > get <key>
  ```

- **Delete a key**:
  ```
  > delete <key>
  ```

- **Compact the store**:
  ```
  > compact
  ```

- **Exit the CLI**:
  ```
  > exit
  ```
