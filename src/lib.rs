extern crate libc;

use libc::{c_int, size_t, c_void, c_char};
use libc::wchar_t;

use std::ffi::{CStr, CString};
use std::mem::transmute;

#[link(name = "opencc")]
extern "C" {
    pub fn opencc_open(config_file_path: *const c_char) -> *mut c_void;
    pub fn opencc_open_w(config_file_path: *const wchar_t) -> *mut c_void;
    pub fn opencc_close(opencc: *mut c_void) -> c_int;
    pub fn opencc_convert_utf8(opencc: *mut c_void, input: *const c_char, length: size_t) -> *mut c_char;
    pub fn opencc_convert_utf8_to_buffer(opencc: *mut c_void, input: *const c_char, length: size_t, output: *mut c_char) -> size_t;
    pub fn opencc_convert_utf8_free(str: *mut c_char);
    pub fn opencc_error() -> *const c_char;
}

#[cfg(not(feature = "no-default-config-file-names"))]
/// Default config file names.
pub mod default_config_file_paths {
    /// Simplified Chinese to Traditional Chinese
    pub const S2T: &'static str = "s2t.json";
    /// Traditional Chinese to Simplified Chinese
    pub const T2S: &'static str = "t2s.json";
    /// Simplified Chinese to Traditional Chinese (Taiwan Standard)
    pub const S2TW: &'static str = "s2tw.json";
    /// Traditional Chinese (Taiwan Standard) to Simplified Chinese
    pub const TW2S: &'static str = "tw2s.json";
    /// Simplified Chinese to Traditional Chinese (Hong Kong Standard)
    pub const S2HK: &'static str = "s2hk.json";
    /// Traditional Chinese (Hong Kong Standard) to Simplified Chinese
    pub const HK2S: &'static str = "hk2s.json";
    /// Simplified Chinese to Traditional Chinese (Taiwan Standard) with Taiwanese idiom
    pub const S2TWP: &'static str = "s2twp.json";
    /// Traditional Chinese (Taiwan Standard) to Simplified Chinese with Mainland Chinese idiom
    pub const TW2SP: &'static str = "tw2sp.json";
    /// Traditional Chinese (OpenCC Standard) to Taiwan Standard
    pub const T2TW: &'static str = "t2tw.json";
    /// Traditional Chinese (OpenCC Standard) to Hong Kong Standard
    pub const T2HK: &'static str = "t2hk.json";
}

pub struct OpenCC {
    opencc: *mut c_void,
}

unsafe impl Send for OpenCC {}

unsafe impl Sync for OpenCC {}

impl OpenCC {
    pub fn new(config_file_path: &str) -> Result<OpenCC, &'static str> {
        let config_file_path = CString::new(config_file_path).unwrap();

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

    pub fn convert(&self, input: &str) -> String {
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

    pub fn convert_owned(&self, input: &str, output: String) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tw2sp() {
        let opencc = OpenCC::new(default_config_file_paths::TW2SP).unwrap();
        assert_eq!("凉风有讯，秋月无边，亏我思娇的情绪好比度日如年。虽然我不是玉树临风，潇洒倜傥，但我有广阔的胸襟，加强劲的臂弯。",
                   &opencc.convert("涼風有訊，秋月無邊，虧我思嬌的情緒好比度日如年。雖然我不是玉樹臨風，瀟灑倜儻，但我有廣闊的胸襟，加強勁的臂彎。"));
    }

    #[test]
    fn test_tw2sp_owned() {
        let s = String::from("涼風有訊，秋月無邊，虧我思嬌的情緒好比度日如年。");

        let opencc = OpenCC::new(default_config_file_paths::TW2SP).unwrap();
        let s = opencc.convert_owned("雖然我不是玉樹臨風，瀟灑倜儻，但我有廣闊的胸襟，加強勁的臂彎。", s);

        assert_eq!("涼風有訊，秋月無邊，虧我思嬌的情緒好比度日如年。虽然我不是玉树临风，潇洒倜傥，但我有广阔的胸襟，加强劲的臂弯。",
                   &s);
    }

    #[test]
    fn test_s2twp() {
        let opencc = OpenCC::new(default_config_file_paths::S2TWP).unwrap();
        assert_eq!("涼風有訊，秋月無邊，虧我思嬌的情緒好比度日如年。雖然我不是玉樹臨風，瀟灑倜儻，但我有廣闊的胸襟，加強勁的臂彎。",
                   &opencc.convert("凉风有讯，秋月无边，亏我思娇的情绪好比度日如年。虽然我不是玉树临风，潇洒倜傥，但我有广阔的胸襟，加强劲的臂弯。"));
    }

    #[test]
    fn test_s2twp_owned() {
        let s = String::from("凉风有讯，秋月无边，亏我思娇的情绪好比度日如年。");

        let opencc = OpenCC::new(default_config_file_paths::S2TWP).unwrap();
        let s = opencc.convert_owned("虽然我不是玉树临风，潇洒倜傥，但我有广阔的胸襟，加强劲的臂弯。", s);

        assert_eq!("凉风有讯，秋月无边，亏我思娇的情绪好比度日如年。雖然我不是玉樹臨風，瀟灑倜儻，但我有廣闊的胸襟，加強勁的臂彎。",
                   &s);
    }
}