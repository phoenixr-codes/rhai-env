# About `rhai-env`

[![crates.io](https://img.shields.io/crates/v/rhai-env?logo=rust)](https://crates.io/crates/rhai-env/)
[![crates.io](https://img.shields.io/crates/d/rhai-env?logo=rust)](https://crates.io/crates/rhai-env/)
[![API Docs](https://docs.rs/rhai-env/badge.svg?logo=docs-rs)](https://docs.rs/rhai-env/)

This crate provides inspection and manipulation utilties of the process's environment for the
[Rhai] scripting language.

This crate is heavily inspired by [rhai-fs][].

## Usage

### `Cargo.toml`

```toml
[dependencies]
rhai-env = "0.1.2"
```

### [Rhai] script

```js
let foo = read_env("FOO");
set_env("BAR", "blah");
```

### Rust source

```rust
use rhai::{Engine, EvalAltResult};
use rhai::packages::Package;
use rhai_env::EnvironmentPackage;

fn main() -> Result<(), Box<EvalAltResult>> {
    // Create Rhai scripting engine
    let mut engine = Engine::new();

    // Create environment package and add the package into the engine
    let package = EnvironmentPackage::new();
    package.register_into_engine(&mut engine);

    // Print the value of the environment variable `PATH`.
    let value = engine.eval::<String>(r#"env("PATH")"#)?;
    println!("{}", value);

    Ok(())
}
```

## Features

|  Feature   | Default  | Description                                          |
| :--------: | :------: | ---------------------------------------------------- |
| `no_index` | disabled | Enables support for `no_index` builds of [Rhai]      |
|   `sync`   | disabled | Enables support for `sync` builds of [Rhai]          |
| `metadata` | disabled | Enables support for generating package documentation |

[Rhai]: https://rhai.rs
[rhai-fs]: https://github.com/rhaiscript/rhai-fs
