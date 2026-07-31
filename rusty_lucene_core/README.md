# rusty_lucene_core

A minimal Rust crate that scaffolds the core functionality for a hypothetical Lucene-like search engine.

## Building
```sh
cargo build   # builds the library in debug mode
cargo test    # runs unit tests (including placeholder_test)
```

## Usage
Add `rusty_lucene_core` to your project's dependencies:

```toml
[dependencies]
rusty_lucene_core = { path = "../rusty_lucene_core" }
```
Then import the public API, e.g. `use rusty_lucene_core::hello_world;`.
