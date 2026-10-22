//! .NET project manifest ownership and semantic safety tests.
//! Test ownership manifest .NET và an toàn ngữ nghĩa.

#[test]
fn dotnet_manifest_refuses_multiple_root_csprojs_instead_of_empty_success() {
    use crate::manifest::parse_csproj_manifest;

    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("App.csproj"), "<Project/>").unwrap();
    std::fs::write(root.path().join("Tests.csproj"), "<Project/>").unwrap();

    let error = parse_csproj_manifest(root.path()).unwrap_err();

    assert!(
        error
            .to_string()
            .contains("no unique regular root-level .csproj")
    );
}
#[test]
fn dotnet_manifest_refuses_unresolved_package_references_before_mutation() {
    use crate::manifest::{parse_csproj_manifest, write_csproj_manifest};
    use mgc_types::{Ecosystem, Manifest};

    for unresolved in [
        r#"<PackageReference Include="Floating" Version="1.*" />"#,
        r#"<PackageReference Include="CentralManaged" />"#,
    ] {
        let root = tempfile::tempdir().unwrap();
        let original = format!(
            "<Project><ItemGroup><PackageReference Include=\"Pinned\" Version=\"1.2.3\" />{unresolved}</ItemGroup></Project>"
        );
        std::fs::write(root.path().join("App.csproj"), &original).unwrap();

        let error = parse_csproj_manifest(root.path())
            .expect_err("unresolved PackageReference must not be omitted from native graph");
        assert!(matches!(error, mgc_types::MgError::Unsupported { .. }));
        assert!(write_csproj_manifest(root.path(), &Manifest::new("app", Ecosystem::Lib)).is_err());
        assert_eq!(
            std::fs::read_to_string(root.path().join("App.csproj")).unwrap(),
            original,
            "refusal must leave every PackageReference unchanged"
        );
    }
}
#[test]
fn dotnet_manifest_refuses_unmodeled_package_reference_semantics_before_mutation() {
    use crate::manifest::{parse_csproj_manifest, write_csproj_manifest};
    use mgc_types::{Ecosystem, Manifest};

    for conditional_group in [
        r#"<ItemGroup Condition="'$(TargetFramework)' == 'net8.0'"><PackageReference Include="Conditional" Version="1.2.3" /></ItemGroup>"#,
        r#"<ItemGroup><PackageReference Include="Conditional" Version="1.2.3" Condition="'$(Configuration)' == 'Release'" /></ItemGroup>"#,
        r#"<ItemGroup><PackageReference Update="Pinned" Version="2.0.0" /></ItemGroup>"#,
        r#"<ItemGroup><PackageReference Remove="Pinned" /></ItemGroup>"#,
        r#"<m:ItemGroup xmlns:m="urn:msbuild"><m:PackageReference Include="Prefixed" Version="1.2.3" /></m:ItemGroup>"#,
    ] {
        let root = tempfile::tempdir().unwrap();
        let original = format!(
            "<Project><!-- <ItemGroup Condition=\"comment only\"> --><ItemGroup><PackageReference Include=\"Pinned\" Version=\"1.2.3\" /></ItemGroup>{conditional_group}</Project>"
        );
        std::fs::write(root.path().join("App.csproj"), &original).unwrap();

        assert!(parse_csproj_manifest(root.path()).is_err());
        assert!(write_csproj_manifest(root.path(), &Manifest::new("app", Ecosystem::Lib)).is_err());
        assert_eq!(
            std::fs::read_to_string(root.path().join("App.csproj")).unwrap(),
            original,
            "conditional graph refusal must leave csproj unchanged"
        );
    }
}
