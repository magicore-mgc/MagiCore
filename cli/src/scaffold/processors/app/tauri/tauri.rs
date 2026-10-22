//! Tauri implementation.
//! Triển khai Tauri.

use std::path::Path;

use anyhow::Result;

use crate::scaffold::processors::{slugify, write_file};

pub(in crate::scaffold::processors::app::implementation) fn write(
    target: &Path,
    name: &str,
) -> Result<()> {
    let slug = slugify(name);
    let identifier = format!("dev.magicore.{}", slug.replace('-', "."));
    let tauri_config = serde_json::json!({
        "productName": name,
        "version": "0.1.0",
        "identifier": identifier,
        "build": {
            "devUrl": "http://localhost:1420",
            "frontendDist": "../dist"
        },
        "app": {
            "windows": [{ "title": name, "width": 800, "height": 600 }],
            "security": { "csp": null }
        },
        "bundle": { "active": true, "targets": "all" }
    });
    write_file(
        &target.join("package.json"),
        &format!(
            "{{\n  \"name\": \"{slug}\",\n  \"private\": true,\n  \"version\": \"0.1.0\",\n  \"scripts\": {{ \"build\": \"vite build\", \"dev\": \"vite --host 127.0.0.1 --port 1420 --strictPort\" }},\n  \"devDependencies\": {{ \"vite\": \"^6.0.0\" }}\n}}\n"
        ),
    )?;
    write_file(
        &target.join("index.html"),
        "<!doctype html>\n<html><head><meta charset=\"UTF-8\"><title>MagiCore Tauri</title></head><body><main id=\"app\"></main><script type=\"module\" src=\"/src/main.js\"></script></body></html>\n",
    )?;
    write_file(
        &target.join("src/main.js"),
        "document.querySelector('#app').innerHTML = '<h1>MagiCore Tauri app</h1><p>Frontend scaffold ready.</p>';\n",
    )?;
    write_file(
        &target.join("dist/index.html"),
        "<!doctype html>\n<html><head><meta charset=\"UTF-8\"><title>MagiCore Tauri</title></head><body><main><h1>MagiCore Tauri app</h1></main></body></html>\n",
    )?;
    write_file(
        &target.join("src-tauri/Cargo.toml"),
        &format!(
            "[package]\nname = \"{slug}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[build-dependencies]\ntauri-build = {{ version = \"2\" }}\n\n[dependencies]\ntauri = {{ version = \"2\" }}\n\n[features]\ncustom-protocol = [\"tauri/custom-protocol\"]\n"
        ),
    )?;
    write_file(
        &target.join("src-tauri/build.rs"),
        "fn main() { tauri_build::build() }\n",
    )?;
    write_file(
        &target.join("src-tauri/src/main.rs"),
        "#![cfg_attr(not(debug_assertions), windows_subsystem = \"windows\")]\n\nfn main() {\n    tauri::Builder::default()\n        .run(tauri::generate_context!())\n        .expect(\"failed to run Tauri application\");\n}\n",
    )?;
    write_file(
        &target.join("src-tauri/tauri.conf.json"),
        &format!("{}\n", serde_json::to_string_pretty(&tauri_config)?),
    )?;
    Ok(())
}
