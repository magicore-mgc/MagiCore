//! `script_policy_tests.rs` — F5: a PRESENT but broken `[scripts]`
//! table must fail the install (hard error), never warn-and-ignore.
//! Missing file/table stays Ok(None) — only breakage fails.
//! Bảng `[scripts]` hỏng phải lỗi cứng; thiếu file vẫn Ok(None).

use crate::install::script_policy::load_file_scripts_policy;

fn tmp(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("mgc-script-policy-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn broken_scripts_toml_fails_closed() {
    // P0/F5-RED: a syntactically broken mgc.toml must Err (a deny the
    // user believes is active must never be silently dropped).
    let dir = tmp("broken");
    std::fs::write(dir.join("mgc.toml"), "[scripts\npolicy = \"allow\"\n").unwrap();
    assert!(
        load_file_scripts_policy(&dir).is_err(),
        "broken [scripts] TOML must fail, not warn-and-ignore"
    );
}

#[test]
fn wrongly_typed_scripts_table_fails_closed() {
    // Present table with the wrong shape (deny must be a list) → Err.
    let dir = tmp("wrongtype");
    std::fs::write(dir.join("mgc.toml"), "[scripts]\ndeny = 42\n").unwrap();
    assert!(
        load_file_scripts_policy(&dir).is_err(),
        "wrongly-typed [scripts] table must fail"
    );
}

#[test]
fn missing_file_or_table_is_no_opinion() {
    // No file / no table → Ok(None): a missing policy must not break
    // installs.
    let bare = tmp("bare");
    assert!(load_file_scripts_policy(&bare).unwrap().is_none());
    let no_table = tmp("notable");
    std::fs::write(no_table.join("mgc.toml"), "name = \"x\"\n").unwrap();
    assert!(load_file_scripts_policy(&no_table).unwrap().is_none());
}

/// P0-3-RED: an unreadable trust DB must Err (fail-closed), never an
/// empty map that silently drops believed-active denies.
/// Trust DB không đọc được phải lỗi cứng, không map rỗng.
#[test]
fn trust_db_unreadable_fails_closed() {
    use crate::install::script_policy::load_trust_policies;
    // A layout whose DB path sits under a missing directory tree cannot
    // open — the loader must surface it, not default to empty.
    let missing = std::env::temp_dir().join(format!(
        "mgc-trust-broken-{}-no-such-dir",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&missing);
    let layout = mgc_store::Layout::new(missing.join("store"));
    assert!(
        load_trust_policies(&layout).is_err(),
        "unopenable trust DB must fail closed"
    );
}
