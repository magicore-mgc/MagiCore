//! Maven POM parser edge tests.
//! Test biên cho parser POM Maven.

use super::*;

#[test]
fn dependency_management_entries_are_skipped_from_resolved_build_pins() {
    let pom = r#"
<project>
<dependencyManagement>
<dependencies>
<dependency>
<groupId>example</groupId>
<artifactId>managed-only</artifactId>
<version>2.0</version>
</dependency>
</dependencies>
</dependencyManagement>
<dependencies>
<dependency>
<groupId>example</groupId>
<artifactId>resolved</artifactId>
<version>1.0</version>
</dependency>
</dependencies>
</project>
"#;
    let (pins, skipped) = read_pom_gavs(pom);
    assert_eq!(pins.len(), 1);
    assert_eq!(pins[0].name, "example:resolved");
    assert_eq!(skipped.len(), 1);
    assert!(skipped[0].contains("dependencyManagement"));
}
