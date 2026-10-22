//! Compiled-cache record validation, schema handling and quarantine behavior.
//! Xác thực record compiled-cache, xử lý schema và quarantine.

use crate::compiled_cache_integrity::support::{
    SRC, entry_path, module, out_digest, tmp_store_dir,
};
use mgc_store::cas::{
    COMPILED_CACHE_SCHEMA_VERSION, CompilationKey, CompiledCache, ContentStore, Loader, StoreError,
};
use std::fs;

pub(crate) fn poisoned_valid_json_is_rejected_and_quarantined() {
    // THE P0-3 audit case: an attacker swaps the entry for a WELL-FORMED
    // JSON with a malicious js payload. get() must verify the embedded
    // output digest, quarantine the poison, and fail — the payload is
    // never served.
    // (Đúng case P0-3 của audit: kẻ tấn công thay entry bằng JSON HỢP LỆ
    // mang js độc. get() phải verify digest nhúng, cách ly độc, và fail —
    // payload không bao giờ được phục vụ.)
    let root = tmp_store_dir("poison-json");
    let store = ContentStore::new(root.clone()).unwrap();
    let cache: CompiledCache = store.compiled_cache();
    let key = CompilationKey::new(SRC, Loader::Tsx, "esbuild-rs", "0.13.8", "{}").unwrap();

    cache.put(&key, &module("console.log('clean')")).unwrap();

    // Poison behind the cache's back: valid JSON, digest still names the
    // CLEAN payload while the js field carries the malicious code.
    // (Đầu độc sau lưng cache: JSON hợp lệ, digest vẫn trỏ payload SẠCH
    // còn field js mang code độc.)
    let path = entry_path(&root, &key);
    let poison = format!(
        r#"{{"schema_version":{COMPILED_CACHE_SCHEMA_VERSION},"key":{},"output_digest":"{}","js":"maliciousCode()","source_map":null}}"#,
        serde_json::to_string(&key).unwrap(),
        out_digest("console.log('clean')", None)
    );
    fs::write(&path, poison).unwrap();

    match cache.get(&key) {
        Err(StoreError::CacheCorrupt { .. }) => {}
        other => panic!("poisoned entry must fail with CacheCorrupt, got: {other:?}"),
    }

    // The poison was quarantined for forensics.
    // (Độc đã bị cách ly để điều tra.)
    let qdir = root.join("quarantine");
    let q: Vec<_> = fs::read_dir(&qdir)
        .map(|rd| rd.filter_map(|e| e.ok()).collect())
        .unwrap_or_default();
    assert!(!q.is_empty(), "poisoned entry must land in quarantine/");

    // And the slot self-heals: the next put publishes a fresh record and
    // get serves it.
    // (Slot tự sửa: put kế tiếp publish record mới và get phục vụ nó.)
    cache.put(&key, &module("console.log('clean')")).unwrap();
    let healed = cache.get(&key).unwrap().unwrap();
    assert_eq!(healed.js, "console.log('clean')");
}

pub(crate) fn valid_json_with_wrong_embedded_key_is_rejected() {
    // A record whose embedded key names a DIFFERENT compilation context,
    // copied onto this key's path, is a cross-context steal attempt
    // (P0-1+P0-3): quarantine + CacheCorrupt, never served.
    // (Record mà key nhúng chỉ DANH NGỮ cảnh biên dịch KHÁC, chép sang path
    // key này, là trộm chéo ngữ cảnh (P0-1+P0-3): cách ly + CacheCorrupt,
    // không phục vụ.)
    let root = tmp_store_dir("wrong-key");
    let store = ContentStore::new(root.clone()).unwrap();
    let cache = store.compiled_cache();
    let requested = CompilationKey::new(SRC, Loader::Ts, "esbuild-rs", "0.13.8", "{}").unwrap();
    let other = CompilationKey::new(SRC, Loader::Tsx, "esbuild-rs", "0.13.8", "{}").unwrap();

    // Hand-craft a VALID record for `other` (correct digest for "stolen").
    // (Tay dựng record HỢP LỆ cho `other` (digest đúng cho "stolen").)
    let rec = serde_json::json!({
        "schema_version": COMPILED_CACHE_SCHEMA_VERSION,
        "key": other,
        "output_digest": out_digest("stolen", None),
        "js": "stolen",
        "source_map": null,
    });
    let requested_path = entry_path(&root, &requested);
    fs::create_dir_all(requested_path.parent().unwrap()).unwrap();
    fs::write(&requested_path, serde_json::to_vec(&rec).unwrap()).unwrap();

    match cache.get(&requested) {
        Err(StoreError::CacheCorrupt { .. }) => {}
        other => panic!("key-mismatched record must fail with CacheCorrupt, got: {other:?}"),
    }
    // Nothing was served, and the tampered bytes went to quarantine.
    // (Không phục vụ gì, bytes bị can thiệp vào quarantine.)
    let qdir = root.join("quarantine");
    let q: Vec<_> = fs::read_dir(&qdir)
        .map(|rd| rd.filter_map(|e| e.ok()).collect())
        .unwrap_or_default();
    assert!(
        !q.is_empty(),
        "key-mismatched entry must land in quarantine/"
    );
}

pub(crate) fn stale_schema_entry_reports_miss_and_is_replaced() {
    // An old-schema (v0) entry is a LEGIT old cache, not corruption: get
    // reports a miss; the next put replaces it (P0-3 migration path).
    // (Entry schema cũ (v0) là cache hợp lệ của thời trước, không phải hỏng:
    // get báo miss; put kế tiếp thay nó (đường migration P0-3).)
    let root = tmp_store_dir("stale-schema");
    let store = ContentStore::new(root.clone()).unwrap();
    let cache = store.compiled_cache();
    let key = CompilationKey::new(SRC, Loader::Ts, "esbuild-rs", "0.13.8", "{}").unwrap();

    // Hand-write a v0 record (schema_version missing → serde default 0).
    // (Tay viết record v0 (thiếu schema_version → serde default 0).)
    let path = entry_path(&root, &key);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let stale = format!(
        r#"{{"key":{},"output_digest":"{}","js":"old","source_map":null}}"#,
        serde_json::to_string(&key).unwrap(),
        out_digest("old", None)
    );
    fs::write(&path, stale).unwrap();

    // Miss, not an error.
    // (Miss, không phải lỗi.)
    assert!(cache.get(&key).unwrap().is_none());

    // The following put replaces it and the fresh record serves.
    // (Put kế tiếp thay nó và record mới phục vụ được.)
    cache.put(&key, &module("fresh")).unwrap();
    assert_eq!(cache.get(&key).unwrap().unwrap().js, "fresh");
}

pub(crate) fn truncated_entry_fails_at_get_and_quarantines() {
    // Torn bytes at the entry path → CacheCorrupt + quarantine (the old
    // design only surfaced a parse error; now forensics + typed error).
    // (Bytes đứt ở path entry → CacheCorrupt + cách ly (thiết kế cũ chỉ
    // báo lỗi parse; giờ có forensics + lỗi typed).)
    let root = tmp_store_dir("torn");
    let store = ContentStore::new(root.clone()).unwrap();
    let cache = store.compiled_cache();
    let key = CompilationKey::new(SRC, Loader::Ts, "esbuild-rs", "0.13.8", "{}").unwrap();

    cache.put(&key, &module("ok")).unwrap();
    let path = entry_path(&root, &key);
    fs::write(&path, b"{TORN-JSON").unwrap();

    match cache.get(&key) {
        Err(StoreError::CacheCorrupt { .. }) => {}
        other => panic!("torn entry must fail with CacheCorrupt, got: {other:?}"),
    }
    let qdir = root.join("quarantine");
    let q: Vec<_> = fs::read_dir(&qdir)
        .map(|rd| rd.filter_map(|e| e.ok()).collect())
        .unwrap_or_default();
    assert!(!q.is_empty(), "torn entry must land in quarantine/");
}

pub(crate) fn record_with_garbage_loader_json_fails_to_parse() {
    // A hand-crafted record whose key carries "garbage-loader" used to
    // deserialize into a free-form CompilationKey; with the closed-set
    // Loader the WHOLE record fails to parse → get() quarantines it and
    // fails closed (P0-1).
    // (Record tự chế với key mang "garbage-loader" từng deserialize thành
    // CompilationKey string tự do; với Loader tập đóng CẢ record fail
    // parse → get() cách ly và fail cứng (P0-1).)
    let root = tmp_store_dir("garbage-loader");
    let store = ContentStore::new(root.clone()).unwrap();
    let cache = store.compiled_cache();
    let key = CompilationKey::new(SRC, Loader::Ts, "esbuild-rs", "0.13.8", "{}").unwrap();

    let rec = serde_json::json!({
        "schema_version": COMPILED_CACHE_SCHEMA_VERSION,
        "key": {
            "schema_version": COMPILED_CACHE_SCHEMA_VERSION,
            "source_digest": SRC,
            "loader": "garbage-loader",
            "compiler": "esbuild-rs",
            "compiler_version": "0.13.8",
            "options_digest": out_digest("{}", None),
        },
        "output_digest": out_digest("x", None),
        "js": "x",
        "source_map": null,
    });
    let path = entry_path(&root, &key);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, serde_json::to_vec(&rec).unwrap()).unwrap();

    match cache.get(&key) {
        Err(StoreError::CacheCorrupt { .. }) => {}
        other => panic!("garbage-loader record must be CacheCorrupt, got: {other:?}"),
    }
}

pub(crate) fn record_with_junk_digest_key_fails_to_parse() {
    // Junk (non-BLAKE3) digests in a record's key fail CompilationKey's
    // deserialize-validation → corrupt, never served (P0-1).
    // (Digest rác (không BLAKE3) trong key của record fail validate khi
    // deserialize CompilationKey → hỏng, không bao giờ phục vụ (P0-1).)
    let root = tmp_store_dir("junk-digest");
    let store = ContentStore::new(root.clone()).unwrap();
    let cache = store.compiled_cache();
    let key = CompilationKey::new(SRC, Loader::Ts, "esbuild-rs", "0.13.8", "{}").unwrap();

    let rec = serde_json::json!({
        "schema_version": COMPILED_CACHE_SCHEMA_VERSION,
        "key": {
            "schema_version": COMPILED_CACHE_SCHEMA_VERSION,
            "source_digest": "not-a-blake3-digest",
            "loader": "ts",
            "compiler": "esbuild-rs",
            "compiler_version": "0.13.8",
            "options_digest": "also-not-blake3",
        },
        "output_digest": out_digest("x", None),
        "js": "x",
        "source_map": null,
    });
    let path = entry_path(&root, &key);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, serde_json::to_vec(&rec).unwrap()).unwrap();

    match cache.get(&key) {
        Err(StoreError::CacheCorrupt { .. }) => {}
        other => panic!("junk-digest record must be CacheCorrupt, got: {other:?}"),
    }
}
