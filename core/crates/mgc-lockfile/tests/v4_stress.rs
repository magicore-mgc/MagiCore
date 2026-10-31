//! v4 stress test — stress test v4 cho tới khi ra V4 thì thôi.
//!
//! Deterministic pseudo-random install cycles (no new dependencies — a tiny
//! inline LCG): build → finalize (digest + round-trip proof) → parse →
//! digest-verify → replay to graph, 200 rounds across peer/multi-source/
//! multi-registry shapes. Any divergence fails loudly.
//! (Chu kỳ install giả ngẫu nhiên deterministic (không dep mới — LCG nhỏ
//! inline): build → finalize (digest + chứng minh round-trip) → parse →
//! verify digest → replay ra graph, 200 vòng qua shape peer/nhiều nguồn/
//! nhiều registry. Lệch đâu fail rõ đó.)

use mgc_lockfile::EcosystemTag;
use mgc_lockfile::canonical::{LockfileV4, payload_digest};
use mgc_lockfile::v4_graph::{build_v4_from_resolved_graph, graph_from_v4_lockfile};
use mgc_types::{PackageId, ResolvedGraph, ResolvedPackage};
use std::collections::HashSet;

/// Tiny deterministic LCG (no new dependency for test randomness).
/// (LCG nhỏ deterministic (không dep mới cho random test).)
struct Lcg(u64);

impl Lcg {
    fn next(&mut self, bound: usize) -> usize {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 33) as usize) % bound.max(1)
    }
}

fn node_name(rng: &mut Lcg, lane: usize) -> String {
    const WORDS: &[&str] = &[
        "alpha", "beta", "gamma", "delta", "epsilon", "zeta", "eta", "theta",
    ];
    format!(
        "{}-{}-pkg{}",
        WORDS[rng.next(WORDS.len())],
        lane,
        rng.next(4)
    )
}

fn version_of(rng: &mut Lcg) -> String {
    format!("{}.{}.{}", rng.next(5), rng.next(10), rng.next(10))
}

fn sri_of(rng: &mut Lcg) -> String {
    // Installer-enforced shape most rounds; occasionally empty (SRI-less
    // pins stay hash-less, never fabricated).
    // (Dạng SRI thực thi được hầu hết vòng; thỉnh thoảng rỗng (pin thiếu
    // SRI không hash, không bịa).)
    if rng.next(5) == 0 {
        return String::new();
    }
    let mut payload = String::with_capacity(86);
    for _ in 0..86 {
        payload.push(
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/="[rng.next(64)]
                as char,
        );
    }
    format!("sha512-{payload}")
}

fn random_graph(rng: &mut Lcg, lane: usize, size: usize) -> ResolvedGraph {
    let mut ids = Vec::new();
    for _ in 0..size {
        let id = PackageId::parse(&format!("{}@{}", node_name(rng, lane), version_of(rng)))
            .expect("test operation must succeed");
        if !ids.contains(&id) {
            ids.push(id);
        }
    }
    let mut packages = Vec::new();
    for (index, id) in ids.iter().enumerate() {
        let mut deps = Vec::new();
        let mut peers = Vec::new();
        // Edges only point backwards: the graph stays acyclic by construction.
        // (Cạnh chỉ trỏ ngược: graph không chu trình theo cấu trúc.)
        for earlier in ids.iter().take(index) {
            let roll = rng.next(10);
            if roll < 2 {
                deps.push(earlier.clone());
            } else if roll == 2 {
                peers.push(earlier.clone());
            }
        }
        let version = id.version().to_string();
        packages.push(ResolvedPackage {
            id: id.clone(),
            integrity: sri_of(rng),
            tarball_url: format!(
                "https://registry-{}.example/{}/-/{}-{}.tgz",
                lane,
                id.name_str(),
                id.name_str(),
                version
            ),
            deps,
            peer_deps: peers,
            direct: false,
            dev: false,
        });
    }
    // Every acyclic component needs a manifest root, otherwise it is an orphan fixture.
    // Mỗi thành phần DAG cần root manifest; fixture không được giả package mồ côi là graph hợp lệ.
    let referenced: HashSet<PackageId> = packages
        .iter()
        .flat_map(|package| package.deps.iter().chain(&package.peer_deps))
        .cloned()
        .collect();
    for package in &mut packages {
        package.direct = !referenced.contains(&package.id);
    }
    ResolvedGraph { packages }
}

/// Compare content, ignoring collection order the writer sorts.
///
/// The writer emits packages/edges in a canonical order and the reader
/// re-sorts package items, so an ordered comparison rejects a faithful
/// round trip. Every field is still compared after normalization.
/// (So nội dung, bỏ qua thứ tự collection mà writer sắp xếp. Writer xuất
/// thứ tự canonical và reader sắp lại item package nên so theo thứ tự sẽ
/// từ chối round-trip trung thực. Mọi field vẫn được so sau chuẩn hóa.)
fn assert_same_document(back: &LockfileV4, doc: &LockfileV4, round: usize) {
    let mut left = back.clone();
    let mut right = doc.clone();
    for item in [&mut left, &mut right] {
        item.packages.sort_by(|a, b| {
            (
                a.key.name.clone(),
                a.key.version.clone(),
                a.key.source_id.clone(),
                format!("{:?}", a.key.variant),
            )
                .cmp(&(
                    b.key.name.clone(),
                    b.key.version.clone(),
                    b.key.source_id.clone(),
                    format!("{:?}", b.key.variant),
                ))
        });
        for package in &mut item.packages {
            package
                .edges
                .sort_by(|a, b| format!("{a:?}").cmp(&format!("{b:?}")));
        }
    }
    assert_eq!(left, right, "round {round}: round-trip drift");
}

#[test]
fn v4_stress_200_rounds() {
    let mut rng = Lcg(0xC0FFEE);
    for round in 0..200 {
        let lane = round % 3;
        let size = 1 + rng.next(8);
        // Alternate registries so multi-source identity is stressed.
        // (Đổi registry để stress identity nhiều nguồn.)
        let registry = format!("https://registry-{lane}.example");
        let graph = random_graph(&mut rng, lane, size);
        let mut doc: LockfileV4 =
            build_v4_from_resolved_graph(&graph, &registry, EcosystemTag::Web, "web", "mgc/stress")
                .unwrap_or_else(|e| panic!("round {round}: build failed: {e}"));
        doc.metadata.generated_at = "2026-10-07T00:00:00Z".to_string();
        doc.metadata.lockfile_hash = payload_digest(&doc.payload());
        let text = mgc_lockfile::canonical::write_v4_document(&doc)
            .unwrap_or_else(|e| panic!("round {round}: serialize failed: {e}"));
        let back = mgc_lockfile::canonical::parse_v4_document(&text)
            .unwrap_or_else(|e| panic!("round {round}: parse failed: {e}"));
        assert_same_document(&back, &doc, round);
        let report = mgc_lockfile::policy::verify_v4_math(&back)
            .unwrap_or_else(|e| panic!("round {round}: verify failed: {e}"));
        assert!(report.digest_ok, "round {round}: digest mismatch");
        // Replay must recover every locked instance (peer instances kept
        // distinct — no name@version collapse). SRI-less pins are the one
        // honest exception: the builder preserves them hash-less (never
        // fabricated) and replay refuses exactly those pins fail-closed.
        // (Replay phải khôi phục mọi instance. Pin thiếu SRI là ngoại lệ
        // trung thực duy nhất: builder giữ chỗ không hash (không bịa) và
        // replay từ chối đúng pin đó fail-closed.)
        let has_srless = doc.packages.iter().any(|p| {
            p.artifact
                .as_ref()
                .is_none_or(|a| a.integrity_sri.is_none())
        });
        match graph_from_v4_lockfile(&back, "web") {
            Ok(replayed) => {
                assert!(!has_srless, "round {round}: SRI-less graph replayed?!");
                assert_eq!(
                    replayed.packages.len(),
                    doc.packages.len(),
                    "round {round}: replay lost instances"
                );
            }
            Err(e) => {
                assert!(
                    has_srless && e.to_string().contains("no installer-enforced SRI"),
                    "round {round}: unexpected replay refusal: {e}"
                );
            }
        }
    }
}
