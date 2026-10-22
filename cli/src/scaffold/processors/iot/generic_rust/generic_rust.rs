//! Generic Rust writer implementation.
//! Triển khai writer rust tổng quát.

use std::path::Path;

use anyhow::Result;

use crate::scaffold::processors::{slugify, write_file};

pub(in crate::scaffold::processors::iot::implementation) fn write(
    target: &Path,
    name: &str,
) -> Result<()> {
    write_file(
        &target.join("Cargo.toml"),
        &format!(
            "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
            slugify(name)
        ),
    )?;
    write_file(
        &target.join("src").join("main.rs"),
        "#![no_std]\n#![no_main]\n\n#[no_mangle]\npub extern \"C\" fn main() -> ! {\n    loop {}\n}\n",
    )?;
    Ok(())
}
