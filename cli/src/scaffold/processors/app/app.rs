//! Stable AppProcessor API and framework dispatch.
//! API AppProcessor ổn định và điều phối framework.

use std::path::Path;

use anyhow::Result;

#[path = "error/mod.rs"]
mod error;
#[path = "flutter/mod.rs"]
mod flutter;
#[path = "kotlin/mod.rs"]
mod kotlin;
#[path = "maui/mod.rs"]
mod maui;
#[path = "multi/mod.rs"]
mod multi;
#[path = "react_native/mod.rs"]
mod react_native;
#[path = "swift/mod.rs"]
mod swift;
#[path = "tauri/mod.rs"]
mod tauri;

pub struct AppProcessor;

impl AppProcessor {
    pub fn files(target: &Path, name: &str, framework: &str) -> Result<()> {
        match framework {
            "kotlin" => kotlin::write(target, name),
            "swift" => swift::write(target, name),
            "maui" => maui::write(target, name),
            "tauri" => tauri::write(target, name),
            "react-native" => react_native::write(target, name),
            "flutter" => flutter::write(target, name),
            other => Err(error::AppScaffoldError::UnsupportedFramework {
                framework: other.to_owned(),
            }
            .into()),
        }
    }

    pub fn files_multi(target: &Path, name: &str) -> Result<()> {
        multi::write(target, name)
    }
}
