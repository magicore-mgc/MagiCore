use super::super::*;

#[test]
fn xml_attr_and_tfm() {
    let group = r#"<group targetFramework="net6.0"><dependency id="A" version="[1.0]" /></group>"#;
    assert_eq!(group_tfm(group).as_deref(), Some("net6.0"));
    assert_eq!(tag_attr(group, "id"), None, "group has no id attribute");
    let dep = r#"<dependency id="A" version="[1.0]" exclude="Build,Analyzers" />"#;
    assert_eq!(tag_attr(dep, "id").as_deref(), Some("A"));
    assert_eq!(tag_attr(dep, "version").as_deref(), Some("[1.0]"));
    assert_eq!(tag_attr(dep, "exclude").as_deref(), Some("Build,Analyzers"));
}
