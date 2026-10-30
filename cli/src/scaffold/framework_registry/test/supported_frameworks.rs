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
    for id in WEB_FRONTEND
        .iter()
        .chain(WEB_BACKEND.iter())
        .chain(WEB_FULLSTACK.iter())
    {
        assert!(
            WEB_ALL.contains(id),
            "'{id}' is accepted by is_supported but missing from WEB_ALL, so the error \
             message would omit a name the user can actually type"
        );
    }
}

/// No duplicates: a repeated id makes the count in the error message wrong.
/// (Không trùng lặp: id lặp làm số đếm trong thông điệp lỗi sai.)
#[test]
fn the_diagnostic_list_has_no_duplicates() {
    let mut seen = std::collections::BTreeSet::new();
    for id in WEB_ALL {
        assert!(seen.insert(*id), "'{id}' appears twice in WEB_ALL");
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
    for id in WEB_ALL
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
