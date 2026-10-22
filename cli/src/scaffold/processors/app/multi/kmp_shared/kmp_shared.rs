//! KMP shared output implementation.
//! Triển khai output KMP dùng chung.

use super::super::super::MultiProjectContext;
use anyhow::Result;

use crate::scaffold::processors::write_file;

pub(in crate::scaffold::processors::app::implementation) fn write(
    context: &MultiProjectContext<'_>,
) -> Result<()> {
    let target = context.target;
    let name = context.name;
    let slug = context.slug.as_str();
    let pkg = context.package.as_str();

    // KMP commonMain and Android sources share application logic.
    // shared/ — Kotlin KMP commonMain + androidMain (nơi chứa logic dùng chung)
    write_file(
        &target.join("shared").join("build.gradle.kts"),
        &format!(
            "plugins {{\n    kotlin(\"multiplatform\") version \"2.0.0\"\n}}\n\nkotlin {{\n    androidTarget()\n    iosX64()\n    iosArm64()\n    iosSimulatorArm64()\n\n    sourceSets {{\n        commonMain.dependencies {{\n            implementation(\"org.jetbrains.kotlinx:kotlinx-coroutines-core:1.9.0\")\n        }}\n    }}\n\n    listOf(\n        iosX64(),\n        iosArm64(),\n        iosSimulatorArm64()\n    ).forEach {{ iosTarget ->\n        iosTarget.binaries.framework {{\n            baseName = \"{name}\"\n            isStatic = true\n        }}\n    }}\n}}\n\nandroid {{\n    namespace = \"{pkg}.shared\"\n    compileSdk = 34\n}}\n"
        ),
    )?;
    write_file(
        &target.join("shared").join("settings.gradle.kts"),
        &format!("rootProject.name = \"{}-shared\"\n", slug),
    )?;
    write_file(
        &target
            .join("shared")
            .join("src")
            .join("commonMain")
            .join("kotlin")
            .join(pkg)
            .join("Shared.kt"),
        "package {pkg}\n\nobject Shared {\n    fun hello(): String = \"hello from MagiCore shared (KMP)\"\n}\n",
    )?;
    Ok(())
}
