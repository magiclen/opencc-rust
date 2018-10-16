//! Open Chinese Convert(OpenCC, 開放中文轉換) binding for the Rust language for conversion between Traditional Chinese and Simplified Chinese.
//!
//! ## Compilation
//!
//! To compile this crate, you need to compile the OpenCC C++ library first. You can install OpenCC in your operating system, or in somewhere in your file system. As for the latter, you need to set the following environment variables to link the OpenCC library:
//!
//! * `OPENCC_LIB_DIRS`: The directories of library files, like `-L`. Use `:` to separate.
//! * `OPENCC_LIBS`: The library names that you want to link, like `-l`. Use `:` to separate. Typically, it only contains **opencc**.
//! * `OPENCC_INCLUDE_DIRS`: The directories of header files, like `-i`. Use `:` to separate.
//! * `OPENCC_STATIC`: Whether to use `static` or `dylib`.
//! * `OPENCC_DYLIB_STDCPP`: If you use `static` linking, and your OpenCC library is compiled by the GNU C, this environment variable should be set.
//!
//! ## Examples
//!
//! ```
//! extern crate opencc_rust;
//!
//! use opencc_rust::*;
//!
//! let opencc = OpenCC::new(DefaultConfig::TW2SP).unwrap();
//!
//! let s = opencc.convert("涼風有訊");
//!
//! assert_eq!("凉风有讯", &s);
//!
//! let s = opencc.convert_to_buffer("，秋月無邊", s);
//!
//! assert_eq!("凉风有讯，秋月无边", &s);
//! ```
//!
//! ```
//! extern crate opencc_rust;
//!
//! use opencc_rust::*;
//!
//! let opencc = OpenCC::new(DefaultConfig::S2TWP).unwrap();
//!
//! let s = opencc.convert("凉风有讯");
//!
//! assert_eq!("涼風有訊", &s);
//!
//! let s = opencc.convert_to_buffer("，秋月无边", s);
//!
//! assert_eq!("涼風有訊，秋月無邊", &s);
//! ```
//!
//! ## Static Dictionaries
//!
//! Usually, OpenCC needs to be executed on an environment where OpenCC is installed. If you want to make it portable, you can enable the `static-dictionaries` feature.
//!
//! ```toml
//! [dependencies.opencc-rust]
//! version = "*"
//! features = ["static-dictionaries"]
//! ```
//! Then, the `generate_static_dictionary` and `generate_static_dictionaries` functions are available.
//!
//! The default OpenCC dictionaries will be compiled into the binary file by `lazy_static_include` crate. And you can use the two functions to recover them on demand.
//!
//! For example,
//!
//! ```rust,ignore
//! extern crate opencc_rust;
//!
//! use opencc_rust::*;
//!
//! let output_path = "/path/to/dictionaries-directory";
//!
//! generate_static_dictionary(&output_path, DefaultConfig::TW2SP).unwrap();
//!
//! let opencc = OpenCC::new(Path::join(&output_path, DefaultConfig::TW2SP)).unwrap();
//!
//! assert_eq!("凉风有讯", &opencc.convert("涼風有訊"));
//! ```

extern crate libc;
#[cfg(feature = "static-dictionaries")]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "static-dictionaries")]
#[macro_use]
extern crate lazy_static_include;

use libc::{c_int, size_t, c_void, c_char};

use std::ffi::{CStr, CString};
use std::mem::transmute;
use std::path::Path;
#[cfg(feature = "static-dictionaries")]
use std::fs::{self, File};
#[cfg(feature = "static-dictionaries")]
use std::io::Write;

#[link(name = "opencc")]
extern "C" {
    pub fn opencc_open(config_file_path: *const c_char) -> *mut c_void;
    pub fn opencc_close(opencc: *mut c_void) -> c_int;
    pub fn opencc_convert_utf8(opencc: *mut c_void, input: *const c_char, length: size_t) -> *mut c_char;
    pub fn opencc_convert_utf8_to_buffer(opencc: *mut c_void, input: *const c_char, length: size_t, output: *mut c_char) -> size_t;
    pub fn opencc_convert_utf8_free(str: *mut c_char);
    pub fn opencc_error() -> *const c_char;
}

#[cfg(feature = "static-dictionaries")]
struct SD(&'static str, &'static [u8]);

#[cfg(feature = "static-dictionaries")]
macro_rules! new_sd_instance {
    ($name:ident, $file_name:expr) => {
        lazy_static! {
            static ref $name: SD = {
                lazy_static_include_bytes!(Res, concat!("opencc/", $file_name));
                SD($file_name, &Res)
            };
        }
    };
}

#[cfg(feature = "static-dictionaries")]
new_sd_instance!(HK2S_JSON, "hk2s.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(HKVARIANTS_OCD, "HKVariants.ocd");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(HKVARIANTS_PHRASES_OCD, "HKVariantsPhrases.ocd");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(HKVARIANTS_REV_OCD, "HKVariantsRev.ocd");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(HKVARIANTS_REV_PHRASES_OCD, "HKVariantsRevPhrases.ocd");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(JPVARIANTS_OCD, "JPVariants.ocd");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(S2HK_JSON, "s2hk.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(S2T_JSON, "s2t.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(S2TW_JSON, "s2tw.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(S2TWP_JSON, "s2twp.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(STCHARACTERS_OCD, "STCharacters.ocd");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(STPHRASES_OCD, "STPhrases.ocd");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(T2HK_JSON, "t2hk.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(T2S_JSON, "t2s.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(T2TW_JSON, "t2tw.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(TSCHARACTERS_OCD, "TSCharacters.ocd");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(TSPHRASES_OCD, "TSPhrases.ocd");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(TW2S_JSON, "tw2s.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(TW2SP_JSON, "tw2sp.json");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(TWPHRASES_OCD, "TWPhrases.ocd");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(TWPHRASES_REV_OCD, "TWPhrasesRev.ocd");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(TWVARIANTS_OCD, "TWVariants.ocd");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(TWVARIANTS_REV_OCD, "TWVariantsRev.ocd");
#[cfg(feature = "static-dictionaries")]
new_sd_instance!(TWVARIANTS_REV_PHRASES_OCD, "TWVariantsRevPhrases.ocd");

/// Default configs.
#[derive(Copy, Clone)]
pub enum DefaultConfig {
    /// Simplified Chinese to Traditional Chinese
    S2T,
    /// Traditional Chinese to Simplified Chinese
    T2S,
    /// Simplified Chinese to Traditional Chinese (Taiwan Standard)
    S2TW,
    /// Traditional Chinese (Taiwan Standard) to Simplified Chinese
    TW2S,
    /// Simplified Chinese to Traditional Chinese (Hong Kong Standard)
    S2HK,
    /// Traditional Chinese (Hong Kong Standard) to Simplified Chinese
    HK2S,
    /// Simplified Chinese to Traditional Chinese (Taiwan Standard) with Taiwanese idiom
    S2TWP,
    /// Traditional Chinese (Taiwan Standard) to Simplified Chinese with Mainland Chinese idiom
    TW2SP,
    /// Traditional Chinese (OpenCC Standard) to Taiwan Standard
    T2TW,
    /// Traditional Chinese (OpenCC Standard) to Hong Kong Standard
    T2HK,
}

impl DefaultConfig {
    /// Get the file name for this default config.
    pub fn get_file_name(&self) -> &'static str {
        match self {
            DefaultConfig::S2T => "s2t.json",
            DefaultConfig::T2S => "t2s.json",
            DefaultConfig::S2TW => "s2tw.json",
            DefaultConfig::TW2S => "tw2s.json",
            DefaultConfig::S2HK => "s2hk.json",
            DefaultConfig::HK2S => "hk2s.json",
            DefaultConfig::S2TWP => "s2twp.json",
            DefaultConfig::TW2SP => "tw2sp.json",
            DefaultConfig::T2TW => "t2tw.json",
            DefaultConfig::T2HK => "t2hk.json",
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

unsafe impl Send for OpenCC {}

unsafe impl Sync for OpenCC {}

impl OpenCC {
    /// Create a new OpenCC instance through a file provided by its path.
    pub fn new<P: AsRef<Path>>(config_file_path: P) -> Result<OpenCC, &'static str> {
        let config_file_path = CString::new(config_file_path.as_ref().as_os_str().to_str().unwrap()).unwrap();

        let opencc = unsafe {
            opencc_open(config_file_path.as_ptr())
        };

        let v: size_t = unsafe {
            transmute(opencc)
        };
        if v == !0 {
            return Err("Cannot use this config file path.");
        }

        Ok(OpenCC {
            opencc
        })
    }

    /// Convert a string to another string.
    pub fn convert<S: AsRef<str>>(&self, input: S) -> String {
        let input = input.as_ref();

        let length = input.len();
        let input = CString::new(input).unwrap();

        let result_ptr = unsafe {
            opencc_convert_utf8(self.opencc, input.as_ptr(), length)
        };
        let result_cstr = unsafe {
            CStr::from_ptr(result_ptr)
        };
        let result = result_cstr.to_string_lossy().to_string();

        unsafe {
            opencc_convert_utf8_free(result_ptr);
        }

        result
    }

    /// Convert a string to another string and store into a buffer.
    pub fn convert_to_buffer<S: AsRef<str>>(&self, input: S, output: String) -> String {
        let input = input.as_ref();

        let length = input.len();
        let input = CString::new(input).unwrap();

        let mut output = output.into_bytes();
        let o_len = output.len();

        output.reserve(length * 2);

        let input_ptr = unsafe {
            output.as_ptr().add(output.len()) as *mut c_char
        };

        let size = unsafe {
            opencc_convert_utf8_to_buffer(self.opencc, input.as_ptr(), length, input_ptr)
        };

        unsafe {
            output.set_len(o_len + size);
        }

        unsafe {
            String::from_utf8_unchecked(output)
        }
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
fn generate_static_dictionary_inner<P: AsRef<Path>>(path: P, config: DefaultConfig) -> Result<(), &'static str> {
    let path = path.as_ref();

    let mut output_data: Vec<&SD> = Vec::new();

    match config {
        DefaultConfig::S2T => {
            output_data.push(&S2T_JSON);
            output_data.push(&STPHRASES_OCD);
            output_data.push(&STCHARACTERS_OCD);
        }
        DefaultConfig::T2S => {
            output_data.push(&T2S_JSON);
            output_data.push(&TSPHRASES_OCD);
            output_data.push(&TSCHARACTERS_OCD);
        }
        DefaultConfig::S2TW => {
            output_data.push(&S2TW_JSON);
            output_data.push(&STPHRASES_OCD);
            output_data.push(&STCHARACTERS_OCD);
            output_data.push(&TWVARIANTS_OCD);
        }
        DefaultConfig::TW2S => {
            output_data.push(&TW2S_JSON);
            output_data.push(&TSPHRASES_OCD);
            output_data.push(&TSCHARACTERS_OCD);
            output_data.push(&TWVARIANTS_REV_PHRASES_OCD);
            output_data.push(&TWVARIANTS_REV_OCD);
        }
        DefaultConfig::S2HK => {
            output_data.push(&S2HK_JSON);
            output_data.push(&STPHRASES_OCD);
            output_data.push(&STCHARACTERS_OCD);
            output_data.push(&HKVARIANTS_PHRASES_OCD);
            output_data.push(&HKVARIANTS_OCD);
        }
        DefaultConfig::HK2S => {
            output_data.push(&HK2S_JSON);
            output_data.push(&TSPHRASES_OCD);
            output_data.push(&TSCHARACTERS_OCD);
            output_data.push(&HKVARIANTS_REV_PHRASES_OCD);
            output_data.push(&HKVARIANTS_REV_OCD);
        }
        DefaultConfig::S2TWP => {
            output_data.push(&S2TWP_JSON);
            output_data.push(&STPHRASES_OCD);
            output_data.push(&STCHARACTERS_OCD);
            output_data.push(&TWPHRASES_OCD);
            output_data.push(&TWVARIANTS_OCD);
        }
        DefaultConfig::TW2SP => {
            output_data.push(&TW2SP_JSON);
            output_data.push(&TSPHRASES_OCD);
            output_data.push(&TSCHARACTERS_OCD);
            output_data.push(&TWVARIANTS_REV_PHRASES_OCD);
            output_data.push(&TWVARIANTS_OCD);
            output_data.push(&TWPHRASES_REV_OCD);
        }
        DefaultConfig::T2TW => {
            output_data.push(&T2TW_JSON);
            output_data.push(&TWVARIANTS_OCD);
        }
        DefaultConfig::T2HK => {
            output_data.push(&T2HK_JSON);
            output_data.push(&HKVARIANTS_OCD);
        }
    }

    for data in output_data {
        let output_path = Path::join(&path, Path::new(data.0));
        if output_path.exists() {
            if output_path.is_file() {
                continue;
            } else {
                return Err("The dictionary is not correct.");
            }
        }
        let mut file = match File::create(output_path) {
            Ok(file) => file,
            Err(_) => return Err("Cannot create a new file.")
        };

        if let Err(_) = file.write(data.1) {
            return Err("Cannot write data to a file.");
        }

        if let Err(_) = file.flush() {
            return Err("Cannot flush file.");
        }
    }

    Ok(())
}

#[cfg(feature = "static-dictionaries")]
/// Generate files for a specific dictionary. These files are used for opening a new OpenCC instance.
pub fn generate_static_dictionary<P: AsRef<Path>>(path: P, config: DefaultConfig) -> Result<(), &'static str> {
    let path = path.as_ref();

    if path.exists() {
        if !path.is_dir() {
            return Err("The path of static dictionaries needs to be a directory.");
        }
    } else {
        match fs::create_dir_all(path) {
            Ok(_) => (),
            Err(_) => return Err("Cannot create new directories.")
        }
    }

    generate_static_dictionary_inner(path, config)
}

#[cfg(feature = "static-dictionaries")]
/// Generate files for specific dictionaries. These files are used for opening a new OpenCC instance.
pub fn generate_static_dictionaries<P: AsRef<Path>>(path: P, configs: &[DefaultConfig]) -> Result<(), &'static str> {
    let path = path.as_ref();

    if path.exists() {
        if !path.is_dir() {
            return Err("The path of static dictionaries needs to be a directory.");
        }
    } else {
        match fs::create_dir_all(path) {
            Ok(_) => (),
            Err(_) => return Err("Cannot create new directories.")
        }
    }

    for config in configs {
        generate_static_dictionary_inner(path, config.clone())?
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "static-dictionaries")]
    use std::env;

    #[test]
    fn test_tw2sp() {
        let opencc = OpenCC::new(DefaultConfig::TW2SP).unwrap();
        assert_eq!("凉风有讯，秋月无边，亏我思娇的情绪好比度日如年。虽然我不是玉树临风，潇洒倜傥，但我有广阔的胸襟，加强劲的臂弯。",
                   &opencc.convert("涼風有訊，秋月無邊，虧我思嬌的情緒好比度日如年。雖然我不是玉樹臨風，瀟灑倜儻，但我有廣闊的胸襟，加強勁的臂彎。"));
    }

    #[test]
    fn test_tw2sp_to_buffer() {
        let s = String::from("涼風有訊，秋月無邊，虧我思嬌的情緒好比度日如年。");

        let opencc = OpenCC::new(DefaultConfig::TW2SP).unwrap();
        let s = opencc.convert_to_buffer("雖然我不是玉樹臨風，瀟灑倜儻，但我有廣闊的胸襟，加強勁的臂彎。", s);

        assert_eq!("涼風有訊，秋月無邊，虧我思嬌的情緒好比度日如年。虽然我不是玉树临风，潇洒倜傥，但我有广阔的胸襟，加强劲的臂弯。",
                   &s);
    }

    #[test]
    fn test_s2twp() {
        let opencc = OpenCC::new(DefaultConfig::S2TWP).unwrap();
        assert_eq!("涼風有訊，秋月無邊，虧我思嬌的情緒好比度日如年。雖然我不是玉樹臨風，瀟灑倜儻，但我有廣闊的胸襟，加強勁的臂彎。",
                   &opencc.convert("凉风有讯，秋月无边，亏我思娇的情绪好比度日如年。虽然我不是玉树临风，潇洒倜傥，但我有广阔的胸襟，加强劲的臂弯。"));
    }

    #[test]
    fn test_s2twp_to_buffer() {
        let s = String::from("凉风有讯，秋月无边，亏我思娇的情绪好比度日如年。");

        let opencc = OpenCC::new(DefaultConfig::S2TWP).unwrap();
        let s = opencc.convert_to_buffer("虽然我不是玉树临风，潇洒倜傥，但我有广阔的胸襟，加强劲的臂弯。", s);

        assert_eq!("凉风有讯，秋月无边，亏我思娇的情绪好比度日如年。雖然我不是玉樹臨風，瀟灑倜儻，但我有廣闊的胸襟，加強勁的臂彎。",
                   &s);
    }

    #[cfg(feature = "static-dictionaries")]
    #[test]
    fn test_generate_static_dictionary() {
        let cwd = env::current_dir().unwrap();

        let output_path = Path::join(&cwd, "dict_output");

        generate_static_dictionary(&output_path, DefaultConfig::TW2SP).unwrap();

        let s = String::from("無");

        let opencc = OpenCC::new(Path::join(&output_path, DefaultConfig::TW2SP)).unwrap();

        assert_eq!("无", &opencc.convert(&s));
    }
}