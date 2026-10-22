//! Framework-selection owner tests.
//! Test owner chọn framework.

use crate::commands::core::web::scaffold_flags::ScaffoldFlags;

#[test]
fn positional_framework_takes_priority_over_enabled_framework_flags() {
    let flags = ScaffoldFlags {
        react: true,
        ..ScaffoldFlags::default()
    };

    assert_eq!(
        super::resolve_framework(Some("astro"), &flags).unwrap(),
        "astro"
    );
}

#[test]
fn uses_selected_frontend_flag_when_positional_framework_is_absent() {
    let flags = ScaffoldFlags {
        vue: true,
        ..ScaffoldFlags::default()
    };

    assert_eq!(super::resolve_framework(None, &flags).unwrap(), "vue");
}

#[test]
fn requires_selection_when_no_frontend_flag_is_enabled() {
    let flags = ScaffoldFlags::default();

    assert!(super::resolve_framework(None, &flags).is_err());
}

#[test]
fn backend_selection_keeps_declared_flag_priority() {
    let flags = ScaffoldFlags {
        express: true,
        gin: true,
        ..ScaffoldFlags::default()
    };
    assert_eq!(
        super::detect_backend_framework(&flags).as_deref(),
        Some("express")
    );

    let empty = ScaffoldFlags::default();
    assert_eq!(super::detect_backend_framework(&empty), None);
}

#[test]
fn nestjs_backend_forces_typescript_and_clears_javascript() {
    let mut flags = ScaffoldFlags {
        nestjs: true,
        js: true,
        ..ScaffoldFlags::default()
    };

    super::enforce_framework_language_defaults("react", &mut flags);

    assert!(flags.ts);
    assert!(!flags.js);
}
