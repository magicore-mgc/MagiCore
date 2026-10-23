//! Astro frontend starter.
//! Starter frontend Astro.

use std::path::Path;

use anyhow::Result;

use super::super::super::write_file;
use super::super::common::package_name;

pub(crate) fn write_astro_frontend(target: &Path, name: &str) -> Result<()> {
    let manifest = serde_json::json!({
        "name": package_name(name),
        "private": true,
        "version": "0.1.0",
        "type": "module",
        "scripts": {
            "dev": "astro dev",
            "build": "astro build",
            "preview": "astro preview"
        }
    });
    write_file(
        &target.join("package.json"),
        &serde_json::to_string_pretty(&manifest)?,
    )?;
    write_file(
        &target.join("astro.config.mjs"),
        "import { defineConfig } from 'astro/config';\n\nexport default defineConfig({});\n",
    )?;
    write_file(
        &target.join("src/pages/index.astro"),
        "---\nconst title = 'MagiCore + Astro';\n---\n<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\n    <title>{title}</title>\n  </head>\n  <body>\n    <main><h1>{title}</h1><p>Your Astro project is ready.</p></main>\n  </body>\n</html>\n",
    )
}
