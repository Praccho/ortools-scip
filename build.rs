use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

const ORTOOLS_TAG: &str = "v9.15";
const ORTOOLS_BUILD: &str = "v9.15.6755";

fn ortools_url(target_os: &str, target_arch: &str) -> String {
    let asset_name = match (target_os, target_arch) {
        ("macos", "aarch64") => format!("or-tools_arm64_macOS-26.2_cpp_{ORTOOLS_BUILD}.tar.gz"),
        ("macos", "x86_64") => format!("or-tools_x86_64_macOS-26.2_cpp_{ORTOOLS_BUILD}.tar.gz"),
        ("linux", "x86_64") => {
            format!("or-tools_amd64_ubuntu-22.04_cpp_{ORTOOLS_BUILD}.tar.gz")
        }
        ("linux", "aarch64") => {
            format!("or-tools_aarch64_AlmaLinux-8.10_cpp_{ORTOOLS_BUILD}.tar.gz")
        }
        _ => panic!(
            "Unsupported platform: {target_os}/{target_arch}. \
             Set ORTOOLS_DIR to a manually downloaded OR-Tools C++ distribution."
        ),
    };
    format!("https://github.com/google/or-tools/releases/download/{ORTOOLS_TAG}/{asset_name}")
}

fn extracted_dir_name(target_os: &str, target_arch: &str) -> String {
    match (target_os, target_arch) {
        ("macos", "aarch64") => format!("or-tools_arm64_macOS-26.2_cpp_{ORTOOLS_BUILD}"),
        ("macos", "x86_64") => format!("or-tools_x86_64_macOS-26.2_cpp_{ORTOOLS_BUILD}"),
        ("linux", "x86_64") => format!("or-tools_amd64_ubuntu-22.04_cpp_{ORTOOLS_BUILD}"),
        ("linux", "aarch64") => format!("or-tools_aarch64_AlmaLinux-8.10_cpp_{ORTOOLS_BUILD}"),
        _ => unreachable!(),
    }
}

fn cache_dir() -> PathBuf {
    let home = env::var("HOME").expect("HOME not set");
    PathBuf::from(home)
        .join(".cache")
        .join("ortools-scip")
        .join(ORTOOLS_BUILD)
}

fn target_info() -> (String, String) {
    let target = env::var("TARGET").unwrap();
    let os = if target.contains("apple") || target.contains("darwin") {
        "macos"
    } else if target.contains("linux") {
        "linux"
    } else {
        panic!("Unsupported target OS in: {target}")
    };
    let arch = if target.contains("aarch64") || target.contains("arm64") {
        "aarch64"
    } else if target.contains("x86_64") {
        "x86_64"
    } else {
        panic!("Unsupported target arch in: {target}")
    };
    (os.to_string(), arch.to_string())
}

fn download_ortools(ortools_dir: &Path, target_os: &str, target_arch: &str) {
    let cache = cache_dir();
    std::fs::create_dir_all(&cache).expect("Failed to create cache directory");

    let url = ortools_url(target_os, target_arch);
    let tarball = cache.join("ortools.tar.gz");

    if !ortools_dir.exists() {
        eprintln!("cargo:warning=Downloading OR-Tools from {url}");
        eprintln!("cargo:warning=This may take a few minutes on the first build...");

        let status = Command::new("curl")
            .args(["-fSL", "--progress-bar", "-o"])
            .arg(&tarball)
            .arg(&url)
            .status()
            .expect("Failed to run curl. Is curl installed?");
        if !status.success() {
            panic!("Failed to download OR-Tools from {url}");
        }

        eprintln!("cargo:warning=Extracting OR-Tools...");
        let status = Command::new("tar")
            .args(["xzf"])
            .arg(&tarball)
            .arg("-C")
            .arg(&cache)
            .status()
            .expect("Failed to run tar");
        if !status.success() {
            panic!("Failed to extract OR-Tools tarball");
        }

        std::fs::remove_file(&tarball).ok();

        let extracted = cache.join(extracted_dir_name(target_os, target_arch));
        if extracted != *ortools_dir && extracted.exists() {
            std::fs::rename(&extracted, ortools_dir)
                .expect("Failed to rename extracted directory");
        }
    }
}

fn find_ortools() -> PathBuf {
    if let Ok(dir) = env::var("ORTOOLS_DIR") {
        let p = PathBuf::from(&dir);
        if !p.join("include").exists() || !p.join("lib").exists() {
            panic!(
                "ORTOOLS_DIR={dir} does not contain include/ and lib/ subdirectories. \
                 Point it to the root of an extracted OR-Tools C++ distribution."
            );
        }
        return p;
    }

    let (target_os, target_arch) = target_info();
    let ortools_dir = cache_dir().join("ortools");
    download_ortools(&ortools_dir, &target_os, &target_arch);

    if !ortools_dir.join("include").exists() {
        panic!(
            "OR-Tools extraction failed: {} missing include/ directory",
            ortools_dir.display()
        );
    }
    ortools_dir
}

fn main() {
    println!("cargo:rerun-if-env-changed=ORTOOLS_DIR");
    println!("cargo:rerun-if-changed=cpp/ortools_shim.cc");

    let ortools = find_ortools();
    let include_dir = ortools.join("include");
    let lib_dir = ortools.join("lib");

    cc::Build::new()
        .cpp(true)
        .std("c++17")
        .file("cpp/ortools_shim.cc")
        .include(&include_dir)
        .define("OR_PROTO_DLL", "")
        .pic(true)
        .warnings(false)
        .compile("ortools_shim");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=ortools");

    // The C++ shim includes OR-Tools headers which inline abseil code.
    // We must link the abseil shared libraries so those symbols resolve.
    for entry in std::fs::read_dir(&lib_dir).expect("Failed to read lib dir") {
        let entry = entry.unwrap();
        let name = entry.file_name().to_string_lossy().to_string();
        let is_dylib = name.ends_with(".dylib") || name.ends_with(".so");
        if !is_dylib {
            continue;
        }
        // Skip versioned symlinks (e.g. libfoo.9.15.dylib, libfoo.so.1.2)
        // to avoid duplicate link directives. Only link the unversioned name.
        let stem = if name.ends_with(".dylib") {
            name.strip_suffix(".dylib").unwrap()
        } else {
            // For .so files, take everything before the first ".so"
            name.split(".so").next().unwrap()
        };
        // Skip if the stem still contains version numbers (e.g. "libfoo.9")
        if stem.contains('.') {
            continue;
        }
        let lib_name = stem.strip_prefix("lib").unwrap_or(stem);
        if lib_name == "ortools" {
            continue; // already linked above
        }
        println!("cargo:rustc-link-lib=dylib={lib_name}");
    }

    let target = env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=dylib=c++");
        println!(
            "cargo:rustc-link-arg=-Wl,-rpath,{}",
            lib_dir.display()
        );
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!(
            "cargo:rustc-link-arg=-Wl,-rpath,{}",
            lib_dir.display()
        );
    }
}
