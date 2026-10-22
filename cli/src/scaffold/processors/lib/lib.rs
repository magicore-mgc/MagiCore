//! Stable LibProcessor API and per-language scaffold dispatch.
//! API LibProcessor ổn định và dispatch scaffold theo ngôn ngữ.

use std::path::Path;

use anyhow::Result;

#[path = "dotnet/mod.rs"]
mod dotnet;
#[path = "go/mod.rs"]
mod go;
#[path = "java/mod.rs"]
mod java;
#[path = "python/mod.rs"]
mod python;
#[path = "rust_fallback/mod.rs"]
mod rust_fallback;
#[path = "typescript/mod.rs"]
mod typescript;

pub struct LibProcessor;

impl LibProcessor {
    pub fn files(target: &Path, name: &str, framework: &str) -> Result<()> {
        match framework {
            "ts" | "typescript" => typescript::write(target, name),
            "python" => python::write(target, name),
            "go" => go::write(target, name),
            "dotnet" => dotnet::write(target, name),
            "java" => java::write(target, name),
            // Preserve the current catch-all: unrecognized keys also scaffold Rust.
            _ => rust_fallback::write(target, name),
        }
    }
}
