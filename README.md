# Todo Tree for Zed

## Development

1. Install Rust via rustup (required by Zed for dev extensions).
2. In Zed: command palette → `zed: install dev extension` → select this directory.
3. Logs: run `zed --foreground` to see `println!`/`dbg!` output.

## Layout

- `extension.toml` – extension manifest
- `Cargo.toml` / `src/lib.rs` – Rust (WASM) extension code
- `LICENSE` – required for publishing to the Zed extension registry
