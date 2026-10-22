//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

use crate::commands::dep_gate::test_support::*;

#[cfg(any(feature = "lib", feature = "ai"))]
#[test]
fn lib_runtime_context_uses_the_project_manifest_format() {
    let pom = tempfile::tempdir().expect("Maven fixture");
    std::fs::write(pom.path().join("pom.xml"), "<project/>").expect("write POM");
    assert!(matches!(
        owner_for(&crate::commands::dep_gate::lib_project_context(
            pom.path(),
            DepOp::Install
        )),
        DepOwner::Native
    ));

    let gradle = tempfile::tempdir().expect("Gradle fixture");
    std::fs::write(gradle.path().join("build.gradle"), "plugins {}").expect("write Gradle script");
    let gradle_context =
        crate::commands::dep_gate::lib_project_context(gradle.path(), DepOp::Install);
    assert!(matches!(owner_for(&gradle_context), DepOwner::Unsupported));
    assert_eq!(gradle_context.describe(), "lib[java|manifest:gradle]");
    let error = gate(&gradle_context, None, &native(), None)
        .expect_err("Gradle must be rejected before entering the adapter")
        .to_string();
    assert!(error.contains("Gradle build scripts are not dependency manifests"));

    let mixed = tempfile::tempdir().expect("mixed Maven/Gradle fixture");
    std::fs::write(mixed.path().join("pom.xml"), "<project/>").expect("write POM");
    std::fs::write(mixed.path().join("build.gradle"), "plugins {}").expect("write Gradle script");
    let mixed_context =
        crate::commands::dep_gate::lib_project_context(mixed.path(), DepOp::Install);
    assert!(matches!(owner_for(&mixed_context), DepOwner::Unsupported));
    let error = gate(&mixed_context, None, &native(), None)
        .expect_err("mixed build systems must not be silently assigned to Maven")
        .to_string();
    assert!(error.contains("both Maven and Gradle manifests"));

    let empty_dotnet = tempfile::tempdir().expect("empty .NET fixture");
    assert!(matches!(
        owner_for(&crate::commands::dep_gate::lib_project_context(
            empty_dotnet.path(),
            DepOp::Add
        )),
        DepOwner::Unsupported
    ));
    std::fs::write(empty_dotnet.path().join("Project.CSPROJ"), "<Project/>").expect("write csproj");
    assert!(matches!(
        owner_for(&crate::commands::dep_gate::lib_project_context(
            empty_dotnet.path(),
            DepOp::Add
        )),
        DepOwner::Native
    ));
    std::fs::write(empty_dotnet.path().join("Second.csproj"), "<Project/>")
        .expect("write second csproj");
    assert!(matches!(
        owner_for(&crate::commands::dep_gate::lib_project_context(
            empty_dotnet.path(),
            DepOp::Add
        )),
        DepOwner::Unsupported
    ));
}
