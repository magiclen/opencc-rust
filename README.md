OpenCC Rust
====================

[![CI](https://github.com/magiclen/opencc-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/opencc-rust/actions/workflows/ci.yml)

Open Chinese Convert(OpenCC, 開放中文轉換) binding for the Rust language for conversion between Traditional Chinese and Simplified Chinese.

## Compilation

To compile this crate, you need to compile the OpenCC C++ library first. You can install OpenCC in your operating system, or in somewhere in your file system. As for the latter, you need to set the following environment variables to link the OpenCC library:

* `OPENCC_LIB_DIRS`: The directories of library files, like `-L`. Use `:` to separate.
* `OPENCC_LIBS`: The library names that you want to link, like `-l`. Use `:` to separate. Typically, it contains **opencc:marisa**.
* `OPENCC_INCLUDE_DIRS`: The directories of header files, like `-i`. Use `:` to separate.
* `OPENCC_STATIC`: Whether to use `static` or `dylib`.
* `OPENCC_DYLIB_STDCPP`: If you use `static` linking, and your OpenCC library is compiled by the GNU C, this environment variable should be set.
* `OPENCC_STATIC_STDCPP`: If you use `static` linking, and your OpenCC library is compiled by musl libc, this environment variable should be set.

## Examples

```rust
use opencc_rust::*;

let opencc = OpenCC::new(DefaultConfig::TW2SP).unwrap();

let s = opencc.convert("涼風有訊");

assert_eq!("凉风有讯", &s);

let s = opencc.convert_to_buffer("，秋月無邊", s);

assert_eq!("凉风有讯，秋月无边", &s);
```

```rust
use opencc_rust::*;

let opencc = OpenCC::new(DefaultConfig::S2TWP).unwrap();

let s = opencc.convert("凉风有讯");

assert_eq!("涼風有訊", &s);

let s = opencc.convert_to_buffer("，秋月无边", s);

assert_eq!("涼風有訊，秋月無邊", &s);
```

## Static Dictionaries

Usually, OpenCC needs to be executed on an environment where OpenCC is installed. If you want to make it portable, you can enable the `static-dictionaries` feature.

```toml
[dependencies.opencc-rust]
version = "*"
features = ["static-dictionaries"]
```
Then, the `generate_static_dictionary` and `generate_static_dictionaries` functions are available.

The default OpenCC dictionaries will be compiled into the binary file by `lazy_static_include` crate. And you can use the two functions to recover them on demand.

For example,

```rust
use opencc_rust::*;

let output_path = "/path/to/dictionaries-directory";

generate_static_dictionary(&output_path, DefaultConfig::TW2SP).unwrap();

let opencc = OpenCC::new(Path::join(&output_path, DefaultConfig::TW2SP)).unwrap();

assert_eq!("凉风有讯", &opencc.convert("涼風有訊"));
```

## Supported Platforms

This crate currently supports **Linux**. Other platforms are not guaranteed.

## Crates.io

https://crates.io/crates/opencc-rust

## Documentation

https://docs.rs/opencc-rust

## License

[Apache-2.0](LICENSE)