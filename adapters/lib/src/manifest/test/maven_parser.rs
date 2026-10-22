//! Maven POM parser behavior and fail-closed tests.
//! Test parser Maven POM và hành vi fail-closed.

#[test]
fn maven_manifest_rejects_unresolved_direct_dependencies_instead_of_dropping_them() {
    use crate::manifest::parse_maven_manifest;

    let root = tempfile::tempdir().unwrap();
    std::fs::write(
        root.path().join("pom.xml"),
        r#"<project>
  <groupId>com.example</groupId><artifactId>spring-app</artifactId><version>1.0.0</version>
  <dependencies><dependency>
    <groupId>org.springframework</groupId><artifactId>spring-core</artifactId>
  </dependency></dependencies>
</project>"#,
    )
    .unwrap();

    let error = parse_maven_manifest(root.path()).unwrap_err();

    assert!(error.to_string().contains("spring-core"));
    assert!(error.to_string().contains("cannot be resolved"));
}
#[test]
fn maven_manifest_resolves_local_properties_and_literal_dependency_management() {
    use crate::manifest::parse_maven_manifest;

    let root = tempfile::tempdir().unwrap();
    std::fs::write(
        root.path().join("pom.xml"),
        r#"<project>
  <groupId>com.example</groupId><artifactId>spring-app</artifactId><version>1.0.0</version>
  <properties><spring.version>6.1.0</spring.version></properties>
  <dependencyManagement><dependencies><dependency>
    <groupId>org.springframework</groupId><artifactId>spring-core</artifactId><version>6.1.1</version>
  </dependency></dependencies></dependencyManagement>
  <dependencies>
    <dependency><groupId>org.springframework</groupId><artifactId>spring-web</artifactId><version>${spring.version}</version></dependency>
    <dependency><groupId>org.springframework</groupId><artifactId>spring-core</artifactId></dependency>
  </dependencies>
</project>"#,
    )
    .unwrap();

    let manifest = parse_maven_manifest(root.path()).unwrap();

    assert_eq!(
        manifest
            .find_dep("org.springframework:spring-web")
            .unwrap()
            .range
            .to_string(),
        "6.1.0"
    );
    assert_eq!(
        manifest
            .find_dep("org.springframework:spring-core")
            .unwrap()
            .range
            .to_string(),
        "6.1.1"
    );
}
#[test]
fn maven_manifest_does_not_treat_commented_dependency_as_real() {
    use crate::manifest::parse_maven_manifest;

    let root = tempfile::tempdir().unwrap();
    std::fs::write(
        root.path().join("pom.xml"),
        r#"<project>
  <groupId>com.example</groupId><artifactId>sample</artifactId><version>1.0.0</version>
  <!-- Example only:
  <dependencies><dependency><groupId>bad.example</groupId><artifactId>not-real</artifactId><version>9.9.9</version></dependency></dependencies>
  -->
  <dependencies><dependency><groupId>good.example</groupId><artifactId>real</artifactId><version>1.2.3</version></dependency></dependencies>
</project>"#,
    )
    .unwrap();

    let manifest = parse_maven_manifest(root.path()).unwrap();

    assert!(manifest.find_dep("good.example:real").is_some());
    assert!(manifest.find_dep("bad.example:not-real").is_none());
}
#[test]
fn maven_manifest_rejects_malformed_xml_instead_of_returning_an_empty_graph() {
    use crate::manifest::parse_maven_manifest;

    let root = tempfile::tempdir().unwrap();
    std::fs::write(
        root.path().join("pom.xml"),
        "<project><dependencies><dependency><groupId>g</groupId></project>",
    )
    .unwrap();

    let error = parse_maven_manifest(root.path()).unwrap_err();

    assert!(error.to_string().contains("invalid Maven POM XML"));
}
#[cfg(unix)]
#[test]
fn maven_manifest_refuses_symlinked_pom_outside_project() {
    use crate::manifest::parse_maven_manifest;
    use std::os::unix::fs::symlink;

    let project = tempfile::tempdir().unwrap();
    let external = tempfile::tempdir().unwrap();
    let external_pom = external.path().join("pom.xml");
    std::fs::write(
        &external_pom,
        "<project><dependencies><dependency><groupId>outside</groupId><artifactId>secret</artifactId><version>1.0.0</version></dependency></dependencies></project>",
    )
    .unwrap();
    symlink(&external_pom, project.path().join("pom.xml")).unwrap();

    let error = parse_maven_manifest(project.path()).unwrap_err();

    assert!(error.to_string().contains("symlink"));
}
#[test]
fn maven_manifest_reads_namespace_prefixed_project_elements() {
    use crate::manifest::parse_maven_manifest;

    let root = tempfile::tempdir().unwrap();
    std::fs::write(
        root.path().join("pom.xml"),
        r#"<m:project xmlns:m="http://maven.apache.org/POM/4.0.0">
  <m:groupId>com.example</m:groupId><m:artifactId>sample</m:artifactId><m:version>1.0.0</m:version>
  <m:dependencies><m:dependency><m:groupId>org.example</m:groupId><m:artifactId>real</m:artifactId><m:version>1.2.3</m:version></m:dependency></m:dependencies>
</m:project>"#,
    )
    .unwrap();

    let manifest = parse_maven_manifest(root.path()).unwrap();

    assert!(manifest.find_dep("org.example:real").is_some());
}
