use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    process::Command,
};
fn files(dir: &Path, out: &mut Vec<PathBuf>) {
    for item in std::fs::read_dir(dir).unwrap() {
        let p = item.unwrap().path();
        if p.is_dir() {
            files(&p, out)
        } else if p.extension().is_some_and(|e| e == "rs") {
            out.push(p);
        }
    }
}
fn main() {
    let root = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let mut paths = Vec::new();
    files(&root.join("src"), &mut paths);
    for name in [
        "Cargo.toml",
        "Cargo.lock",
        "build.rs",
        "codegen.json",
        "src/generated/manifest.json",
        "../tools/native_rust_codegen.py",
    ] {
        let p = root.join(name);
        if p.exists() {
            paths.push(p);
        }
    }
    let mut identities = BTreeMap::new();
    for p in paths {
        println!("cargo:rerun-if-changed={}", p.display());
        let result = Command::new("sha256sum")
            .arg(&p)
            .output()
            .expect("sha256sum");
        assert!(result.status.success());
        identities.insert(
            p.strip_prefix(&root)
                .unwrap_or(&p)
                .to_string_lossy()
                .to_string(),
            String::from_utf8_lossy(&result.stdout)
                .split_whitespace()
                .next()
                .unwrap()
                .to_string(),
        );
    }
    println!("cargo:rerun-if-changed=src");
    let rust = Command::new(std::env::var_os("RUSTC").unwrap())
        .arg("--version")
        .output()
        .unwrap();
    let mut build_settings = BTreeMap::new();
    for name in [
        "CARGO_ENCODED_RUSTFLAGS",
        "CARGO_PROFILE_RELEASE_LTO",
        "CARGO_PROFILE_RELEASE_CODEGEN_UNITS",
        "CLASH_PGO_SHA256",
    ] {
        println!("cargo:rerun-if-env-changed={name}");
        if let Ok(value) = std::env::var(name) {
            build_settings.insert(name, value);
        }
    }
    let record = serde_json::json!({"compiler":String::from_utf8_lossy(&rust.stdout).trim(),"profile":std::env::var("PROFILE").unwrap(),"opt_level":std::env::var("OPT_LEVEL").unwrap(),"target":std::env::var("TARGET").unwrap(),"build_settings":build_settings,"source_files_sha256":identities});
    let out = PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("provenance.json");
    std::fs::write(out, serde_json::to_vec_pretty(&record).unwrap()).unwrap();
}
