//! Library and language-specific project fixtures. / Fixture dự án library và manifest theo ngôn ngữ.

pub(crate) fn write(dir: &std::path::Path, name: &str, content: &str) {
    std::fs::write(dir.join(name), content).unwrap();
}

pub(crate) fn lib_project(
    dir: &std::path::Path,
    language: &str,
    manifest_name: &str,
    manifest_body: &str,
) {
    write(
        dir,
        "mgc.toml",
        &format!("name = \"m\"\necosystem = \"lib\"\n[lib]\nlanguage = \"{language}\"\n"),
    );
    write(dir, manifest_name, manifest_body);
}

pub(crate) fn python_manifest() -> (&'static str, String) {
    (
        "pyproject.toml",
        "[project]\nname = \"m\"\nversion = \"0.1.0\"\nrequires-python = \">=3.11\"\ndependencies = []\n"
            .to_string(),
    )
}

pub(crate) fn rust_manifest() -> (&'static str, String) {
    (
        "Cargo.toml",
        "[package]\nname = \"m\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n"
            .to_string(),
    )
}

pub(crate) fn go_manifest() -> (&'static str, String) {
    ("go.mod", "module m\n\ngo 1.21\n".to_string())
}

pub(crate) fn dotnet_manifest() -> (&'static str, String) {
    (
        "m.csproj",
        "<Project Sdk=\"Microsoft.NET.Sdk\">\n  <PropertyGroup>\n    <TargetFramework>net8.0</TargetFramework>\n  </PropertyGroup>\n</Project>\n"
            .to_string(),
    )
}

pub(crate) fn java_manifest() -> (&'static str, String) {
    (
        "pom.xml",
        "<project>\n  <modelVersion>4.0.0</modelVersion>\n  <groupId>com.example</groupId>\n  <artifactId>m</artifactId>\n  <version>0.1.0</version>\n</project>\n"
            .to_string(),
    )
}
