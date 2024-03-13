# Your First plugin

This example will use [rust](https://www.rust-lang.org/) to build an new plugin for Floneum.

First, edit your cargo.toml to add floneum_rust and wit-bindgen as dependencies and change the crate type to a dynamically linked (C-like) library

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
floneum_rust = { git = "https://github.com/floneum/floneum" }
wit-bindgen = { git = "https://github.com/bytecodealliance/wit-bindgen", rev = "285f0c6ad5da3d6cd8ef2e0635df51f229d6578f" }
```

Then create your plugin with the export plugin macro:
```rust
{{#include examples/add_plugin.rs:plugin}}
```

Next, build your plugin:
```sh
cargo install --git https://github.com/floneum/floneum floneum-cli
floneum build --release
```

> You can look at the default plugins [here](../../../plugins) to see how more complex plugins work

Finally, load your plugin by running the main Floneum application and typing the path to your `.wasm` file in the load plugin text box in the top left.
