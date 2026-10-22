//! Compiled-cache key identity varies by loader, compiler version and options.
//! Khóa cache biên dịch phân biệt loader, phiên bản compiler và options.

use crate::compiled_cache_integrity::support::{SRC, module, tmp_store_dir};
use mgc_store::cas::{CompilationKey, ContentStore, Loader};

pub(crate) fn same_source_different_loaders_get_distinct_entries() {
    // The EXACT collision the audit found: one source text, two loaders.
    // The .ts and .tsx keys must resolve to DIFFERENT entries — neither
    // may serve the other's output.
    // (Đúng collision audit chỉ ra: cùng text source, hai loader. Key .ts
    // với .tsx phải ra 2 entry KHÁC NHAU — không bên nào được phục vụ
    // output của bên kia.)
    let root = tmp_store_dir("loader-split");
    let store = ContentStore::new(root.clone()).unwrap();
    let cache = store.compiled_cache();

    let ts_key = CompilationKey::new(
        SRC,
        Loader::Ts,
        "esbuild-rs",
        "0.13.8",
        r#"{"platform":"browser"}"#,
    )
    .unwrap();
    let tsx_key = CompilationKey::new(
        SRC,
        Loader::Tsx,
        "esbuild-rs",
        "0.13.8",
        r#"{"platform":"browser"}"#,
    )
    .unwrap();
    assert_ne!(
        ts_key.master_digest().as_hex(),
        tsx_key.master_digest().as_hex(),
        "loader must be part of the cache address (P0-1)"
    );

    cache.put(&ts_key, &module("// compiled as TS")).unwrap();
    cache
        .put(&tsx_key, &module("// compiled as TSX (JSX enabled)"))
        .unwrap();

    let ts_hit = cache.get(&ts_key).unwrap().unwrap();
    let tsx_hit = cache.get(&tsx_key).unwrap().unwrap();
    assert_eq!(ts_hit.js, "// compiled as TS");
    assert_eq!(tsx_hit.js, "// compiled as TSX (JSX enabled)");
}

pub(crate) fn same_source_different_compiler_versions_get_distinct_entries() {
    // Bumping the compiler version must fork the cache — an esbuild 0.13.8
    // entry can never be served to a 0.14.0 pipeline (P0-1).
    // (Nâng phiên bản compiler phải rẽ nhánh cache — entry của esbuild
    // 0.13.8 không bao giờ được phục vụ cho pipeline 0.14.0 (P0-1).)
    let root = tmp_store_dir("compiler-split");
    let store = ContentStore::new(root.clone()).unwrap();
    let cache = store.compiled_cache();

    let v1 = CompilationKey::new(SRC, Loader::Tsx, "esbuild-rs", "0.13.8", "{}").unwrap();
    let v2 = CompilationKey::new(SRC, Loader::Tsx, "esbuild-rs", "0.14.0", "{}").unwrap();
    assert_ne!(v1.master_digest().as_hex(), v2.master_digest().as_hex());

    cache.put(&v1, &module("out-v1")).unwrap();
    cache.put(&v2, &module("out-v2")).unwrap();
    assert_eq!(cache.get(&v1).unwrap().unwrap().js, "out-v1");
    assert_eq!(cache.get(&v2).unwrap().unwrap().js, "out-v2");
}

pub(crate) fn same_source_different_options_get_distinct_entries() {
    // Options are part of the key (P0-1): platform browser vs node fork
    // the cache.
    // (Options nằm trong key (P0-1): platform browser với node rẽ nhánh
    // cache.)
    let root = tmp_store_dir("options-split");
    let store = ContentStore::new(root.clone()).unwrap();
    let cache = store.compiled_cache();

    let browser = CompilationKey::new(
        SRC,
        Loader::Ts,
        "esbuild-rs",
        "0.13.8",
        r#"{"platform":"browser"}"#,
    )
    .unwrap();
    let node = CompilationKey::new(
        SRC,
        Loader::Ts,
        "esbuild-rs",
        "0.13.8",
        r#"{"platform":"node"}"#,
    )
    .unwrap();
    assert_ne!(
        browser.master_digest().as_hex(),
        node.master_digest().as_hex()
    );

    cache.put(&browser, &module("browser-bundle")).unwrap();
    cache.put(&node, &module("node-bundle")).unwrap();
    assert_eq!(cache.get(&browser).unwrap().unwrap().js, "browser-bundle");
    assert_eq!(cache.get(&node).unwrap().unwrap().js, "node-bundle");
}

pub(crate) fn malformed_key_inputs_are_rejected() {
    // Key validation is fail-closed: garbage digests / empty identity are
    // rejected at the door (P0-1 defense in depth). The closed-set Loader
    // enum makes "garbage-loader" a COMPILE-TIME impossibility now — the
    // runtime guard that remains is on digests and identity strings.
    // (Key validate fail-closed: digest rác / identity rỗng bị chặn ngay
    // tại cửa (phòng vệ nhiều lớp P0-1). Enum Loader tập đóng khiến
    // "garbage-loader" giờ là bất khả thi ở COMPILE-TIME — runtime guard
    // còn lại nằm ở digest và chuỗi identity.)
    assert!(CompilationKey::new("deadbeef", Loader::Ts, "esbuild-rs", "0.13.8", "{}").is_err());
    // Empty compiler / version still rejected.
    // (Compiler / version rỗng vẫn bị từ chối.)
    assert!(CompilationKey::new(SRC, Loader::Ts, "", "0.13.8", "{}").is_err());
    assert!(CompilationKey::new(SRC, Loader::Ts, "esbuild-rs", "", "{}").is_err());
    // Malformed options JSON still rejected.
    // (JSON options sai dạng vẫn bị từ chối.)
    assert!(CompilationKey::new(SRC, Loader::Ts, "esbuild-rs", "0.13.8", "not json").is_err());
    // The closed set: no loader exists for an unknown extension — the
    // dev-server derivation path returns None instead of a free string.
    // (Tập đóng: không có loader cho đuôi lạ — đường suy ra loader của
    // dev server trả None thay vì string tự do.)
    assert!(Loader::from_extension("garbage").is_none());
    assert!(
        Loader::from_extension("TS").is_some(),
        "extension match is case-insensitive"
    );
}
