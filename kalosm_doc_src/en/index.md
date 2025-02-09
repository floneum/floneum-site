# Introduction

Kalosm is a library with dead simple interfaces for local, language, audio, and image models

## Quick Start

- Add the Kalosm and Tokio libraries

```bash
# Enable the metal or cuda feature if you have a supported accelerator
cargo add kalosm --features language
cargo add tokio --features full
```

- Initialize a Kalosm model

```rust
{{#include src/doc_snippets/kalosm.rs:first}}
```

- Start a chat session with a pirate

```rust
{{#include src/doc_snippets/kalosm.rs:second}}
```

- Add build configuration to your `.cargo/config.toml` for improved performance

```toml
[build]
rustflags = ["-C", "target-cpu=native"]

[target.x86_64-apple-darwin]
rustflags = ["-C", "target-feature=-avx,-avx2"]
```

- Run the program

```bash
cargo run --release
```
