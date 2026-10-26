//! The dev profile derives its target from the optimizer profile.
//! Profile dev suy ra target từ profile optimizer.

use super::super::{DevCompilerProfile, load};

#[test]
fn a_missing_profile_uses_defaults_and_says_so() {
    let dir = tempfile::tempdir().unwrap();
    let profile = load(dir.path()).expect("a missing profile is not an error");
    // Defaults with a note explaining why they are defaults.
    // Mặc định kèm ghi chú giải thích vì sao là mặc định.
    assert_eq!(profile.target, DevCompilerProfile::default().target);
    assert_eq!(profile.keep_names, DevCompilerProfile::default().keep_names);
    assert!(
        profile
            .profile_note
            .as_deref()
            .is_some_and(|note| note.contains("run `mgc optimizer`")),
        "the note must tell the user how to get one: {profile:?}"
    );
}

#[test]
fn a_high_performance_profile_emits_the_newest_target() {
    let dir = tempfile::tempdir().unwrap();
    let opt = dir.path().join(".mgc-optimizer");
    std::fs::create_dir_all(&opt).unwrap();
    std::fs::write(opt.join("profile.json"), r#"{"profile":"HighPerformance"}"#).unwrap();
    assert_eq!(load(dir.path()).unwrap().target, "esnext");
}

#[test]
fn a_low_power_profile_emits_a_conservative_target() {
    let dir = tempfile::tempdir().unwrap();
    let opt = dir.path().join(".mgc-optimizer");
    std::fs::create_dir_all(&opt).unwrap();
    std::fs::write(opt.join("profile.json"), r#"{"profile":"LowPower"}"#).unwrap();
    assert_eq!(load(dir.path()).unwrap().target, "es2019");
}

#[test]
fn an_unknown_profile_falls_back_to_the_balanced_target() {
    let dir = tempfile::tempdir().unwrap();
    let opt = dir.path().join(".mgc-optimizer");
    std::fs::create_dir_all(&opt).unwrap();
    std::fs::write(opt.join("profile.json"), r#"{"profile":"Other"}"#).unwrap();
    assert_eq!(load(dir.path()).unwrap().target, "es2020");
}
