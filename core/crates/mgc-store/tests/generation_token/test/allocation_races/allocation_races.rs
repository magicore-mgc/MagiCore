//! Concurrent generation allocation and claim-batch behavior.
//! Cấp generation và nhận batch claim đồng thời.

use crate::generation_token::support::PROJ_A;
use mgc_store::Database;
use std::sync::{Arc, Barrier};

pub(crate) fn concurrent_claim_batches_preserve_every_package_file_claim() {
    const WORKERS: usize = 64;
    const CLAIMS_PER_WORKER: usize = 512;

    let dir = tempfile::tempdir().unwrap();
    let db_path = Arc::new(dir.path().join("store.db"));
    let generation = {
        let db = Database::open(&db_path).unwrap();
        db.begin_cas_generation(PROJ_A).unwrap()
    };
    let barrier = Arc::new(Barrier::new(WORKERS));
    let handles: Vec<_> = (0..WORKERS)
        .map(|worker| {
            let db_path = Arc::clone(&db_path);
            let barrier = Arc::clone(&barrier);
            std::thread::spawn(move || {
                let db = Database::open(&db_path).unwrap();
                let claims: Vec<String> = (0..CLAIMS_PER_WORKER)
                    .map(|index| format!("hash-w{worker}-{index}"))
                    .collect();
                let claim_refs: Vec<&str> = claims.iter().map(String::as_str).collect();
                barrier.wait();
                db.cas_claim_batch(PROJ_A, generation, &claim_refs)
                    .unwrap_or_else(|error| {
                        panic!("worker {worker} failed its claim batch: {error}")
                    });
                claims
            })
        })
        .collect();

    let expected: std::collections::HashSet<String> = handles
        .into_iter()
        .flat_map(|handle| handle.join().unwrap())
        .collect();
    let db = Database::open(&db_path).unwrap();
    let actual: std::collections::HashSet<String> =
        db.list_cas_live_refs().unwrap().into_iter().collect();
    assert_eq!(expected.len(), WORKERS * CLAIMS_PER_WORKER);
    assert_eq!(
        actual, expected,
        "concurrent claim batches must lose no CAS references"
    );
}

pub(crate) fn begin_cas_generation_survives_barrier_race() {
    // The round-10 "concurrency test" called begin twice SEQUENTIALLY on
    // two connections — no race at all. This test runs N threads behind
    // a barrier on the SAME db file (WAL + busy_timeout), each looping
    // begin/promote/abort, and demands: every begin returns a DISTINCT
    // token (allocation is atomic), no unique-violation surfaces, and
    // the final state is deterministic.
    // (Test "concurrency" vòng-10 gọi begin 2 lần TUẦN TỰ trên 2
    // connection — không có race. Test này chạy N thread sau barrier
    // trên CÙNG file db (WAL + busy_timeout), mỗi thread lặp
    // begin/promote/abort, và đòi: mọi begin trả token KHÁC nhau (cấp
    // phát nguyên tử), không lồ lộ unique-violation, state cuối tất định.)
    const THREADS: usize = 8;
    const ROUNDS: usize = 25;

    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("store.db");
    // Initialize the schema once (concurrent CREATE races are another
    // beast; open serializes schema setup here).
    // (Khởi tạo schema một lần (race CREATE song song là chuyện khác;
    // open xếp tuần tự phần setup schema ở đây).)
    {
        let db = Database::open(&db_path).unwrap();
        let t0 = db.begin_cas_generation(PROJ_A).unwrap();
        db.cas_claim(PROJ_A, t0, "hash-base").unwrap();
        db.promote_cas_generation(PROJ_A, t0).unwrap();
    }

    let barrier = Arc::new(Barrier::new(THREADS));
    let db_path = Arc::new(db_path);
    let mut handles = Vec::new();
    for _ in 0..THREADS {
        let barrier = Arc::clone(&barrier);
        let db_path = Arc::clone(&db_path);
        handles.push(std::thread::spawn(move || {
            // Open BEFORE the first barrier: DDL contention happens at
            // open, the race under test is begin/claim/promote only.
            // (Mở TRƯỚC barrier đầu: tranh chấp DDL xảy ra lúc open, race
            // được test chỉ là begin/claim/promote.)
            let db = Database::open(&db_path).unwrap();
            barrier.wait();
            let mut tokens = std::collections::HashSet::new();
            let mut errors: Vec<String> = Vec::new();
            for round in 0..ROUNDS {
                // Barrier per round: every thread hits begin at the same
                // logical moment — the true allocation race. A failing
                // step RECORDS the error and continues to the next
                // barrier round (a thread that returned early would
                // deadlock every other thread at the next barrier).
                // (Barrier mỗi vòng: mọi thread chạm begin cùng thời điểm
                // logic — race cấp phát thật. Bước fail GHI LỖI rồi tiếp
                // tục vòng barrier kế (thread return sớm làm mọi thread
                // khác chết đói ở barrier kế).)
                barrier.wait();
                match db.begin_cas_generation(PROJ_A) {
                    Ok(token) => {
                        if !tokens.insert(token) {
                            errors.push(format!(
                                "thread saw its own token {token} twice — \
                                 allocation not advancing"
                            ));
                            continue;
                        }
                        if let Err(e) =
                            db.cas_claim(PROJ_A, token, &format!("hash-t{token}-r{round}"))
                        {
                            errors.push(format!("claim r{round} t{token}: {e}"));
                            continue;
                        }
                        // Every install also REUSES a shared blob (the
                        // warm-cache pattern) — the reused hash must
                        // survive every promote in the race.
                        // (Mỗi install còn TÁI DỤNG một blob chung (pattern
                        // warm-cache) — hash tái dùng phải sống sót qua
                        // mọi promote trong race.)
                        if let Err(e) = db.cas_claim(PROJ_A, token, "hash-shared") {
                            errors.push(format!("shared claim r{round} t{token}: {e}"));
                            continue;
                        }
                        // Half the rounds promote, half abort — mixed
                        // lifecycle under contention.
                        // (Nửa vòng promote, nửa abort — vòng đời trộn
                        // lẫn dưới tranh chấp.)
                        let flip = if round % 2 == 0 {
                            db.promote_cas_generation(PROJ_A, token)
                        } else {
                            db.abort_cas_generation(PROJ_A, token)
                        };
                        if let Err(e) = flip {
                            errors.push(format!("lifecycle r{round} t{token}: {e}"));
                        }
                    }
                    Err(e) => errors.push(format!("begin r{round}: {e}")),
                }
            }
            (tokens, errors)
        }));
    }

    let mut all_tokens = std::collections::HashSet::new();
    let mut all_errors: Vec<String> = Vec::new();
    for handle in handles {
        let (tokens, errors) = handle.join().unwrap();
        all_errors.extend(errors);
        for token in tokens {
            assert!(
                all_tokens.insert(token),
                "P0-1: two threads were allocated the SAME token {token} — \
                 begin_cas_generation is not atomic"
            );
        }
    }
    assert!(
        all_errors.is_empty(),
        "race run reported errors: {all_errors:?}"
    );
    assert_eq!(all_tokens.len(), THREADS * ROUNDS);

    // Deterministic final state: the SHARED (reused) blob survives every
    // mix of promotes/aborts — the warm-cache invariant under contention.
    // Aborted tokens' exclusive claims are gone; promoted ones remain.
    // (State cuối tất định: blob CHUNG (tái dùng) sống sót qua mọi mix
    // promote/abort — invariant warm-cache dưới tranh chấp. Claim riêng
    // của token bị abort biến mất; của token promote còn lại.)
    let db = Database::open(&db_path).unwrap();
    let live = db.list_cas_live_refs().unwrap();
    assert!(
        live.contains(&"hash-shared".to_string()),
        "the REUSED shared blob must survive the race, live = {live:?}"
    );
}
