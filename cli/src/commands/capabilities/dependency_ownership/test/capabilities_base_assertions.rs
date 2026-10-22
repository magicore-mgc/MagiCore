//! Capability ownership assertions — assertion capability ownership.
//! Keeps the original assertion statements grouped by output surface.
//! Giữ assertion gốc, nhóm theo bề mặt output.

#[allow(dead_code)] // Called by the grouped ownership test to keep assertions together.
pub(super) fn assert_base_rows() {
    // T0.4: `mgc capabilities --json` is the single machine-readable
    // source for the matrix cross-check — every core reports all
    // dependency ops with {owner, tools}, plus per-ecosystem overrides
    // for splitting cores.
    // Đây là nguồn máy đọc cho matrix; mỗi core xuất operation và override ecosystem.
    use crate::commands::capabilities::dependency_ownership::dependency_ownership;
    for core in [
        "web", "lib", "ai", "app", "game", "iot", "clo", "cicd", "hardware",
    ] {
        let ownership = dependency_ownership(core);
        let operations = &ownership["operations"];
        for op in [
            "install",
            "add",
            "remove",
            "update",
            "list",
            "resolve",
            "lock",
            "fetch",
            "verify",
            "store",
            "materialize",
            "frozen-install",
            "offline-reinstall",
            "gc",
        ] {
            let cell = &operations[op];
            assert!(
                cell.get("owner").is_some() && cell.get("tools").is_some(),
                "{core}/{op} must carry {{owner, tools}}"
            );
            assert!(
                ["mgc-native", "delegated", "scaffold-only", "unsupported"]
                    .contains(&cell["owner"].as_str().unwrap_or("")),
                "{core}/{op} owner must be closed-vocabulary"
            );
        }
    }
    let web = &dependency_ownership("web")["operations"];
    assert_eq!(web["install"]["owner"], "unsupported");
    let web_languages = dependency_ownership("web")["languages"].clone();
    assert_eq!(web_languages["js"]["install"]["owner"], "mgc-native");
    assert_eq!(
        web_languages["javascript"]["install"]["owner"],
        "mgc-native"
    );
    let ai = &dependency_ownership("ai")["operations"];
    assert_eq!(ai["install"]["owner"], "unsupported");
    let ai_languages = dependency_ownership("ai")["languages"].clone();
    assert_eq!(ai_languages["python"]["install"]["owner"], "mgc-native");
    assert_eq!(ai_languages["python"]["update"]["owner"], "mgc-native");
    let hardware = &dependency_ownership("hardware")["operations"];
    assert_eq!(hardware["install"]["owner"], "unsupported");
}
