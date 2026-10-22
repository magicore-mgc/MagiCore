//! Native update and outdated-manifest fixtures. / Helper update native và manifest outdated.

use crate::native_ownership_matrix_e2e::support::{MatrixSandbox, lib_project, write};
use tempfile::TempDir;

/// Native update cell: outdated pin bumps to latest with zero spawn.
pub(crate) fn native_update_cell_with_env(
    setup: fn(&std::path::Path),
    update_cmd: &[&str],
    manifest_file: &str,
    old_pin: &str,
    extra_env: &[(&str, &str)],
) {
    let project = TempDir::new().unwrap();
    setup(project.path());
    let sandbox = MatrixSandbox::new();
    let (code, out) = sandbox.run_with_env(update_cmd, project.path(), extra_env);
    assert_eq!(
        code,
        Some(0),
        "{} must succeed:\n{out}",
        update_cmd.join(" ")
    );
    let body = std::fs::read_to_string(project.path().join(manifest_file)).unwrap();
    assert!(
        !body.contains(old_pin),
        "manifest must drop outdated pin {old_pin}:\n{body}"
    );
    sandbox.assert_no_spawn(&update_cmd.join(" "));
}

pub(crate) fn outdated_python(dir: &std::path::Path) {
    lib_project(
        dir,
        "python",
        "pyproject.toml",
        "[project]\nname = \"m\"\nversion = \"0.1.0\"\nrequires-python = \">=3.11\"\ndependencies = [\"six==1.15.0\"]\n",
    );
}

pub(crate) fn outdated_rust(dir: &std::path::Path) {
    lib_project(
        dir,
        "rust",
        "Cargo.toml",
        "[package]\nname = \"m\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nmgcfixture = \"0.9.0\"\n",
    );
}

pub(crate) fn outdated_go(dir: &std::path::Path) {
    lib_project(
        dir,
        "go",
        "go.mod",
        "module m\n\ngo 1.21\n\nrequire github.com/google/uuid v0.9.0\n",
    );
}

pub(crate) fn outdated_dotnet(dir: &std::path::Path) {
    lib_project(
        dir,
        "dotnet",
        "m.csproj",
        "<Project Sdk=\"Microsoft.NET.Sdk\">\n  <PropertyGroup>\n    <TargetFramework>net8.0</TargetFramework>\n  </PropertyGroup>\n  <ItemGroup>\n    <PackageReference Include=\"Newtonsoft.Json\" Version=\"13.0.1\" />\n  </ItemGroup>\n</Project>\n",
    );
}

pub(crate) fn outdated_java(dir: &std::path::Path) {
    lib_project(
        dir,
        "java",
        "pom.xml",
        "<project>\n  <modelVersion>4.0.0</modelVersion>\n  <groupId>com.example</groupId>\n  <artifactId>m</artifactId>\n  <version>0.1.0</version>\n  <dependencies>\n    <dependency>\n      <groupId>org.apache.commons</groupId>\n      <artifactId>commons-lang3</artifactId>\n      <version>3.12.0</version>\n    </dependency>\n  </dependencies>\n</project>\n",
    );
}

pub(crate) fn outdated_flutter(dir: &std::path::Path) {
    write(
        dir,
        "mgc.toml",
        "name = \"m\"\necosystem = \"app\"\n[app]\nlanguage = \"flutter\"\n",
    );
    write(
        dir,
        "pubspec.yaml",
        "name: m\nenvironment:\n  sdk: \">=3.0.0 <4.0.0\"\ndependencies:\n  meta: 1.9.1\n",
    );
}

pub(crate) fn outdated_ai(dir: &std::path::Path) {
    write(
        dir,
        "mgc.toml",
        "name = \"m\"\necosystem = \"ai\"\n[ai]\nframework = \"python-agent\"\n",
    );
    write(
        dir,
        "pyproject.toml",
        "[project]\nname = \"m\"\nversion = \"0.1.0\"\nrequires-python = \">=3.11\"\ndependencies = [\"six==1.15.0\"]\n",
    );
}

pub(crate) fn outdated_bevy(dir: &std::path::Path) {
    write(
        dir,
        "mgc.toml",
        "name = \"m\"\necosystem = \"game\"\n[game]\nengine = \"bevy\"\n",
    );
    write(
        dir,
        "Cargo.toml",
        "[package]\nname = \"m\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nmgcfixture = \"0.9.0\"\n",
    );
}

pub(crate) fn outdated_esp32(dir: &std::path::Path) {
    write(
        dir,
        "mgc.toml",
        "name = \"m\"\necosystem = \"iot\"\n[iot]\nframework = \"esp32-rust\"\n",
    );
    write(
        dir,
        "Cargo.toml",
        "[package]\nname = \"m\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nmgcfixture = \"0.9.0\"\n",
    );
}
