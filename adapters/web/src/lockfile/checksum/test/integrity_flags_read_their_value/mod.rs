// Integrity flags must be read by value, not by presence.
// Cờ integrity phải đọc theo giá trị, không theo sự tồn tại.
//
// Presence-only reads made `=false` behave like `=true`, in both directions:
// strict mode could not be turned off, and the suppression flag could not be
// turned off either — which hid a missing checksum the operator asked to see.
// (Đọc theo sự tồn tại làm `=false` hành xử như `=true` ở cả hai chiều.)

use super::super::checksum::{skip_lockfile_checksum_warning, strict_integrity_enforced};

/// Serialise the cases: `std::env` is one map shared by every test thread.
/// (Tuần tự hoá: `std::env` là map dùng chung giữa mọi thread test.)
static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

const FLAGS: &[&str] = &[
    "MAGICORE_STRICT_INTEGRITY",
    "MGC_STRICT_INTEGRITY",
    "MAGICORE_WEB_SKIP_LOCKFILE_CHECKSUM",
];

#[allow(unsafe_code)] // `set_var`/`remove_var` are unsafe in edition 2024.
fn clear_flags() {
    for name in FLAGS {
        unsafe { std::env::remove_var(name) };
    }
}

#[allow(unsafe_code)]
fn set_flag(name: &str, value: &str) {
    unsafe { std::env::set_var(name, value) };
}

#[test]
fn strict_integrity_is_off_by_default() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    clear_flags();
    assert!(!strict_integrity_enforced());
    clear_flags();
}

#[test]
fn every_enabled_spelling_turns_strict_integrity_on() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    for (flag, value) in [
        ("MAGICORE_STRICT_INTEGRITY", "1"),
        ("MAGICORE_STRICT_INTEGRITY", "true"),
        ("MGC_STRICT_INTEGRITY", "1"),
        ("MGC_STRICT_INTEGRITY", "yes"),
    ] {
        clear_flags();
        set_flag(flag, value);
        assert!(
            strict_integrity_enforced(),
            "{flag}={value} must enable strict integrity"
        );
    }
    clear_flags();
}

/// `=false` must genuinely disable strict mode.
/// (`=false` phải thật sự tắt strict.)
#[test]
fn every_disabled_spelling_turns_strict_integrity_off() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    for value in ["0", "false", "FALSE", "no", "off", ""] {
        clear_flags();
        set_flag("MAGICORE_STRICT_INTEGRITY", value);
        assert!(
            !strict_integrity_enforced(),
            "MAGICORE_STRICT_INTEGRITY={value:?} must not enable strict integrity"
        );
    }
    clear_flags();
}

/// Either spelling enables strict mode, and one disabled spelling does not veto
/// the other: `MAGICORE_=false` + `MGC_=1` is still strict.
/// (Một cách viết tắt không phủ quyết cách còn lại.)
#[test]
fn the_two_spellings_are_independent() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    clear_flags();
    set_flag("MAGICORE_STRICT_INTEGRITY", "false");
    set_flag("MGC_STRICT_INTEGRITY", "1");
    assert!(strict_integrity_enforced());
    clear_flags();
}

/// The suppression flag reads its value too: `=false` must NOT suppress, and
/// `=1` must suppress.
/// (Cờ bỏ cảnh báo đọc theo giá trị: `=false` KHÔNG được bỏ, `=1` phải bỏ.)
#[test]
fn the_lockfile_checksum_warning_flag_reads_its_value() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    clear_flags();
    set_flag("MAGICORE_STRICT_INTEGRITY", "1");
    set_flag("MAGICORE_WEB_SKIP_LOCKFILE_CHECKSUM", "false");
    assert!(
        !skip_lockfile_checksum_warning(),
        "=false must not suppress the missing-checksum warning"
    );

    set_flag("MAGICORE_WEB_SKIP_LOCKFILE_CHECKSUM", "1");
    assert!(
        skip_lockfile_checksum_warning(),
        "=1 must suppress the missing-checksum warning"
    );
    clear_flags();
}
