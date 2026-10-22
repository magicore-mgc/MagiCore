//! Concurrency tests. / Test concurrency.

use super::filesystem_staging::scan_store_temps;
use crate::adversarial_integrity::support::tmp_store_dir;
use mgc_store::cas::integrity::TarballEntry;
use mgc_store::cas::{CompilationKey, ContentStore, IntegrityHash, Loader};
use std::fs;
use std::sync::Arc;

pub(crate) fn concurrent_imports_same_content_all_succeed() {
    let root = tmp_store_dir("concurrent-import");
    let store = Arc::new(ContentStore::new(root).unwrap());
    let data: Vec<u8> = b"concurrent-payload-0123456789".to_vec();

    let handles: Vec<_> = (0..8)
        .map(|i| {
            let store = Arc::clone(&store);
            let data = data.clone();
            std::thread::spawn(move || {
                let h = store.import_bytes(&data).unwrap();
                Some((i, h.as_hex().to_string()))
            })
        })
        .collect();

    let mut hashes = Vec::new();
    for h in handles {
        hashes.push(h.join().unwrap().unwrap());
    }
    let first = hashes[0].1.clone();
    assert!(
        hashes.iter().all(|(_, h)| *h == first),
        "all concurrent imports must agree on the digest"
    );
}

pub(crate) fn concurrent_compiled_cache_puts_never_collide() {
    let root = tmp_store_dir("compiled-concurrent");
    let store = ContentStore::new(root.clone()).unwrap();
    let cache = store.compiled_cache();

    // Full compilation key (P0-1 audit vòng-4): context-full, not
    // source-only.
    // (Key biên dịch đầy đủ (P0-1 audit vòng-4): đủ ngữ cảnh, không chỉ
    // source.)
    let key = CompilationKey::new(
        // blake3 hex of b"source" — stable content digest for the key.
        // (hex blake3 của b"source" — digest nội dung ổn định cho key.)
        "6b84be2b6c09bf573b8e8c663182b1e7d0d5e79ba1cd9e2d5a6f4d3e3f2a1b0c",
        Loader::Tsx,
        "esbuild-rs",
        "0.13.8",
        "{}",
    )
    .unwrap();
    let cache = Arc::new(cache);
    // Audit vòng-3 P0-5: every writer's result is COLLECTED and asserted —
    // a green test must not hide a failed writer behind its siblings, and
    // no temp may be left behind at ANY depth.
    // (P0-5 audit vòng-3: kết quả MỖI writer được THU và assert — test xanh
    // không được giấu writer fail sau lưng, và không temp sót ở MỌI độ sâu.)
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let cache = Arc::clone(&cache);
            let key = key.clone();
            std::thread::spawn(move || {
                let module = mgc_store::cas::CompiledModule {
                    js: "console.log(1)".into(),
                    source_map: None,
                };
                cache.put(&key, &module)
            })
        })
        .collect();
    for (i, h) in handles.into_iter().enumerate() {
        let res = h.join().expect("compiled-cache writer must not panic");
        assert!(
            res.is_ok(),
            "compiled-cache writer {i} must succeed (atomic put, loser verifies winner): {res:?}"
        );
    }
    // The surviving entry is valid JSON with the right payload.
    // (Entry sống sót là JSON hợp lệ với payload đúng.)
    let module = cache.get(&key).unwrap().expect("entry must exist");
    assert_eq!(module.js, "console.log(1)");
    // Recursive temp scan under the whole store root.
    // (Quét temp đệ quy toàn bộ gốc store.)
    let leftovers = scan_store_temps(&root);
    assert!(
        leftovers.is_empty(),
        "compiled-cache temp leftovers: {leftovers:?}"
    );
}

pub(crate) fn concurrent_tarball_import_same_content_is_atomic() {
    // 8 threads import the SAME tarball entry concurrently: every writer
    // must either publish the blob (first) or verify the winner (others) —
    // no AlreadyExists errors, no partial blob at the final CAS path.
    // 8 thread import CÙNG entry tarball: writer đầu publish, 7 writer kia
    // verify winner — không lỗi AlreadyExists, không blob nửa vời ở path
    // CAS cuối.
    let root = tmp_store_dir("tarball-atomic");
    let store = Arc::new(ContentStore::new(root.clone()).unwrap());

    let data = b"tarball-atomic-payload-0123456789".to_vec();
    let entries = || {
        vec![TarballEntry {
            path: "pkg/a.txt".into(),
            data: data.clone(),
            executable: false,
        }]
    };

    let handles: Vec<_> = (0..8)
        .map(|_| {
            let store = Arc::clone(&store);
            let entries = entries();
            std::thread::spawn(move || store.import_tarball_entries(entries))
        })
        .collect();

    // Every writer must SUCCEED (winner publishes, losers verify winner).
    // Mọi writer phải THÀNH CÔNG (winner publish, thua verify winner).
    for (i, h) in handles.into_iter().enumerate() {
        let res = h.join().unwrap();
        assert!(
            res.is_ok(),
            "tarball writer {i} failed race handling: {:?}",
            res.err()
        );
    }

    // The final blob is complete and correct.
    // Blob cuối hoàn chỉnh và đúng.
    let hash = IntegrityHash::from_bytes(&data, false);
    let blob = fs::read(hash.cas_path(&root)).unwrap();
    assert_eq!(blob, data);

    // No leftover temps in the CAS tmp dir.
    // Không còn temp sót trong tmp CAS.
    let leftovers: Vec<_> = fs::read_dir(root.join("tmp"))
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .filter(|e| e.file_name().to_string_lossy().starts_with("write-bytes"))
                .collect()
        })
        .unwrap_or_default();
    assert!(leftovers.is_empty(), "CAS tmp leftovers: {leftovers:?}");
}
