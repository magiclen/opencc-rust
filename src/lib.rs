/*!
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

The default OpenCC dictionaries will be compiled into the binary file by `lazy_static_include` crate. And you can use the two functions to recover them on demand.

For example,

```rust,ignore
use opencc_rust::*;

let output_path = "/path/to/dictionaries-directory";

generate_static_dictionary(&output_path, DefaultConfig::TW2SP).unwrap();

let opencc = OpenCC::new(Path::join(&output_path, DefaultConfig::TW2SP)).unwrap();

assert_eq!("凉风有讯", &opencc.convert("涼風有訊").unwrap());
```
*/

#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "static-dictionaries")]
#[macro_use]
extern crate lazy_static_include;

#[cfg(feature = "static-dictionaries")]
use std::fs::{self, File};
#[cfg(feature = "static-dictionaries")]
use std::io::Write;
#[cfg(feature = "static-dictionaries")]
use std::sync::LazyLock;
use std::{
    ffi::{CStr, CString},
    fmt,
    path::Path,
    sync::Mutex,
};

use enum_ordinalize::Ordinalize;
use libc::{c_char, c_int, c_void, size_t};

#[link(name = "opencc")]
unsafe extern "C" {
    pub fn opencc_open(config_file_path: *const c_char) -> *mut c_void;
    pub fn opencc_close(opencc: *mut c_void) -> c_int;
    pub fn opencc_convert_utf8(
        opencc: *mut c_void,
        input: *const c_char,
        length: size_t,
    ) -> *mut c_char;
    pub fn opencc_convert_utf8_to_buffer(
        opencc: *mut c_void,
        input: *const c_char,
        length: size_t,
        output: *mut c_char,
    ) -> size_t;
    pub fn opencc_convert_utf8_free(str: *mut c_char);
    pub fn opencc_error() -> *const c_char;
}

#[cfg(feature = "static-dictionaries")]
struct SD(&'static str, &'static [u8]);

#[cfg(feature = "static-dictionaries")]
macro_rules! new_sd_instance {
    ($name:ident, $file_name:expr) => {
        static $name: LazyLock<SD> = LazyLock::new(|| {
            lazy_static_include_bytes! {
                RES => ("opencc", $file_name)
            }

            SD($file_name, &RES)
        });
    };
}

#[cfg(feature = "static-dictionaries")]
new_sd_instance!(CJK_COMPATIBILITY_IDEOGRAPHS_OCD, "CJK_Compatibility_Ideographs.ocd2");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(HK2S_JSON, "hk2s.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(HK2SP_JSON, "hk2sp.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(HK2T_JSON, "hk2t.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(HKPHRASES_OCD, "HKPhrases.ocd2");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(HKPHRASES_REV_OCD, "HKPhrasesRev.ocd2");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(HKVARIANTS_OCD, "HKVariants.ocd2");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(HKVARIANTS_PHRASES_OCD, "HKVariantsPhrases.ocd2");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(HKVARIANTS_REV_OCD, "HKVariantsRev.ocd2");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(HKVARIANTS_REV_PHRASES_OCD, "HKVariantsRevPhrases.ocd2");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(JP2T_JSON, "jp2t.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(JPSHINJITAI_CHARACTERS_OCD, "JPShinjitaiCharacters.ocd2");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(JPSHINJITAI_CHARACTERS_REV_OCD, "JPShinjitaiCharactersRev.ocd2");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(JPSHINJITAI_PHRASES_OCD, "JPShinjitaiPhrases.ocd2");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(S2HK_JSON, "s2hk.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(S2HKP_JSON, "s2hkp.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(S2T_JSON, "s2t.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(S2TW_JSON, "s2tw.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(S2TWP_JSON, "s2twp.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(STCHARACTERS_OCD, "STCharacters.ocd2");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(
    STPHRASES_GENERATED_FROM_REGIONAL_PHRASES_OCD,
    "STPhrases_GeneratedFromRegionalPhrases.ocd2"
);
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(STPHRASES_OCD, "STPhrases.ocd2");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(T2HK_JSON, "t2hk.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(T2JP_JSON, "t2jp.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(T2S_JSON, "t2s.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(T2TW_JSON, "t2tw.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(TSCHARACTERS_OCD, "TSCharacters.ocd2");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(TSCHARACTERS_EXT_OCD, "TSCharactersExt.ocd2");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(TSPHRASES_OCD, "TSPhrases.ocd2");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(TW2S_JSON, "tw2s.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(TW2SP_JSON, "tw2sp.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(TW2T_JSON, "tw2t.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(TWPHRASES_OCD, "TWPhrases.ocd2");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(TWPHRASES_REV_OCD, "TWPhrasesRev.ocd2");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(TWVARIANTS_OCD, "TWVariants.ocd2");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(TWVARIANTS_PHRASES_OCD, "TWVariantsPhrases.ocd2");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(TWVARIANTS_REV_OCD, "TWVariantsRev.ocd2");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(TWVARIANTS_REV_PHRASES_OCD, "TWVariantsRevPhrases.ocd2");

/// Default configs.
#[derive(Debug, Copy, Clone, Ordinalize)]
pub enum DefaultConfig {
    /// Traditional Chinese (Hong Kong variant) to Simplified Chinese
    HK2S,
    /// Traditional Chinese (Hong Kong variant) to Simplified Chinese (with phrases)
    HK2SP,
    /// Traditional Chinese (Hong Kong variant) to Traditional Chinese (OpenCC Standard)
    HK2T,
    /// New Japanese Kanji (Shinjitai) to Old Japanese Kanji (Kyūjitai)
    JP2T,
    /// Simplified Chinese to Traditional Chinese (Hong Kong variant)
    S2HK,
    /// Simplified Chinese to Traditional Chinese (Hong Kong variant, with phrases)
    S2HKP,
    /// Simplified Chinese to Traditional Chinese (OpenCC Standard)
    S2T,
    /// Simplified Chinese to Traditional Chinese (Taiwan Standard)
    S2TW,
    /// Simplified Chinese to Traditional Chinese (Taiwan Standard, with Taiwan Phrases)
    S2TWP,
    /// Traditional Chinese (OpenCC Standard) to Traditional Chinese (Hong Kong variant)
    T2HK,
    /// Old Japanese Kanji (Kyūjitai) to New Japanese Kanji (Shinjitai)
    T2JP,
    /// Traditional Chinese (OpenCC Standard) to Simplified Chinese
    T2S,
    /// Traditional Chinese (OpenCC Standard) to Traditional Chinese (Taiwan Standard)
    T2TW,
    /// Traditional Chinese (Taiwan Standard) to Simplified Chinese
    TW2S,
    /// Traditional Chinese (Taiwan Standard) to Simplified Chinese (Mainland China Phrases)
    TW2SP,
    /// Traditional Chinese (Taiwan Standard) to Traditional Chinese (OpenCC Standard)
    TW2T,
}

impl DefaultConfig {
    /// Get the file name for this default config.
    pub fn get_file_name(self) -> &'static str {
        match self {
            DefaultConfig::HK2S => "hk2s.json",
            DefaultConfig::HK2SP => "hk2sp.json",
            DefaultConfig::HK2T => "hk2t.json",
            DefaultConfig::JP2T => "jp2t.json",
            DefaultConfig::S2HK => "s2hk.json",
            DefaultConfig::S2HKP => "s2hkp.json",
            DefaultConfig::S2T => "s2t.json",
            DefaultConfig::S2TW => "s2tw.json",
            DefaultConfig::S2TWP => "s2twp.json",
            DefaultConfig::T2HK => "t2hk.json",
            DefaultConfig::T2JP => "t2jp.json",
            DefaultConfig::T2S => "t2s.json",
            DefaultConfig::T2TW => "t2tw.json",
            DefaultConfig::TW2S => "tw2s.json",
            DefaultConfig::TW2SP => "tw2sp.json",
            DefaultConfig::TW2T => "tw2t.json",
        }
    }
}

impl AsRef<Path> for DefaultConfig {
    fn as_ref(&self) -> &Path {
        Path::new(self.get_file_name())
    }
}

impl AsRef<str> for DefaultConfig {
    fn as_ref(&self) -> &str {
        self.get_file_name()
    }
}

/// OpenCC binding for Rust.
pub struct OpenCC {
    opencc: *mut c_void,
}

/// Errors returned by the OpenCC wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenCCError {
    /// The config file path is not valid UTF-8.
    InvalidConfigPath,
    /// The config file path contains a null byte.
    ConfigPathContainsNul,
    /// The input string contains a null byte.
    InputContainsNul,
    /// OpenCC reported an error through its C API.
    OpenCC(String),
    /// OpenCC returned bytes that are not valid UTF-8.
    InvalidOutput,
}

impl fmt::Display for OpenCCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenCCError::InvalidConfigPath => {
                f.write_str("The config file path is not valid UTF-8.")
            },
            OpenCCError::ConfigPathContainsNul => {
                f.write_str("The config file path contains a null byte.")
            },
            OpenCCError::InputContainsNul => f.write_str("The input string contains a null byte."),
            OpenCCError::OpenCC(message) => f.write_str(message),
            OpenCCError::InvalidOutput => {
                f.write_str("OpenCC returned bytes that are not valid UTF-8.")
            },
        }
    }
}

impl std::error::Error for OpenCCError {}

static OPENCC_ERROR_LOCK: Mutex<()> = Mutex::new(());

fn opencc_error_lock() -> std::sync::MutexGuard<'static, ()> {
    OPENCC_ERROR_LOCK.lock().unwrap_or_else(|error| error.into_inner())
}

fn last_opencc_error() -> OpenCCError {
    let error = unsafe { opencc_error() };

    if error.is_null() {
        return OpenCCError::OpenCC("OpenCC returned an unknown error.".to_string());
    }

    match unsafe { CStr::from_ptr(error) }.to_str() {
        Ok(message) if !message.is_empty() => OpenCCError::OpenCC(message.to_string()),
        _ => OpenCCError::OpenCC("OpenCC returned an unreadable error message.".to_string()),
    }
}

// SAFETY: OpenCC handles can be moved between threads, and the non-thread-safe error API is serialized.
unsafe impl Send for OpenCC {}

// SAFETY: OpenCC conversion calls take an immutable handle, and the non-thread-safe error API is serialized.
unsafe impl Sync for OpenCC {}

impl OpenCC {
    /// Create a new OpenCC instance through a file provided by its path.
    pub fn new<P: AsRef<Path>>(config_file_path: P) -> Result<OpenCC, OpenCCError> {
        let config_file_path =
            config_file_path.as_ref().as_os_str().to_str().ok_or(OpenCCError::InvalidConfigPath)?;
        let config_file_path =
            CString::new(config_file_path).map_err(|_| OpenCCError::ConfigPathContainsNul)?;

        // OpenCC stores the last error globally, so the failing call and error read share one lock.
        let opencc = {
            let _lock = opencc_error_lock();
            let opencc = unsafe { opencc_open(config_file_path.as_ptr()) };

            if opencc.is_null() || opencc as usize == usize::MAX {
                return Err(last_opencc_error());
            }

            opencc
        };

        Ok(OpenCC {
            opencc,
        })
    }

    /// Convert a string to another string.
    pub fn convert<S: AsRef<str>>(&self, input: S) -> Result<String, OpenCCError> {
        let input = input.as_ref();

        let length = input.len();
        let input = CString::new(input).map_err(|_| OpenCCError::InputContainsNul)?;

        let result_ptr = {
            let _lock = opencc_error_lock();
            let result_ptr = unsafe { opencc_convert_utf8(self.opencc, input.as_ptr(), length) };

            if result_ptr.is_null() {
                return Err(last_opencc_error());
            }

            result_ptr
        };

        let result_cstr = unsafe { CStr::from_ptr(result_ptr) };
        let result =
            result_cstr.to_str().map(str::to_string).map_err(|_| OpenCCError::InvalidOutput);

        unsafe {
            opencc_convert_utf8_free(result_ptr);
        }

        result
    }

    /// Convert a string to another string and store into a buffer.
    pub fn convert_to_buffer<S: AsRef<str>>(
        &self,
        input: S,
        mut output: String,
    ) -> Result<String, OpenCCError> {
        output.push_str(&self.convert(input)?);

        Ok(output)
    }
}

impl Drop for OpenCC {
    fn drop(&mut self) {
        if !self.opencc.is_null() {
            unsafe {
                opencc_close(self.opencc);
            }
        }
    }
}

#[cfg(feature = "static-dictionaries")]
fn generate_static_dictionary_inner<P: AsRef<Path>>(
    path: P,
    config: DefaultConfig,
) -> Result<(), &'static str> {
    let path = path.as_ref();

    let mut output_data: Vec<&SD> = Vec::new();

    // Each config JSON must be generated with every OCD2 dictionary file that it references.
    match config {
        DefaultConfig::HK2S => {
            output_data.push(&HK2S_JSON);
            output_data.push(&CJK_COMPATIBILITY_IDEOGRAPHS_OCD);
            output_data.push(&TSPHRASES_OCD);
            output_data.push(&HKVARIANTS_REV_PHRASES_OCD);
            output_data.push(&HKVARIANTS_REV_OCD);
            output_data.push(&TSPHRASES_OCD);
            output_data.push(&TSCHARACTERS_EXT_OCD);
            output_data.push(&TSCHARACTERS_OCD);
        },
        DefaultConfig::HK2SP => {
            output_data.push(&HK2SP_JSON);
            output_data.push(&CJK_COMPATIBILITY_IDEOGRAPHS_OCD);
            output_data.push(&TSPHRASES_OCD);
            output_data.push(&HKPHRASES_REV_OCD);
            output_data.push(&HKVARIANTS_REV_PHRASES_OCD);
            output_data.push(&HKVARIANTS_REV_OCD);
            output_data.push(&TSPHRASES_OCD);
            output_data.push(&TSCHARACTERS_EXT_OCD);
            output_data.push(&TSCHARACTERS_OCD);
        },
        DefaultConfig::HK2T => {
            output_data.push(&HK2T_JSON);
            output_data.push(&CJK_COMPATIBILITY_IDEOGRAPHS_OCD);
            output_data.push(&HKVARIANTS_REV_PHRASES_OCD);
            output_data.push(&HKVARIANTS_REV_OCD);
        },
        DefaultConfig::JP2T => {
            output_data.push(&JP2T_JSON);
            output_data.push(&CJK_COMPATIBILITY_IDEOGRAPHS_OCD);
            output_data.push(&JPSHINJITAI_PHRASES_OCD);
            output_data.push(&JPSHINJITAI_CHARACTERS_OCD);
        },
        DefaultConfig::S2HK => {
            output_data.push(&S2HK_JSON);
            output_data.push(&CJK_COMPATIBILITY_IDEOGRAPHS_OCD);
            output_data.push(&STPHRASES_OCD);
            output_data.push(&STPHRASES_GENERATED_FROM_REGIONAL_PHRASES_OCD);
            output_data.push(&STCHARACTERS_OCD);
            output_data.push(&HKVARIANTS_PHRASES_OCD);
            output_data.push(&HKVARIANTS_OCD);
        },
        DefaultConfig::S2HKP => {
            output_data.push(&S2HKP_JSON);
            output_data.push(&CJK_COMPATIBILITY_IDEOGRAPHS_OCD);
            output_data.push(&STPHRASES_OCD);
            output_data.push(&STPHRASES_GENERATED_FROM_REGIONAL_PHRASES_OCD);
            output_data.push(&STCHARACTERS_OCD);
            output_data.push(&HKPHRASES_OCD);
            output_data.push(&HKVARIANTS_PHRASES_OCD);
            output_data.push(&HKVARIANTS_OCD);
        },
        DefaultConfig::S2T => {
            output_data.push(&S2T_JSON);
            output_data.push(&CJK_COMPATIBILITY_IDEOGRAPHS_OCD);
            output_data.push(&STPHRASES_OCD);
            output_data.push(&STPHRASES_GENERATED_FROM_REGIONAL_PHRASES_OCD);
            output_data.push(&STCHARACTERS_OCD);
        },
        DefaultConfig::S2TW => {
            output_data.push(&S2TW_JSON);
            output_data.push(&CJK_COMPATIBILITY_IDEOGRAPHS_OCD);
            output_data.push(&STPHRASES_OCD);
            output_data.push(&STPHRASES_GENERATED_FROM_REGIONAL_PHRASES_OCD);
            output_data.push(&STCHARACTERS_OCD);
            output_data.push(&TWVARIANTS_PHRASES_OCD);
            output_data.push(&TWVARIANTS_OCD);
        },
        DefaultConfig::S2TWP => {
            output_data.push(&S2TWP_JSON);
            output_data.push(&CJK_COMPATIBILITY_IDEOGRAPHS_OCD);
            output_data.push(&STPHRASES_OCD);
            output_data.push(&STPHRASES_GENERATED_FROM_REGIONAL_PHRASES_OCD);
            output_data.push(&STCHARACTERS_OCD);
            output_data.push(&TWPHRASES_OCD);
            output_data.push(&TWVARIANTS_PHRASES_OCD);
            output_data.push(&TWVARIANTS_OCD);
        },
        DefaultConfig::T2HK => {
            output_data.push(&T2HK_JSON);
            output_data.push(&CJK_COMPATIBILITY_IDEOGRAPHS_OCD);
            output_data.push(&HKVARIANTS_PHRASES_OCD);
            output_data.push(&HKVARIANTS_OCD);
        },
        DefaultConfig::T2JP => {
            output_data.push(&T2JP_JSON);
            output_data.push(&CJK_COMPATIBILITY_IDEOGRAPHS_OCD);
            output_data.push(&JPSHINJITAI_CHARACTERS_REV_OCD);
        },
        DefaultConfig::T2S => {
            output_data.push(&T2S_JSON);
            output_data.push(&CJK_COMPATIBILITY_IDEOGRAPHS_OCD);
            output_data.push(&TSPHRASES_OCD);
            output_data.push(&TSCHARACTERS_EXT_OCD);
            output_data.push(&TSCHARACTERS_OCD);
        },
        DefaultConfig::T2TW => {
            output_data.push(&T2TW_JSON);
            output_data.push(&CJK_COMPATIBILITY_IDEOGRAPHS_OCD);
            output_data.push(&TWVARIANTS_PHRASES_OCD);
            output_data.push(&TWVARIANTS_OCD);
        },
        DefaultConfig::TW2S => {
            output_data.push(&TW2S_JSON);
            output_data.push(&CJK_COMPATIBILITY_IDEOGRAPHS_OCD);
            output_data.push(&TSPHRASES_OCD);
            output_data.push(&TWVARIANTS_REV_PHRASES_OCD);
            output_data.push(&TWVARIANTS_REV_OCD);
            output_data.push(&TSPHRASES_OCD);
            output_data.push(&TSCHARACTERS_EXT_OCD);
            output_data.push(&TSCHARACTERS_OCD);
        },
        DefaultConfig::TW2SP => {
            output_data.push(&TW2SP_JSON);
            output_data.push(&CJK_COMPATIBILITY_IDEOGRAPHS_OCD);
            output_data.push(&TSPHRASES_OCD);
            output_data.push(&TWPHRASES_REV_OCD);
            output_data.push(&TWVARIANTS_REV_PHRASES_OCD);
            output_data.push(&TWVARIANTS_REV_OCD);
            output_data.push(&TSPHRASES_OCD);
            output_data.push(&TSCHARACTERS_EXT_OCD);
            output_data.push(&TSCHARACTERS_OCD);
        },
        DefaultConfig::TW2T => {
            output_data.push(&TW2T_JSON);
            output_data.push(&CJK_COMPATIBILITY_IDEOGRAPHS_OCD);
            output_data.push(&TWVARIANTS_REV_PHRASES_OCD);
            output_data.push(&TWVARIANTS_REV_OCD);
        },
    }

    for data in output_data {
        let output_path = path.join(data.0);

        if output_path.exists() {
            if output_path.is_file() {
                continue;
            } else {
                return Err("The dictionary is not correct.");
            }
        }

        let mut file = File::create(output_path).map_err(|_| "Cannot create a new file.")?;

        file.write(data.1).map_err(|_| "Cannot write data to a file.")?;

        file.flush().map_err(|_| "Cannot flush file.")?;
    }

    Ok(())
}

#[cfg(feature = "static-dictionaries")]
/// Generate files for a specific dictionary. These files are used for opening a new OpenCC instance.
pub fn generate_static_dictionary<P: AsRef<Path>>(
    path: P,
    config: DefaultConfig,
) -> Result<(), &'static str> {
    let path = path.as_ref();

    if path.exists() {
        if !path.is_dir() {
            return Err("The path of static dictionaries needs to be a directory.");
        }
    } else {
        match fs::create_dir_all(path) {
            Ok(_) => (),
            Err(_) => return Err("Cannot create new directories."),
        }
    }

    generate_static_dictionary_inner(path, config)
}

#[cfg(feature = "static-dictionaries")]
/// Generate files for specific dictionaries. These files are used for opening a new OpenCC instance.
pub fn generate_static_dictionaries<P: AsRef<Path>>(
    path: P,
    configs: &[DefaultConfig],
) -> Result<(), &'static str> {
    let path = path.as_ref();

    if path.exists() {
        if !path.is_dir() {
            return Err("The path of static dictionaries needs to be a directory.");
        }
    } else {
        match fs::create_dir_all(path) {
            Ok(_) => (),
            Err(_) => return Err("Cannot create new directories."),
        }
    }

    for config in configs.iter().copied() {
        generate_static_dictionary_inner(path, config)?
    }

    Ok(())
}
