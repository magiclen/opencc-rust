#![cfg(feature = "static-dictionaries")]

use std::{collections::BTreeSet, fs, path::Path};

use enum_ordinalize::Ordinalize;
use opencc_rust::{DefaultConfig, OpenCC};
use serde_json::Value;
use tempfile::tempdir;

#[test]
fn generate_static_dictionary() {
    let output_path = tempdir().unwrap();

    opencc_rust::generate_static_dictionary(output_path.path(), DefaultConfig::TW2SP).unwrap();

    let s = String::from("無");

    let opencc = OpenCC::new(output_path.path().join(DefaultConfig::TW2SP)).unwrap();

    assert_eq!("无", &opencc.convert(s).unwrap());
}

#[test]
fn generate_static_dictionaries_with_json_dependencies() {
    for config in DefaultConfig::VARIANTS.iter().copied() {
        let output_path = tempdir().unwrap();

        opencc_rust::generate_static_dictionary(output_path.path(), config).unwrap();

        assert_eq!(
            expected_generated_files(config),
            generated_files(output_path.path()),
            "{} should be generated with the OCD2 files referenced by its JSON config",
            config.get_file_name(),
        );

        OpenCC::new(output_path.path().join(config)).unwrap();
    }
}

fn expected_generated_files(config: DefaultConfig) -> BTreeSet<String> {
    let json_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("opencc").join(config.get_file_name());
    let value: Value = serde_json::from_str(&fs::read_to_string(json_path).unwrap()).unwrap();
    let mut files = BTreeSet::from([config.get_file_name().to_string()]);

    collect_ocd2_file_names(&value, &mut files);

    files
}

fn generated_files(path: &Path) -> BTreeSet<String> {
    fs::read_dir(path)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect()
}

// OpenCC config JSON can nest dictionary files inside groups, arrays, and conversion chains.
fn collect_ocd2_file_names(value: &Value, files: &mut BTreeSet<String>) {
    match value {
        Value::Array(values) => {
            for value in values {
                collect_ocd2_file_names(value, files);
            }
        },
        Value::Object(map) => {
            if let Some(file_name) = map.get("file").and_then(Value::as_str)
                && file_name.ends_with(".ocd2")
            {
                files.insert(file_name.to_string());
            }

            for value in map.values() {
                collect_ocd2_file_names(value, files);
            }
        },
        _ => {},
    }
}
