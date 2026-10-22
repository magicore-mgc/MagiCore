//! Verify solution project declarations exclude folders and normalize paths.
//! Kiểm tra declaration project bỏ folder và chuẩn hóa path.

#[test]
fn solution_parser_keeps_only_csproj_entries() {
    let raw = r#"Project("{A}") = "Api", "src\Api.csproj", "{B}"
Project("{C}") = "Solution Items", "Solution Items", "{D}"
EndProject"#;

    assert_eq!(
        super::read_solution_projects(raw),
        vec!["src/Api.csproj".to_string()]
    );
}
