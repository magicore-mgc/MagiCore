//! Kotlin implementation.
//! Triển khai Kotlin.

use std::path::Path;

use anyhow::Result;

use crate::scaffold::processors::write_file;

pub(in crate::scaffold::processors::app::implementation) fn write(
    target: &Path,
    name: &str,
) -> Result<()> {
    write_file(
        &target.join("settings.gradle.kts"),
        &format!("rootProject.name = \"{}\"\n", name),
    )?;
    write_file(
        &target
            .join("app")
            .join("src")
            .join("main")
            .join("kotlin")
            .join("Main.kt"),
        "fun main() {\n    println(\"MagiCore Kotlin app scaffold\")\n}\n",
    )?;
    Ok(())
}
