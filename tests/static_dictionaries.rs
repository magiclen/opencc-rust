#![cfg(feature = "static-dictionaries")]

extern crate opencc_rust;

use std::env;
use std::path::Path;

use opencc_rust::{DefaultConfig, OpenCC};

#[test]
fn generate_static_dictionary() {
    let cwd = env::current_dir().unwrap();

    let output_path = Path::join(&cwd, "dict_output");

    opencc_rust::generate_static_dictionary(&output_path, DefaultConfig::TW2SP).unwrap();

    let s = String::from("無");

    let opencc = OpenCC::new(Path::join(&output_path, DefaultConfig::TW2SP)).unwrap();

    assert_eq!("无", &opencc.convert(&s));
}
