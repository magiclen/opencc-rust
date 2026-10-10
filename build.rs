use std::{collections::HashSet, env, path::PathBuf, sync::OnceLock};

const MIN_VERSION: &str = "1.4.0";
const MAX_VERSION: &str = "1.5.0";

fn main() {
    println!("cargo:rustc-check-cfg=cfg(docsrs)");

    if env::var("DOCS_RS").is_ok() {
        return;
    }

    if cfg!(target_os = "freebsd") {
        env_var_set_default("OPENCC_INCLUDE_DIRS", "/usr/include/opencc");
        env_var_set_default("OPENCC_LIB_DIRS", "/usr/lib");
        env_var_set_default("OPENCC_LIBS", "opencc");
    }

    let lib_dirs = find_opencc_lib_dirs();

    for d in &lib_dirs {
        if !d.exists() {
            panic!(
                "OpenCC library directory does not exist: {}. Set OPENCC_LIB_DIRS or OPENCC_DIR \
                 to the correct OpenCC location.",
                d.to_string_lossy()
            );
        }
        println!("cargo:rustc-link-search=native={}", d.to_string_lossy());
    }

    let include_dirs = find_opencc_include_dirs();
    for d in &include_dirs {
        if !d.exists() {
            panic!(
                "OpenCC include directory does not exist: {}. Set OPENCC_INCLUDE_DIRS or \
                 OPENCC_DIR to the correct OpenCC location.",
                d.to_string_lossy()
            );
        }
        println!("cargo:include={}", d.to_string_lossy());
    }
    println!("cargo:rerun-if-env-changed=OPENCC_LIBS");

    let target = env::var("TARGET").expect("Cargo did not provide TARGET to the build script.");
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
                run_pkg_config().libs.clone()
            }
        },
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
        unsafe {
            env::set_var(name, value);
        }
    }
}

fn find_opencc_lib_dirs() -> Vec<PathBuf> {
    println!("cargo:rerun-if-env-changed=OPENCC_LIB_DIRS");

    let sep = if cfg!(target_os = "windows") { ";" } else { ":" };

    env::var("OPENCC_LIB_DIRS")
        .map(|x| x.split(sep).map(PathBuf::from).collect::<Vec<PathBuf>>())
        .or_else(|_| Ok(vec![find_opencc_dir()?.join("lib")]))
        .or_else(|_: env::VarError| -> Result<_, env::VarError> {
            Ok(run_pkg_config().link_paths.clone())
        })
        .expect(
            "Couldn't find OpenCC library directory. Set OPENCC_LIB_DIRS, OPENCC_DIR, or make \
             pkg-config find OpenCC.",
        )
}

fn find_opencc_include_dirs() -> Vec<PathBuf> {
    println!("cargo:rerun-if-env-changed=OPENCC_INCLUDE_DIRS");

    let sep = if cfg!(target_os = "windows") { ";" } else { ":" };

    env::var("OPENCC_INCLUDE_DIRS")
        .map(|x| x.split(sep).map(PathBuf::from).collect::<Vec<PathBuf>>())
        .or_else(|_| Ok(vec![find_opencc_dir()?.join("include")]))
        .or_else(|_: env::VarError| -> Result<_, env::VarError> {
            Ok(run_pkg_config().include_paths.clone())
        })
        .expect(
            "Couldn't find OpenCC include directory. Set OPENCC_INCLUDE_DIRS, OPENCC_DIR, or make \
             pkg-config find OpenCC.",
        )
}

fn find_opencc_dir() -> Result<PathBuf, env::VarError> {
    println!("cargo:rerun-if-env-changed=OPENCC_DIR");
    env::var("OPENCC_DIR").map(PathBuf::from)
}

fn opencc_static() -> Option<bool> {
    println!("cargo:rerun-if-env-changed=OPENCC_STATIC");
    env::var("OPENCC_STATIC").ok().map(|kind| kind != "0")
}

fn determine_mode<T: AsRef<str>>(libdirs: &[PathBuf], libs: &[T]) -> &'static str {
    match opencc_static() {
        Some(true) => return "static",
        Some(false) => return "dylib",
        None => {},
    }

    let files = libdirs
        .iter()
        .flat_map(|d| {
            d.read_dir().unwrap_or_else(|error| {
                panic!("Cannot read OpenCC library directory {}: {}", d.to_string_lossy(), error)
            })
        })
        .map(|e| e.expect("Cannot read an entry from the OpenCC library directory."))
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
                "OpenCC libdirs at `{:?}` do not contain libraries for {:?}. Set OPENCC_LIBS or \
                 check OPENCC_LIB_DIRS.",
                libdirs,
                libs.iter().map(|l| l.as_ref()).collect::<Vec<_>>()
            );
        },
        (true, true) => {},
    }

    "dylib"
}

fn run_pkg_config() -> &'static pkg_config::Library {
    static LIBRARY: OnceLock<pkg_config::Library> = OnceLock::new();

    LIBRARY.get_or_init(|| {
        let mut config = pkg_config::Config::new();

        config.cargo_metadata(false).range_version(MIN_VERSION..=MAX_VERSION);

        if let Some(statik) = opencc_static() {
            config.statik(statik);
        }

        config.probe("opencc").expect(
            "pkg-config could not find OpenCC in the supported version range. Set OPENCC_DIR, \
             OPENCC_LIB_DIRS, OPENCC_INCLUDE_DIRS, and OPENCC_LIBS if OpenCC is not installed for \
             pkg-config.",
        )
    })
}
