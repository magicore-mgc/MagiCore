//! AI and Flutter application fixtures. / Fixture dự án AI và ứng dụng Flutter.

use crate::native_ownership_matrix_e2e::support::{python_manifest, write};

pub(crate) fn ai_project(dir: &std::path::Path) {
    write(
        dir,
        "mgc.toml",
        "name = \"m\"\necosystem = \"ai\"\n[ai]\nframework = \"python-agent\"\n",
    );
    let (_, body) = python_manifest();
    write(dir, "pyproject.toml", &body);
}

pub(crate) fn flutter_project(dir: &std::path::Path) {
    write(
        dir,
        "mgc.toml",
        "name = \"m\"\necosystem = \"app\"\n[app]\nlanguage = \"flutter\"\n",
    );
    write(
        dir,
        "pubspec.yaml",
        "name: m\nenvironment:\n  sdk: \">=3.0.0 <4.0.0\"\ndependencies:\n  meta: ^1.12.0\n",
    );
}

pub(crate) fn flutter_with_meta(dir: &std::path::Path) {
    write(
        dir,
        "mgc.toml",
        "name = \"m\"\necosystem = \"app\"\n[app]\nlanguage = \"flutter\"\n",
    );
    write(
        dir,
        "pubspec.yaml",
        "name: m\nenvironment:\n  sdk: \">=3.0.0 <4.0.0\"\ndependencies:\n  flutter:\n    sdk: flutter\n  meta: ^1.12.0\n",
    );
}

pub(crate) fn swift_with_dep(dir: &std::path::Path) {
    write(
        dir,
        "mgc.toml",
        "name = \"m\"\necosystem = \"app\"\n[app]\nlanguage = \"swift\"\n",
    );
    write(
        dir,
        "Package.swift",
        concat!(
            "// swift-tools-version: 5.9\n",
            "import PackageDescription\n\n",
            "let package = Package(\n",
            "    name: \"m\",\n",
            "    dependencies: [\n",
            "        .package(id: \"scope.lib\", from: \"1.0.0\"),\n",
            "    ],\n",
            ")\n",
        ),
    );
}

pub(crate) fn kotlin_with_dep(dir: &std::path::Path) {
    write(
        dir,
        "mgc.toml",
        "name = \"m\"\necosystem = \"app\"\n[app]\nlanguage = \"kotlin\"\n",
    );
    write(dir, "settings.gradle.kts", "rootProject.name = \"m\"\n");
    std::fs::create_dir_all(dir.join("gradle")).unwrap();
    std::fs::create_dir_all(dir.join("gradle")).unwrap();
    write(
        dir,
        "gradle/libs.versions.toml",
        "[versions]\nlang3 = \"3.14.0\"\n\n[libraries]\ncommons-lang3 = { module = \"org.apache.commons:commons-lang3\", version.ref = \"lang3\" }\n",
    );
    write(
        dir,
        "build.gradle.kts",
        "plugins { kotlin(\"jvm\") version \"2.0.0\" }\ndependencies {\n    implementation(libs.commons.lang3)\n}\n",
    );
}
