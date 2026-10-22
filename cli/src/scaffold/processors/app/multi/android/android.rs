//! Android output implementation.
//! Triển khai output Android.

use super::super::super::MultiProjectContext;
use anyhow::Result;

use crate::scaffold::processors::write_file;

pub(in crate::scaffold::processors::app::implementation) fn write(
    context: &MultiProjectContext<'_>,
) -> Result<()> {
    let target = context.target;
    let slug = context.slug.as_str();
    let pkg = context.package.as_str();

    // The Android entrypoint depends on the shared Kotlin Gradle module.
    // android/ — kotlin entry dùng shared (gradle include + app module phụ thuộc shared)
    write_file(
        &target.join("android").join("settings.gradle.kts"),
        &format!(
            "rootProject.name = \"{}-android\"\n\ninclude(\":app\", \":shared\")\n\nproject(\":shared\").projectDir = file(\"../shared\")\n",
            slug
        ),
    )?;
    write_file(
        &target.join("android").join("build.gradle.kts"),
        "plugins {\n    id(\"com.android.application\") version \"8.5.0\" apply false\n    kotlin(\"android\") version \"2.0.0\" apply false\n}\n",
    )?;
    write_file(
        &target.join("android").join("app").join("build.gradle.kts"),
        &format!(
            "plugins {{\n    id(\"com.android.application\")\n    kotlin(\"android\")\n}}\n\nandroid {{\n    namespace = \"{pkg}.android\"\n    compileSdk = 34\n    defaultConfig {{\n        applicationId = \"{pkg}.android\"\n        minSdk = 24\n    }}\n}}\n\ndependencies {{\n    implementation(project(\":shared\"))\n}}\n"
        ),
    )?;
    write_file(
        &target
            .join("android")
            .join("app")
            .join("src")
            .join("main")
            .join("kotlin")
            .join("Main.kt"),
        "package {pkg}.android\n\nimport {pkg}.Shared\n\nfun main() {\n    println(Shared.hello())\n}\n",
    )?;
    Ok(())
}
