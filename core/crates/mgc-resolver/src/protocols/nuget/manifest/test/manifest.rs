use super::super::*;

#[test]
fn package_reference_parsing() {
    let csproj = r#"<Project Sdk="Microsoft.NET.Sdk">
      <ItemGroup>
        <PackageReference Include="Newtonsoft.Json" Version="13.0.3" />
        <PackageReference Include="Wildcard" Version="1.*" />
        <PackageReference Include="NoVersion" />
        <PackageReference Include="Star" Version="*" />
        <PackageReference Include="Range" Version="[1.0, 2.0)" />
      </ItemGroup>
    </Project>"#;
    let (refs, unresolved) = parse_package_references(csproj);
    assert_eq!(refs.len(), 2);
    assert_eq!(refs[0].id, "Newtonsoft.Json");
    assert_eq!(refs[0].version.as_deref(), Some("13.0.3"));
    assert_eq!(refs[1].id, "Range");
    assert_eq!(unresolved, vec!["Wildcard", "NoVersion", "Star"]);
}
