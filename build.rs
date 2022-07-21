use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
use std::process::Command;

const MIN_VERSION: &str = "1.1.2";
const MAX_VERSION: &str = "1.2.0";

fn main() {
    if cfg!(target_os = "freebsd") {
        env_var_set_default("OPENCC_INCLUDE_DIRS", "/usr/include/opencc");
        env_var_set_default("OPENCC_LIB_DIRS", "/usr/lib");
        env_var_set_default("OPENCC_LIBS", "opencc");
    }

    let lib_dirs = find_opencc_lib_dirs();

    for d in &lib_dirs {
        if !d.exists() {
            panic!("OpenCC library directory does not exist: {}", d.to_string_lossy());
        }
        println!("cargo:rustc-link-search=native={}", d.to_string_lossy());
    }

    let include_dirs = find_opencc_include_dirs();
    for d in &include_dirs {
        if !d.exists() {
            panic!("OpenCC include directory does not exist: {}", d.to_string_lossy());
        }
        println!("cargo:include={}", d.to_string_lossy());
    }
    println!("cargo:rerun-if-env-changed=OPENCC_LIBS");

    let target = env::var("TARGET").unwrap();
    let libs_env = env::var("OPENCC_LIBS").ok();

    let libs = match libs_env {
        Some(ref v) => v.split(':').map(|x| x.to_owned()).collect(),
        None => {
            #[allow(clippy::if_same_then_else)]
            if target.contains("windows") {
                vec!["opencc".to_string()] // TODO: not sure
            } else if target.contains("freebsd") {
                vec!["opencc".to_string()]
            } else {
                run_pkg_config().libs
            }
        }
    };

    let kind = determine_mode(&lib_dirs, libs.as_slice());
    for lib in libs.into_iter() {
        println!("cargo:rustc-link-lib={}={}", kind, lib);
    }

    println!("cargo:rerun-if-env-changed=OPENCC_DYLIB_STDCPP");
    let kind = env::var("OPENCC_DYLIB_STDCPP");
    match kind.as_ref().map(|s| &s[..]) {
        Ok("0") => (),
        Ok(_) => println!("cargo:rustc-link-lib=dylib=stdc++"),
        Err(_) => (),
    }

    println!("cargo:rerun-if-env-changed=OPENCC_STATIC_STDCPP");
    let kind = env::var("OPENCC_STATIC_STDCPP");
    match kind.as_ref().map(|s| &s[..]) {
        Ok("0") => (),
        Ok(_) => println!("cargo:rustc-link-lib=static=stdc++"),
        Err(_) => (),
    }
}

fn env_var_set_default(name: &str, value: &str) {
    if env::var(name).is_err() {
        env::set_var(name, value);
    }
}

fn find_opencc_lib_dirs() -> Vec<PathBuf> {
    println!("cargo:rerun-if-env-changed=OPENCC_LIB_DIRS");
    env::var("OPENCC_LIB_DIRS")
        .map(|x| x.split(':').map(PathBuf::from).collect::<Vec<PathBuf>>())
        .or_else(|_| Ok(vec![find_opencc_dir()?.join("lib")]))
        .or_else(|_: env::VarError| -> Result<_, env::VarError> { Ok(run_pkg_config().link_paths) })
        .expect("Couldn't find OpenCC library directory")
}

fn find_opencc_include_dirs() -> Vec<PathBuf> {
    println!("cargo:rerun-if-env-changed=OPENCC_INCLUDE_DIRS");
    env::var("OPENCC_INCLUDE_DIRS")
        .map(|x| x.split(':').map(PathBuf::from).collect::<Vec<PathBuf>>())
        .or_else(|_| Ok(vec![find_opencc_dir()?.join("include")]))
        .or_else(|_: env::VarError| -> Result<_, env::VarError> {
            Ok(run_pkg_config().include_paths)
        })
        .expect("Couldn't find OpenCC include directory")
}

fn find_opencc_dir() -> Result<PathBuf, env::VarError> {
    println!("cargo:rerun-if-env-changed=OPENCC_DIR");
    env::var("OPENCC_DIR").map(PathBuf::from)
}

fn determine_mode<T: AsRef<str>>(libdirs: &[PathBuf], libs: &[T]) -> &'static str {
    println!("cargo:rerun-if-env-changed=OPENCC_STATIC");
    let kind = env::var("OPENCC_STATIC").ok();
    match kind.as_ref().map(|s| &s[..]) {
        Some("0") => return "dylib",
        Some(_) => return "static",
        None => {}
    }

    let files = libdirs
        .iter()
        .flat_map(|d| d.read_dir().unwrap())
        .map(|e| e.unwrap())
        .map(|e| e.file_name())
        .filter_map(|e| e.into_string().ok())
        .collect::<HashSet<_>>();
    let can_static = libs.iter().all(|l| {
        files.contains(&format!("lib{}.a", l.as_ref()))
            || files.contains(&format!("{}.lib", l.as_ref()))
    });
    let can_dylib = libs.iter().all(|l| {
        files.contains(&format!("lib{}.so", l.as_ref()))
            || files.contains(&format!("{}.dll", l.as_ref()))
            || files.contains(&format!("lib{}.dylib", l.as_ref()))
    });

    match (can_static, can_dylib) {
        (true, false) => return "static",
        (false, true) => return "dylib",
        (false, false) => {
            panic!(
                "OpenCC libdirs at `{:?}` do not contain the required files \
                 to either statically or dynamically link OpenCC",
                libdirs
            );
        }
        (true, true) => {}
    }

    "dylib"
}

fn run_pkg_config() -> pkg_config::Library {
    pkg_config::Config::new()
        .cargo_metadata(false)
        .atleast_version(MIN_VERSION)
        .probe("opencc")
        .unwrap();

    if !Command::new("pkg-config")
        .arg(format!("--max-version={}", MAX_VERSION))
        .arg("opencc")
        .status()
        .unwrap()
        .success()
    {
        panic!("OpenCC version must be no higher than {}", MAX_VERSION);
    }

    pkg_config::Config::new().cargo_metadata(false).probe("opencc").unwrap()
}
