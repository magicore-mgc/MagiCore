//! Concurrent same-key writes preserve deterministic cache integrity.
//! Ghi đồng thời cùng key giữ tính toàn vẹn cache tất định.

use crate::compiled_cache_integrity::support::{SRC, module, scan_temps, tmp_store_dir};
use mgc_store::cas::{CompilationKey, ContentStore, Loader, StoreError};
use std::sync::Arc;

pub(crate) fn concurrent_same_key_same_payload_all_succeed() {
    // 8 writers, SAME key, IDENTICAL payload: everyone succeeds (first
    // publishes, losers verify+dedup). No temp leftovers at any depth.
    // (8 writer, CÙNG key, payload GIỐNG HỆT: tất cả thành công (writer
    // đầu publish, thua verify+dedup). Không sót temp ở mọi độ sâu.)
    let root = tmp_store_dir("race-same");
    let store = ContentStore::new(root.clone()).unwrap();
    let cache = Arc::new(store.compiled_cache());
    let key =
        Arc::new(CompilationKey::new(SRC, Loader::Tsx, "esbuild-rs", "0.13.8", "{}").unwrap());

    let handles: Vec<_> = (0..8)
        .map(|_| {
            let cache = Arc::clone(&cache);
            let key = Arc::clone(&key);
            std::thread::spawn(move || cache.put(&key, &module("console.log(1)")))
        })
        .collect();
    for (i, h) in handles.into_iter().enumerate() {
        let res = h.join().expect("writer must not panic");
        assert!(res.is_ok(), "identical writer {i} must succeed: {res:?}");
    }

    let m = cache.get(&key).unwrap().expect("entry must exist");
    assert_eq!(m.js, "console.log(1)");
    let leftovers = scan_temps(&root);
    assert!(leftovers.is_empty(), "temp leftovers: {leftovers:?}");
}

pub(crate) fn sequential_same_key_divergent_payload_conflicts() {
    // The API-level contract (P0-2): a SECOND put of a DIFFERENT payload
    // under the same key must NOT silently win or lose — it must surface a
    // typed CacheConflict. The first (valid) winner stays in place.
    // (Hợp đồng cấp API (P0-2): put THỨ HAI với payload KHÁC dưới cùng key
    // KHÔNG được thắng/thua im lặng — phải báo CacheConflict typed. Winner
    // đầu (hợp lệ) giữ nguyên vị trí.)
    let root = tmp_store_dir("conflict-seq");
    let store = ContentStore::new(root.clone()).unwrap();
    let cache = store.compiled_cache();
    let key = CompilationKey::new(SRC, Loader::Ts, "esbuild-rs", "0.13.8", "{}").unwrap();

    cache.put(&key, &module("output-A")).unwrap();
    let res = cache.put(&key, &module("output-B"));
    match res {
        Err(StoreError::CacheConflict { .. }) => {}
        other => panic!("divergent same-key put must conflict, got: {other:?}"),
    }
    // The valid winner is still in place.
    // (Winner hợp lệ vẫn ở chỗ.)
    assert_eq!(cache.get(&key).unwrap().unwrap().js, "output-A");
}

pub(crate) fn concurrent_same_key_divergent_payloads_deterministic_outcome() {
    // 8 writers, same key, FOUR divergent payloads racing: the served
    // entry is ALWAYS one of the racing payloads — intact, verifiable,
    // never a mix or a torn file. Losers either dedup (same payload as
    // winner) or get a typed conflict/corrupt error. Deterministic
    // first-wins via hard_link on Unix AND Windows.
    //
    // (8 writer, cùng key, BỐN payload phân kỳ đua: entry được phục vụ
    // luôn luôn là MỘT trong các payload đua — nguyên vẹn, verify được,
    // không trộn, không đứt. Bên thua hoặc dedup (cùng payload với
    // winner) hoặc nhận lỗi conflict/corrupt typed. First-wins tất định
    // qua hard_link trên Unix VÀ Windows.)
    let root = tmp_store_dir("race-divergent");
    let store = ContentStore::new(root.clone()).unwrap();
    let cache = Arc::new(store.compiled_cache());
    let key =
        Arc::new(CompilationKey::new(SRC, Loader::Tsx, "esbuild-rs", "0.13.8", "{}").unwrap());

    let payloads = ["pa", "pb", "pc", "pd"];
    let handles: Vec<_> = (0..8)
        .map(|i| {
            let cache = Arc::clone(&cache);
            let key = Arc::clone(&key);
            let payload = payloads[i % payloads.len()].to_string();
            std::thread::spawn(move || {
                let res = cache.put(&key, &module(&payload));
                (payload, res)
            })
        })
        .collect();

    // REVIEW vòng-2 hardening: EVERY Ok must be the payload that actually
    // won, EVERY Err must be a typed conflict/corrupt — a fake Ok for a
    // divergent payload would be exactly the silent-nondeterminism bug the
    // audit flagged. We can only check after collecting all results, so
    // gather first, assert the served payload afterwards.
    // (Harden vòng-2 review: MỌI Ok phải là payload thực sự thắng, MỌI Err
    // phải là conflict/corrupt typed — Ok giả cho payload phân kỳ chính là
    // bug nondeterminism im lặng mà audit chỉ ra. Chỉ check được sau khi
    // thu đủ kết quả nên gom trước, assert payload được serve sau.)
    let mut results = Vec::new();
    for h in handles {
        results.push(h.join().expect("writer must not panic"));
    }

    // The surviving entry is intact and IS one of the racing payloads.
    // (Entry sống sót nguyên vẹn và LÀ một trong các payload đua.)
    let served = cache.get(&key).unwrap().expect("entry must exist").js;
    assert!(
        payloads.contains(&served.as_str()),
        "served payload must be one of the racers, got '{served}'"
    );

    let mut winners = 0;
    for (payload, res) in &results {
        match res {
            Ok(()) => {
                // Only the WINNING payload (and its dedups) may report Ok.
                // (Chỉ payload THẮNG (và dedup của nó) được báo Ok.)
                assert_eq!(
                    payload, &served,
                    "put() reported Ok for '{payload}' but '{served}' was served — fake Ok"
                );
                winners += 1;
            }
            Err(StoreError::CacheConflict { .. }) | Err(StoreError::CacheCorrupt { .. }) => {
                // Typed race outcome — the documented loser contract.
                // (Kết quả race typed — hợp đồng bên thua được document.)
            }
            Err(other) => {
                panic!("unexpected race error (must be CacheConflict/CacheCorrupt): {other:?}")
            }
        }
    }
    assert!(
        winners >= 1,
        "at least the first writer must win (winners={winners})"
    );
    let leftovers = scan_temps(&root);
    assert!(leftovers.is_empty(), "temp leftovers: {leftovers:?}");
}
