# 💾 nfd2

[![Build Status](https://github.com/EmbarkStudios/nfd2/workflows/CI/badge.svg)](https://github.com/EmbarkStudios/nfd2/actions?workflow=CI)
[![Crates.io](https://img.shields.io/crates/v/nfd2.svg)](https://crates.io/crates/nfd2)
[![Docs](https://docs.rs/nfd2/badge.svg)](https://docs.rs/nfd2)
[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4%20adopted-ff69b4.svg)](CODE_OF_CONDUCT.md)
[![Embark](https://img.shields.io/badge/embark-open%20source-blueviolet.svg)](https://embark.dev)

`nfd2` is a Rust binding to the [nativefiledialog](https://github.com/mlabbe/nativefiledialog) library, that provides a convenient cross-platform interface to opening file dialogs on Windows, MacOS, and Linux.

## This is a fork!

The original [nfd-rs](https://github.com/saurvs/nfd-rs) crate appears essentially unmaintained by now, so we have made this fork with the intent of making sure that it is at least maintained and that bugs stay fixed so we can have something to rely on.

That being said, our ultimate goal with this crate is to eventually make it pure Rust, without a need for external C code or a build script at all.

## Usage

### Single File Dialog

```rust
use nfd2::Response;

fn main() {
    match nfd2::open_file_dialog(None, None).expect("oh no") {
        Response::Okay(file_path) => println!("File path = {:?}", file_path),
        Response::OkayMultiple(files) => println!("Files {:?}", files),
        Response::Cancel => println!("User canceled"),
    }
}
```

### Multiple File Dialog

```rust
use nfd2::Response;

fn main() {
    /// Only show .jpg files
    let result = nfd2::dialog_multiple().filter("jpg").open().expect("oh no");

    match result {
        Response::Okay(file_path) => println!("File path = {:?}", file_path),
        Response::OkayMultiple(files) => println!("Files {:?}", files),
        Response::Cancel => println!("User canceled"),
    }
}
```

## Contributing

We welcome community contributions to this project.

Please read our [Contributor Guide](CONTRIBUTING.md) for more information on how to get started.

## License

MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
