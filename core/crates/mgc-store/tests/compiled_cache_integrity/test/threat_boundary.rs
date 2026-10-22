//! Documented same-user compiled-cache threat boundary.
//! Biên đe dọa compiled-cache same-user đã được ghi nhận.

use crate::compiled_cache_integrity::support::{SRC, entry_path, out_digest, tmp_store_dir};
use mgc_store::cas::{COMPILED_CACHE_SCHEMA_VERSION, CompilationKey, ContentStore, Loader};
use std::fs;

pub(crate) fn attacker_recompute_is_served_documented_boundary() {
    // DOCUMENTED BOUNDARY (P0-4): a same-user attacker who recomputes the
    // output digest for a forged record PRODUCES a record indistinguish-
    // able from a legit one — the cache is self-VALIDATING (corruption/
    // torn writes), not self-AUTHENTICATING. This test pins the boundary:
    // the forged-but-consistent record IS served. Changing this behavior
    // requires keyed MACs / signatures (remote-cache scope, not local).
    // (BIÊN ĐÃ GHI RÕ (P0-4): attacker cùng user recompute digest output
    // cho record giả sẽ tạo record KHÔNG PHÂN BIỆT được với record thật —
    // cache TỰ KIỂM TRA (hỏng/đứt ghi), không TỰ XÁC THỰC. Test ghim
    // biên: record giả-nhưng-nhất-quán VẪN được phục vụ. Đổi hành vi này
    // cần MAC keyed / chữ ký (scope remote-cache, không phải local).)
    let root = tmp_store_dir("attacker-recompute");
    let store = ContentStore::new(root.clone()).unwrap();
    let cache = store.compiled_cache();
    let key = CompilationKey::new(SRC, Loader::Tsx, "esbuild-rs", "0.13.8", "{}").unwrap();

    // Attacker forges a fully-consistent record (digest matches payload).
    // (Attacker giả record nhất quán hoàn toàn (digest khớp payload).)
    let forged = serde_json::json!({
        "schema_version": COMPILED_CACHE_SCHEMA_VERSION,
        "key": key,
        "output_digest": out_digest("attacker-payload()", None),
        "js": "attacker-payload()",
        "source_map": null,
    });
    let path = entry_path(&root, &key);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, serde_json::to_vec(&forged).unwrap()).unwrap();

    // The cache serves it: this IS the accepted threat-model boundary.
    // (Cache phục vụ nó: ĐÂY là biên threat model được chấp nhận.)
    let served = cache.get(&key).unwrap().unwrap();
    assert_eq!(served.js, "attacker-payload()");
}
