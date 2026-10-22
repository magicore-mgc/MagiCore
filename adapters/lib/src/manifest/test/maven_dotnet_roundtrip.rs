//! Maven and .NET native writer round-trip tests.
//! Test round-trip writer native Maven và .NET.

/// csproj writer (native add): mgc owns PackageReference pins — inserts
/// `<PackageReference Include Version>` into an ItemGroup (creating one
/// when absent), preserving everything else. Round-trip stable.
/// (Writer csproj: ghim PackageReference, giữ phần còn lại.)
#[test]
fn csproj_writer_pins_reference_and_round_trips() {
    use crate::manifest::{parse_csproj_manifest, write_csproj_manifest};
    use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageName, VersionRange};

    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("demo.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk\">\n  <PropertyGroup>\n    <TargetFramework>net8.0</TargetFramework>\n  </PropertyGroup>\n</Project>\n",
    )
    .unwrap();
    let mut manifest = Manifest::new("demo", Ecosystem::Lib);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("Newtonsoft.Json").unwrap(),
            VersionRange::parse("13.0.3").unwrap(),
        ),
        false,
        false,
        false,
    );
    write_csproj_manifest(dir.path(), &manifest).unwrap();
    let body = std::fs::read_to_string(dir.path().join("demo.csproj")).unwrap();
    assert!(
        body.contains("Newtonsoft.Json") && body.contains("13.0.3"),
        "PackageReference pin must be written: {body}"
    );
    assert!(
        body.contains("<TargetFramework>net8.0</TargetFramework>"),
        "project body must survive: {body}"
    );
    let again = parse_csproj_manifest(dir.path()).unwrap();
    let dep = again
        .find_dep("Newtonsoft.Json")
        .expect("re-parse finds pin");
    assert_eq!(
        dep.range.satisfying_version().map(|v| v.to_string()),
        Some("13.0.3".to_string()),
        "re-parsed pin must hold the version"
    );
}
/// pom.xml writer (native add): mgc owns dependency pins — inserts a
/// `<dependency>` block into top-level `<dependencies>` (creating the
/// block when absent), preserving everything else. Round-trip stable.
/// (Writer pom.xml: ghim dependency, giữ phần còn lại.)
#[test]
fn pom_writer_pins_dependency_and_round_trips() {
    use crate::manifest::{parse_maven_manifest, write_pom_manifest};
    use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageName, VersionRange};

    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("pom.xml"),
        "<project>\n  <modelVersion>4.0.0</modelVersion>\n  <groupId>com.example</groupId>\n  <artifactId>demo</artifactId>\n  <version>0.1.0</version>\n  <dependencyManagement>\n    <dependencies>\n      <dependency>\n        <groupId>org.managed</groupId>\n        <artifactId>managed-lib</artifactId>\n        <version>9.9.9</version>\n      </dependency>\n    </dependencies>\n  </dependencyManagement>\n</project>\n",
    )
    .unwrap();
    let mut manifest = Manifest::new("demo", Ecosystem::Lib);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("org.apache.commons:commons-lang3").unwrap(),
            VersionRange::parse("3.14.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    write_pom_manifest(dir.path(), &manifest).unwrap();
    let body = std::fs::read_to_string(dir.path().join("pom.xml")).unwrap();
    assert!(
        body.contains("commons-lang3") && body.contains("3.14.0"),
        "dependency pin must be written: {body}"
    );
    assert!(
        body.contains("<artifactId>demo</artifactId>"),
        "project body must survive: {body}"
    );
    assert!(
        body.contains("<artifactId>managed-lib</artifactId>"),
        "dependencyManagement must survive untouched: {body}"
    );
    let managed_count = body.matches("managed-lib").count();
    assert_eq!(managed_count, 1, "managed entry must not duplicate: {body}");
    let again = parse_maven_manifest(dir.path()).unwrap();
    let dep = again
        .find_dep("org.apache.commons:commons-lang3")
        .expect("re-parse finds pin");
    assert_eq!(
        dep.range.satisfying_version().map(|v| v.to_string()),
        Some("3.14.0".to_string()),
        "re-parsed pin must hold the version"
    );
}
#[test]
fn pom_writer_preserves_non_runtime_dependencies_and_existing_metadata() {
    use crate::manifest::write_pom_manifest;
    use mgc_types::{DependencySpec, Ecosystem, Manifest, PackageName, VersionRange};

    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("pom.xml"),
        r#"<project>
  <dependencies>
    <dependency>
      <groupId>org.example</groupId><artifactId>test-only</artifactId><version>1.0.0</version>
      <scope>test</scope>
    </dependency>
    <dependency>
      <groupId>org.example</groupId><artifactId>runtime-lib</artifactId><version>1.0.0</version>
      <exclusions><exclusion><groupId>bad.example</groupId><artifactId>bad-child</artifactId></exclusion></exclusions>
    </dependency>
    <dependency>
      <groupId>org.example</groupId><artifactId>removed-runtime</artifactId><version>1.0.0</version>
    </dependency>
    <dependency>
      <groupId>org.example</groupId><artifactId>optional-lib</artifactId><version>1.0.0</version>
      <optional>true</optional>
    </dependency>
  </dependencies>
</project>
"#,
    )
    .unwrap();
    let mut manifest = Manifest::new("demo", Ecosystem::Lib);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("org.example:runtime-lib").unwrap(),
            VersionRange::parse("2.0.0").unwrap(),
        ),
        false,
        false,
        false,
    );

    write_pom_manifest(dir.path(), &manifest).unwrap();

    let body = std::fs::read_to_string(dir.path().join("pom.xml")).unwrap();
    assert!(
        body.contains("<scope>test</scope>"),
        "test dependency survives: {body}"
    );
    assert!(
        body.contains("<optional>true</optional>"),
        "optional dependency survives: {body}"
    );
    assert!(
        body.contains("<artifactId>bad-child</artifactId>"),
        "exclusion survives: {body}"
    );
    assert!(
        body.contains("<version>2.0.0</version>"),
        "owned pin updates: {body}"
    );
    assert!(
        !body.contains("removed-runtime"),
        "stale runtime pin removed: {body}"
    );
}
