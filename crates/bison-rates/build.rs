use std::process::Command;

fn main() {
    let hash_out = Command::new("git")
        .args(&["rev-parse", "--short=8", "HEAD"])
        .output()
        .unwrap();

    let git_hash = String::from_utf8(hash_out.stdout).unwrap();

    let status_out = Command::new("git")
        .args(&["status", "--short", "--untracked-files=no"])
        .output()
        .unwrap();
    let git_status = String::from_utf8(status_out.stdout).unwrap();

    if git_status.trim().is_empty() {
        println!("cargo:rustc-env=GIT_HASH={}", git_hash.trim());
    } else {
        println!("cargo:rustc-env=GIT_HASH={}-dirty", git_hash.trim());
    }

    match std::env::var("BISON_ENV")
        .unwrap_or_else(|_| "release".to_string())
        .as_str()
    {
        "dev" | "d" => println!("cargo:rustc-env=ENV=dev-"),
        "prod" | "p" => println!("cargo:rustc-env=ENV=prod-"),
        _ => println!("cargo:rustc-env=ENV="),
    }
}
