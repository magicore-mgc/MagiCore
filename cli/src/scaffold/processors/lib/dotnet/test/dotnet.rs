use crate::scaffold::processors::lib::LibProcessor;

/// `create-lib dotnet` must scaffold a .csproj (not the rust fallback)
/// so NuGet audit/add lanes detect the project.
#[test]
fn dotnet_scaffold_emits_csproj_not_cargo() {
    let temp = tempfile::tempdir().expect("tempdir");
    LibProcessor::files(temp.path(), "demo-lib", "dotnet").expect("dotnet scaffold");
    assert!(
        !temp.path().join("Cargo.toml").exists(),
        "dotnet scaffold must not contain Cargo.toml"
    );
    assert!(
        temp.path().join("demo-lib.csproj").exists(),
        "csproj must exist"
    );
}
