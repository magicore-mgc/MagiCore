//! Manifest writer stale dependency pruning tests.
//! Test writer loại dependency cũ khỏi manifest.

/// csproj remove honesty: pins absent from the manifest are deleted,
/// not left behind.
#[test]
fn csproj_writer_prunes_removed_references() {
    use crate::manifest::write_csproj_manifest;
    use mgc_types::{Ecosystem, Manifest};

    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("demo.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk\">\n  <ItemGroup>\n    <PackageReference Include=\"Old.Lib\" Version=\"1.0.0\" />\n  </ItemGroup>\n</Project>\n",
    )
    .unwrap();
    let manifest = Manifest::new("demo", Ecosystem::Lib);
    write_csproj_manifest(dir.path(), &manifest).unwrap();
    let body = std::fs::read_to_string(dir.path().join("demo.csproj")).unwrap();
    assert!(
        !body.contains("Old.Lib"),
        "removed reference must be gone: {body}"
    );
}
/// pom remove honesty: stale top-level pins are deleted, managed ones kept.
#[test]
fn pom_writer_prunes_removed_dependencies() {
    use crate::manifest::write_pom_manifest;
    use mgc_types::{Ecosystem, Manifest};

    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("pom.xml"),
        "<project>\n  <modelVersion>4.0.0</modelVersion>\n  <dependencies>\n    <dependency>\n      <groupId>com.old</groupId>\n      <artifactId>old-lib</artifactId>\n      <version>1.0.0</version>\n    </dependency>\n  </dependencies>\n</project>\n",
    )
    .unwrap();
    let manifest = Manifest::new("demo", Ecosystem::Lib);
    write_pom_manifest(dir.path(), &manifest).unwrap();
    let body = std::fs::read_to_string(dir.path().join("pom.xml")).unwrap();
    assert!(
        !body.contains("old-lib"),
        "removed dependency must be gone: {body}"
    );
}
