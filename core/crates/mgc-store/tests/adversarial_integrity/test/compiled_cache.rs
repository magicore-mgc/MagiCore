//! Compiled Cache tests. / Test compiled cache.

use crate::adversarial_integrity::support::tmp_store_dir;
use mgc_store::cas::{CompilationKey, ContentStore, Loader};
use std::fs;

pub(crate) fn compiled_cache_rejects_corrupt_json_at_get() {
    // A torn/corrupt cache entry must fail at GET with a hard error — never
    // be silently served as a compiled module.
    // (Entry cache đứt/hỏng phải fail tại GET với lỗi cứng — không bao giờ
    // được phục vụ im lặng như một compiled module.)
    let root = tmp_store_dir("compiled-corrupt");
    let store = ContentStore::new(root.clone()).unwrap();
    let cache = store.compiled_cache();
    // Full compilation key (P0-1 audit vòng-4).
    // (Key biên dịch đầy đủ (P0-1 audit vòng-4).)
    let key = CompilationKey::new(
        "a1b2c3d4e5f60718a9b0c1d2e3f40516a7b8c9d0e1f2a3b4c5d6e7f80910a1b2",
        Loader::Ts,
        "esbuild-rs",
        "0.13.8",
        "{}",
    )
    .unwrap();

    let module = mgc_store::cas::CompiledModule {
        js: "ok".into(),
        source_map: None,
    };
    cache.put(&key, &module).unwrap();

    // Corrupt the entry behind the cache's back (path from the key's
    // master digest, mirroring module_path).
    // (Sửa entry sau lưng cache (path từ master digest của key, phản chiếu
    // module_path).)
    let master = key.master_digest();
    let hex = master.as_hex();
    let algo = root.join("compiled").join("blake3");
    let path = algo.join(&hex[..2]).join(format!("{hex}.json"));
    fs::write(&path, b"{TORN-JSON").unwrap();

    let res = cache.get(&key);
    assert!(
        res.is_err(),
        "corrupt compiled-cache JSON must fail at get()"
    );
}
