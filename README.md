OpenCC Rust
====================

[![CI](https://github.com/magiclen/opencc-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/opencc-rust/actions/workflows/ci.yml)

Open Chinese Convert(OpenCC, 開放中文轉換) binding for the Rust language for conversion between Traditional Chinese and Simplified Chinese.

## Compilation

To compile this crate, you need to compile the OpenCC C++ library first. You can install OpenCC in your operating system, or in somewhere in your file system. As for the latter, you need to set the following environment variables to link the OpenCC library:

* `OPENCC_LIB_DIRS`: The directories of library files, like `-L`. Use `:` to separate.
* `OPENCC_LIBS`: The library names that you want to link, like `-l`. Use `:` to separate. Typically, it contains **opencc:marisa**.
* `OPENCC_INCLUDE_DIRS`: The directories of header files, like `-i`. Use `:` to separate.
* `OPENCC_STATIC`: Set to `0` for dynamic linking, or any other value for static linking. If unset, the library files determine the linking mode.
* `OPENCC_DYLIB_STDCPP`: If you use `static` linking, and your OpenCC library is compiled by the GNU C, this environment variable should be set.
* `OPENCC_STATIC_STDCPP`: If you use `static` linking, and your OpenCC library is compiled by musl libc, this environment variable should be set.

## Examples

```rust
use opencc_rust::*;

let opencc = OpenCC::new(DefaultConfig::TW2SP).unwrap();

let s = opencc.convert("涼風有訊").unwrap();

assert_eq!("凉风有讯", &s);

let s = opencc.convert_to_buffer("，秋月無邊", s).unwrap();

assert_eq!("凉风有讯，秋月无边", &s);
```

```rust
use opencc_rust::*;

let opencc = OpenCC::new(DefaultConfig::S2TWP).unwrap();

let s = opencc.convert("凉风有讯").unwrap();

assert_eq!("涼風有訊", &s);

let s = opencc.convert_to_buffer("，秋月无边", s).unwrap();

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

The `lazy_static_include` crate embeds the default OpenCC dictionaries when `debug_assertions` is disabled, as in the default release profile.
When `debug_assertions` is enabled, as in the default debug profile, it reads the original dictionary files at runtime, so those files must remain at their original paths.
You can use the two functions to recover the dictionaries on demand.
Existing files in the output directory are not overwritten, so delete them before generating the dictionaries again after upgrading this crate.

This feature only provides dictionary files.
You still need to link the OpenCC library.
Dynamic linking also requires that library at runtime.
The bundled `.ocd2` files are little-endian, so they cannot be used on big-endian targets.

For example,

```rust
use std::path::Path;

use opencc_rust::*;

let output_path = "/path/to/dictionaries-directory";

generate_static_dictionary(&output_path, DefaultConfig::TW2SP).unwrap();

let opencc = OpenCC::new(Path::new(output_path).join(DefaultConfig::TW2SP)).unwrap();

assert_eq!("凉风有讯", &opencc.convert("涼風有訊").unwrap());
```

## Supported Platforms

This crate officially supports **Linux**. FreeBSD defaults and Windows library naming are handled on a best-effort basis, but those platforms are not guaranteed.

## Crates.io

https://crates.io/crates/opencc-rust

## Documentation

https://docs.rs/opencc-rust

## License

[Apache-2.0](LICENSE)
