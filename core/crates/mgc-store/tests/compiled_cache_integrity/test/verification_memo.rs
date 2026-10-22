//! Verification memo capacity, LRU accounting and mutation invalidation.
//! Dung lượng memo xác minh, kế toán LRU và vô hiệu khi blob bị sửa.

use crate::compiled_cache_integrity::support::tmp_store_dir;
use mgc_store::cas::ContentStore;
use std::fs;

#[cfg(unix)]
pub(crate) fn memo_is_bounded_and_evicts_lru() {
    // With a tiny capacity the memo MUST evict: touch capacity+2 blobs and
    // verify the eviction counter climbs while correctness holds (evicted
    // blobs re-verify from disk — never a wrong serve).
    // (Capacity nhỏ thì memo PHẢI đẩy entry: chạm capacity+2 blob và verify
    // bộ đếm eviction tăng trong khi correctness giữ nguyên (blob bị đẩy
    // re-verify từ đĩa — không bao giờ serve sai).)
    let root = tmp_store_dir("memo-lru");
    let store = ContentStore::new(root.clone()).unwrap();
    // NonZeroUsize at the TYPE level (P1-3): zero capacity is now
    // unrepresentable — the builder refuses it at compile time.
    // (NonZeroUsize ở tầng KIỂU (P1-3): capacity-0 giờ không thể biểu
    // diễn — builder chặn ngay lúc compile.)
    let tiny = store.with_memo_capacity(std::num::NonZeroUsize::new(4).unwrap());

    // Touch capacity+2 distinct blobs through export (verify+memoize).
    // (Chạm capacity+2 blob khác nhau qua export (verify + memo hóa).)
    for i in 0..6u32 {
        let data = format!("memo-blob-{i}").into_bytes();
        let hash = tiny.import_bytes(&data).unwrap();
        let dest = root.join(format!("out{i}.bin"));
        tiny.export_to(&hash, &dest).unwrap();
    }

    let stats = tiny.memo_stats();
    assert!(
        stats.evictions >= 2,
        "capacity-4 memo must evict ≥2 after 6 touches: {stats:?}"
    );
    assert!(stats.misses >= 6, "each first touch is a miss: {stats:?}");
}

#[cfg(unix)]
pub(crate) fn memo_replacement_is_not_counted_as_eviction() {
    // Eviction accounting (P1-3, Tech Lead vòng-5/6/7): lru 0.18 `push`
    // returns the REPLACED value for an existing key even when nothing
    // was evicted — the old `is_some()` check miscounted replacements as
    // evictions. Re-touching ONE blob below capacity must NEVER raise the
    // eviction counter; only a genuine capacity overflow may.
    // (Kế toán eviction (P1-3): lru 0.18 `push` trả value bị THAY THẾ cho
    // key đã tồn tại kể cả khi không đẩy entry nào — check `is_some()` cũ
    // đếm replacement thành eviction. Chạm lại MỘT blob dưới capacity
    // KHÔNG BAO GIỜ được tăng bộ đếm eviction; chỉ capacity overflow thật
    // mới được.)
    let root = tmp_store_dir("memo-replace");
    let store = ContentStore::new(root.clone()).unwrap();
    let tiny = store.with_memo_capacity(std::num::NonZeroUsize::new(4).unwrap());

    // Fill BELOW capacity (3 of 4) with distinct blobs.
    // (Điền DƯỚI capacity (3/4) bằng blob khác nhau.)
    let mut dests = Vec::new();
    for i in 0..3u32 {
        let data = format!("replace-blob-{i}").into_bytes();
        let hash = tiny.import_bytes(&data).unwrap();
        let dest = root.join(format!("r{i}.bin"));
        tiny.export_to(&hash, &dest).unwrap();
        dests.push((hash, dest));
    }
    let before = tiny.memo_stats();
    assert_eq!(
        before.evictions, 0,
        "3 touches of a 4-capacity memo evict nothing"
    );

    // Re-export the SAME blobs many times — pure memo hits/replacements,
    // still below capacity: evictions must stay 0. Each export needs a
    // FRESH destination (export_to refuses to overwrite).
    // (Export lại CÙNG blob nhiều lần — thuần hit/replacement, vẫn dưới
    // capacity: eviction phải giữ 0. Mỗi export cần đích MỚI (export_to
    // từ chối ghi đè).)
    for round in 0..5u32 {
        for (idx, (hash, _dest)) in dests.iter().enumerate() {
            let again = root.join(format!("again-{round}-{idx}.bin"));
            tiny.export_to(hash, &again).unwrap();
        }
    }
    let after = tiny.memo_stats();
    assert_eq!(
        after.evictions, 0,
        "re-touches below capacity are replacements, NOT evictions (P1-3): {after:?}"
    );
    assert!(
        after.hits >= 5 * 3,
        "re-exports must hit the memo: {after:?}"
    );
}

#[cfg(unix)]
pub(crate) fn memo_full_cache_replacement_is_not_counted_as_eviction() {
    // P1-1 (adversarial review vòng-9): fill the memo to EXACT capacity,
    // then re-verify ONE key it already holds. The old accounting
    // (`len_before == cap && len == cap`) counted that replacement as an
    // eviction. Contract: evictions stays 0 — only a FRESH key entering
    // a full cache evicts.
    // (P1-1: điền memo ĐÚNG capacity, rồi verify lại MỘT key nó đang giữ.
    // Kế toán cũ (`len_before == cap && len == cap`) đếm replacement đó
    // thành eviction. Hợp đồng: evictions giữ 0 — chỉ key MỚI vào cache
    // đầy mới đẩy entry.)
    let root = tmp_store_dir("memo-full-replace");
    let store = ContentStore::new(root.clone()).unwrap();
    let tiny = store.with_memo_capacity(std::num::NonZeroUsize::new(2).unwrap());

    // Fill BOTH slots (capacity 2) with distinct blobs.
    // (Điền CẢ HAI slot (capacity 2) bằng blob khác nhau.)
    let mut keys = Vec::new();
    for i in 0..2u32 {
        let data = format!("full-replace-blob-{i}").into_bytes();
        let hash = tiny.import_bytes(&data).unwrap();
        let dest = root.join(format!("f{i}.bin"));
        tiny.export_to(&hash, &dest).unwrap();
        keys.push(hash);
    }
    let before = tiny.memo_stats();
    assert_eq!(
        before.evictions, 0,
        "filling exactly to capacity evicts nothing: {before:?}"
    );

    // Replace ONE existing key on the FULL cache many times — each
    // export needs a FRESH destination (export_to refuses overwrite).
    // (Thay MỘT key đang có trên cache ĐẦY nhiều lần — mỗi export cần
    // đích MỚI (export_to từ chối ghi đè).)
    for round in 0..4u32 {
        let again = root.join(format!("full-again-{round}.bin"));
        tiny.export_to(&keys[0], &again).unwrap();
    }
    let after = tiny.memo_stats();
    assert_eq!(
        after.evictions, 0,
        "replacement on a FULL cache is NOT an eviction (P1-1): {after:?}"
    );
    assert!(after.hits >= 4, "re-verifies must hit: {after:?}");

    // The genuine eviction: a THIRD, FRESH key must displace one —
    // evictions must finally increment (exactly 1).
    // (Eviction thật: key THỨ BA MỚI phải đẩy bớt một entry — evictions
    // phải nhích (đúng 1).)
    let data = b"full-replace-blob-new".to_vec();
    let hash = tiny.import_bytes(&data).unwrap();
    let dest = root.join("f-new.bin");
    tiny.export_to(&hash, &dest).unwrap();
    let final_stats = tiny.memo_stats();
    assert_eq!(
        final_stats.evictions, 1,
        "a fresh key on a full cache is ONE genuine eviction: {final_stats:?}"
    );
}

#[cfg(unix)]
pub(crate) fn memo_hits_count_and_mutation_still_fails() {
    // Hit path accounting + eviction NEVER weakens security: a memoized
    // blob that is then rewritten on disk still fails (the LRU entry dies
    // with the file identity).
    // (Kế toán đường hit + eviction KHÔNG BAO GIỜ yếu hóa bảo mật: blob
    // đã memo mà bị ghi lại trên đĩa vẫn fail (entry LRU chết cùng file
    // identity).)
    let root = tmp_store_dir("memo-hit");
    let store = ContentStore::new(root.clone()).unwrap();
    let data = b"memo-hit-payload".to_vec();
    let hash = store.import_bytes(&data).unwrap();

    let o1 = root.join("a.bin");
    store.export_to(&hash, &o1).unwrap(); // miss + memoize
    let o2 = root.join("b.bin");
    store.export_to(&hash, &o2).unwrap(); // hit

    let stats = store.memo_stats();
    assert!(
        stats.hits >= 1,
        "second export must hit the memo: {stats:?}"
    );
    assert_eq!(stats.misses, 1, "first export is the only miss: {stats:?}");

    // Mutate → memo entry dies → fresh verify fails closed.
    // (Sửa → entry memo chết → verify mới fail cứng.)
    fs::write(hash.cas_path(&root), b"POISON-AFTER-LRU").unwrap();
    let o3 = root.join("c.bin");
    assert!(store.export_to(&hash, &o3).is_err());
}
