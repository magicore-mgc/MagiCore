//! Registry membership contract tests — test contract membership registry.
//! A name may only be reported as supported when some path in the product can
//! scaffold or materialize it; a name that exists in one table and not the other
//! is the exact failure that made `create-web django` contradict
//! `is_supported("web", "django")`.
//! (Tên chỉ được coi là hỗ trợ khi có đường thật trong sản phẩm scaffold hoặc
//! materialize nó; tên có ở bảng này mà thiếu bảng kia chính là lỗi từng làm
//! `create-web django` mâu thuẫn `is_supported`.)

use super::super::*;

/// Every fullstack id must resolve to a backend half the registry knows.
///
/// `WEB_BACKEND` is the scaffoldable set and `WEB_BACKEND_UNSUPPORTED` the
/// known-but-not-yet set; the union is every backend name the registry models,
/// and a fullstack half must belong to it. Checking only `WEB_BACKEND` would
/// fail for the Rust/Go/Python/PHP halves that have no record yet.
/// (Mọi id fullstack phải quy về một nửa backend mà registry biết. `WEB_BACKEND`
/// là tập scaffold được, `WEB_BACKEND_UNSUPPORTED` là tập đã biết chưa hỗ trợ;
/// hợp là toàn bộ backend registry mô hình hoá, và nửa fullstack phải thuộc đó.)
#[test]
fn every_fullstack_id_has_a_known_backend() {
    for id in WEB_FULLSTACK {
        let (_, backend) = id
            .split_once('-')
            .unwrap_or_else(|| panic!("fullstack id '{id}' has no '<frontend>-<backend>' shape"));
        let backend = match backend {
            // The five ids whose backend half is a different registry name.
            "spring" => "spring-boot",
            "laravel" => "laravel",
            other => other,
        };
        let known = WEB_BACKEND.contains(&backend) || WEB_BACKEND_UNSUPPORTED.contains(&backend);
        assert!(
            known,
            "fullstack id '{id}' names backend '{backend}', which the registry does not model \
             in WEB_BACKEND or WEB_BACKEND_UNSUPPORTED"
        );
    }
}

/// The diagnostic list must be a superset of every accepted id.
/// (Danh sách chẩn đoán phải là siêu tập của mọi id được nhận.)
#[test]
fn the_diagnostic_list_covers_every_accepted_web_id() {
    let web_all = web_all_ids();
    for id in WEB_FRONTEND
        .iter()
        .chain(WEB_BACKEND.iter())
        .chain(WEB_FULLSTACK.iter())
        .chain(WEB_ALIAS.iter())
    {
        assert!(
            web_all.contains(id),
            "'{id}' is accepted by is_supported but missing from web_all_ids(), so the error \
             message would omit a name the user can actually type"
        );
    }
}

/// No duplicates: a repeated id makes the count in the error message wrong.
/// (Không trùng lặp: id lặp làm số đếm trong thông điệp lỗi sai.)
#[test]
fn the_diagnostic_list_has_no_duplicates() {
    let web_all = web_all_ids();
    let mut seen = std::collections::BTreeSet::new();
    for id in &web_all {
        assert!(seen.insert(*id), "'{id}' appears twice in web_all_ids()");
    }
}

/// A host-tool backend with a record is advertised; the gate accepts every
/// backend the create lane can resolve a source for.
/// (Backend host-tool có record được công bố; cổng nhận mọi backend mà create
/// lane resolve được nguồn.)
#[test]
fn host_tool_backends_reflect_their_records() {
    assert!(is_supported("web", "django"));
    assert!(is_supported("web", "laravel"));
}

/// Supported frameworks are returned per core and never for an unknown core.
/// (Framework hỗ trợ trả theo core; core lạ trả rỗng.)
#[test]
fn supported_frameworks_is_scoped_per_core() {
    assert!(supported_frameworks("web").contains(&"react-vite"));
    assert!(!supported_frameworks("web").contains(&"bevy"));
    assert!(supported_frameworks("game").contains(&"bevy"));
    assert!(!supported_frameworks("game").contains(&"react-vite"));
    assert!(supported_frameworks("lib").contains(&"rust"));
    assert!(!supported_frameworks("lib").contains(&"kotlin"));
    assert!(supported_frameworks("unknown-core").is_empty());
}

/// `library` and `clo` are CLI aliases that must resolve to their registry table.
/// (`library` và `clo` là alias CLI phải quy về đúng bảng registry.)
#[test]
fn cli_aliases_resolve_to_the_registry_tables() {
    assert_eq!(supported_frameworks("library"), supported_frameworks("lib"));
    assert_eq!(supported_frameworks("cloud"), supported_frameworks("clo"));
    // The layer/registry namespace for the Cloud core is its published
    // directory name; the project identity stays `clo`.
    // (Namespace layer/registry là tên thư mục đã publish; identity vẫn `clo`.)
    assert_eq!(ecosystem_for_core("clo"), "cloud");
    assert_eq!(template_registry_namespace("clo"), "cloud");
    assert_eq!(ecosystem_for_core("web"), "web");
    assert_eq!(template_registry_namespace("web"), "web");
    assert_eq!(ecosystem_for_core("library"), "library");
}

/// A framework from another core is never accepted for this core.
/// (Framework của core khác không bao giờ được nhận cho core này.)
#[test]
fn names_do_not_leak_between_cores() {
    for (core, foreign) in [
        ("web", "bevy"),
        ("game", "react-vite"),
        ("lib", "flutter"),
        ("app", "cargo"),
        ("iot", "pulumi"),
        ("clo", "godot"),
        ("cicd", "express"),
        ("hardware", "bevy"),
    ] {
        let registry_core = ecosystem_for_core(core);
        assert!(
            !is_supported(registry_core, foreign),
            "'{foreign}' must not be accepted for core '{core}'"
        );
    }
}

/// Every declared id is non-empty and lowercase kebab-case.
/// (Mọi id khai báo là kebab-case chữ thường, không rỗng.)
#[test]
fn every_id_is_kebab_case() {
    let web_all = web_all_ids();
    for id in web_all
        .iter()
        .chain(APP.iter())
        .chain(LIB.iter())
        .chain(AI.iter())
    {
        assert!(!id.is_empty(), "empty framework id");
        assert!(
            id.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
            "'{id}' breaks kebab-case: only lowercase ASCII, digits and '-' are allowed"
        );
    }
}

/// Every alias resolves to a canonical id, and that id is itself accepted.
///
/// The property is stated through the public gate rather than by importing the
/// normaliser: if `is_supported` accepts the alias, the canonical name must be
/// reachable too, otherwise the alias would be the only way in.
/// (Mọi alias quy về một id chuẩn và id đó cũng được nhận — phát biểu qua cổng
/// công khai thay vì import hàm chuẩn hoá.)
#[test]
fn every_alias_is_itself_accepted_and_its_target_is_too() {
    for alias in WEB_ALIAS {
        assert!(
            is_supported("web", alias),
            "alias '{alias}' is accepted by create-web but rejected by the gate"
        );
    }
    // The canonical targets the aliases stand for.
    for canonical in [
        "vanilla",
        "remix",
        "express",
        "gin",
        "spring-boot",
        "laravel",
        "fastapi",
        "axum",
    ] {
        assert!(
            is_supported("web", canonical),
            "canonical id '{canonical}' must be accepted"
        );
    }
}

/// Validation helpers catch duplicate ids within arrays.
/// (Helper validation bắt id trùng lặp trong mảng.)
#[test]
fn validate_no_duplicates_catches_duplicates() {
    assert!(validate_no_duplicates(&["a", "b", "c"]).is_ok());
    assert_eq!(validate_no_duplicates(&["a", "b", "a"]), Err("a"));
    assert_eq!(validate_no_duplicates(&["x", "x"]), Err("x"));
}

/// All framework IDs must be globally unique (except Web families).
/// (Toàn bộ id framework phải unique toàn cục trừ nhóm Web.)
#[test]
fn all_framework_ids_are_globally_unique() {
    match validate_all_ids_unique() {
        Ok(()) => (),
        Err(msg) => panic!("Framework ID validation failed: {}", msg),
    }
}

/// Framework detection from markers works for known frameworks.
/// (Detection framework từ marker hoạt động cho framework đã biết.)
#[test]
fn detect_framework_from_markers_finds_django() {
    let temp = std::env::temp_dir().join("mgc-test-django-marker");
    std::fs::create_dir_all(&temp).ok();
    std::fs::write(temp.join("manage.py"), b"#!/usr/bin/env python").ok();

    let detected = detect_framework_from_markers("web", &temp);
    assert_eq!(detected, Some("django"));

    std::fs::remove_dir_all(&temp).ok();
}

/// Framework detection returns None when markers don't match.
/// (Detection trả None khi marker không khớp.)
#[test]
fn detect_framework_from_markers_returns_none_when_no_match() {
    let temp = std::env::temp_dir().join("mgc-test-no-marker");
    std::fs::create_dir_all(&temp).ok();

    let detected = detect_framework_from_markers("web", &temp);
    assert_eq!(detected, None);

    std::fs::remove_dir_all(&temp).ok();
}
