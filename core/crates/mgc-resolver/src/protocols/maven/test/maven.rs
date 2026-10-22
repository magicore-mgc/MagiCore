use super::*;

#[test]
fn element_blocks_finds_and_counts_nesting() {
    let xml = "<a><versions><version>1.0</version><version>2.0</version></versions></a>";
    let blocks = element_blocks(xml, "versions");
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0], "<version>1.0</version><version>2.0</version>");
    let versions = element_blocks(blocks[0], "version");
    assert_eq!(versions, vec!["1.0", "2.0"]);
    // The `<version>` search must return exactly the two leaf elements —
    // it must NOT swallow the `<versions>` block as a match.
    // (Tìm `<version>` phải trả đúng hai phần tử lá — KHÔNG được nuốt
    // khối `<versions>` như một kết quả khớp.)
    assert_eq!(element_blocks(xml, "version"), vec!["1.0", "2.0"]);
}

#[test]
fn dependency_management_is_stripped_before_collection() {
    let pom = r#"<project>
          <packaging>jar</packaging>
          <dependencyManagement>
            <dependencies>
              <dependency>
                <groupId>com.dm</groupId><artifactId>managed</artifactId><version>9.9</version>
              </dependency>
            </dependencies>
          </dependencyManagement>
          <dependencies>
            <dependency>
              <groupId>com.example</groupId><artifactId>core</artifactId><version>1.2.3</version>
            </dependency>
            <dependency>
              <groupId>com.example</groupId><artifactId>test-dep</artifactId><version>2.0</version><scope>test</scope>
            </dependency>
            <dependency>
              <groupId>com.example</groupId><artifactId>opt</artifactId><version>0.1</version><optional>true</optional>
            </dependency>
            <dependency>
              <groupId>com.example</groupId><artifactId>prop</artifactId><version>${core.version}</version>
            </dependency>
          </dependencies>
        </project>"#;
    let (deps, packaging) = collect_pom_dependencies(pom).expect("test operation must succeed");
    assert_eq!(packaging.as_deref(), Some("jar"));
    assert_eq!(deps.len(), 4, "managed entries must not be collected");
    let core = &deps[0];
    assert_eq!(core.group.as_deref(), Some("com.example"));
    assert_eq!(core.artifact.as_deref(), Some("core"));
    assert_eq!(core.version.as_deref(), Some("1.2.3"));
    assert_eq!(core.scope, None, "no scope element = compile");
    assert!(!core.optional);
}

#[test]
fn maven_range_semantics() {
    let v = Version::parse("1.2.3").expect("test operation must succeed");
    assert!(maven_matches("1.2", &v), "bare = caret");
    assert!(!maven_matches("2.0", &v));
    assert!(maven_matches("[1.2.3]", &v));
    assert!(!maven_matches("[1.2.4]", &v));
    assert!(maven_matches("[1.0,2.0)", &v));
    assert!(!maven_matches("[1.0,1.2.3)", &v));
    assert!(maven_matches("(1.0,]", &v));
    assert!(maven_matches("*", &v));
}

#[test]
fn plugin_reporting_profile_deps_never_enter_the_graph() {
    // junit-style pom: plugin <dependencies> (surefire), <reporting>
    // and <profiles> must not pollute the project graph — only the
    // top-level <dependencies> count.
    // (Dep của plugin/reporting/profile không vào graph.)
    let pom = r#"<project>
          <dependencies>
            <dependency>
              <groupId>com.example</groupId><artifactId>core</artifactId><version>1.0</version>
            </dependency>
          </dependencies>
          <build>
            <plugins>
              <plugin>
                <artifactId>surefire</artifactId>
                <dependencies>
                  <dependency>
                    <groupId>org.apache.maven.surefire</groupId><artifactId>surefire-junit47</artifactId><version>${surefireVersion}</version>
                  </dependency>
                </dependencies>
              </plugin>
            </plugins>
          </build>
          <reporting>
            <plugins>
              <plugin>
                <artifactId>site</artifactId>
                <dependencies>
                  <dependency>
                    <groupId>org.apache.maven.doxia</groupId><artifactId>doxia-module-xhtml</artifactId>
                  </dependency>
                </dependencies>
              </plugin>
            </plugins>
          </reporting>
          <profiles>
            <profile>
              <dependencies>
                <dependency>
                  <groupId>com.example</groupId><artifactId>prof</artifactId><version>9.9</version>
                </dependency>
              </dependencies>
            </profile>
          </profiles>
        </project>"#;
    let (deps, _) = collect_pom_dependencies(pom).expect("test operation must succeed");
    assert_eq!(deps.len(), 1, "only the top-level dep counts: {deps:?}");
    assert_eq!(deps[0].artifact.as_deref(), Some("core"));
}
