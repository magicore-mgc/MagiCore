//! v4 byte reproducibility: identical graphs must serialize to identical bytes.
//! Byte tái lập v4: cùng graph phải serialize ra byte y hệt.
//!
//! Reported defect: resolving the same graph repeatedly produced different
//! `mgc.lock` hashes, the only difference being the order of set-like
//! collections inside each `[[package]]` item. These tests pin the fix:
//! shuffled inputs converge on one byte string, `lockfile_hash` is filled
//! and stable, and any material change moves the hash.
//! (Lỗi đã báo: resolve cùng graph nhiều lần cho hash `mgc.lock` khác nhau,
//! chỉ khác thứ tự collection dạng tập trong mỗi `[[package]]`. Test dưới
//! đây khóa bản sửa: input xáo trộn hội tụ về một chuỗi byte duy nhất,
//! `lockfile_hash` được điền và ổn định, mọi thay đổi thực chất đổi hash.)

use mgc_lockfile::EcosystemTag;
use mgc_lockfile::canonical::{LockfileV4, payload_digest};
use mgc_lockfile::v4_graph::build_v4_from_resolved_graph;
use mgc_lockfile::writer::finalize_v4_document;
use mgc_types::{PackageId, ResolvedGraph, ResolvedPackage};

const GENERATED_AT: &str = "2026-10-31T00:00:00Z";
const SRI: &str = "sha512-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==";

fn pkg(
    name: &str,
    version: &str,
    deps: Vec<PackageId>,
    peers: Vec<PackageId>,
    direct: bool,
) -> ResolvedPackage {
    ResolvedPackage {
        id: PackageId::parse(&format!("{name}@{version}")).expect("test fixture id must parse"),
        integrity: SRI.to_string(),
        tarball_url: format!("https://registry.npmjs.org/{name}/-/{name}-{version}.tgz"),
        deps,
        peer_deps: peers,
        direct,
        dev: false,
    }
}

/// A graph whose packages carry several set-like collections: dependency
/// edges, peer edges and a non-empty peer context.
/// (Graph có nhiều collection dạng tập: cạnh dependency, cạnh peer và một
/// peer context khác rỗng.)
fn dependency_graph() -> ResolvedGraph {
    let names = [
        "typescript",
        "react-dom",
        "react",
        "scheduler",
        "loose-envify",
    ];
    let ids: Vec<PackageId> = names
        .iter()
        .map(|name| PackageId::parse(&format!("{name}@1.0.0")).expect("test fixture id must parse"))
        .collect();
    let mut packages = vec![pkg("app", "1.0.0", ids.clone(), Vec::new(), true)];
    for (index, id) in ids.iter().enumerate() {
        let peers = if index % 2 == 0 {
            vec![ids[(index + 1) % ids.len()].clone()]
        } else {
            Vec::new()
        };
        packages.push(ResolvedPackage {
            id: id.clone(),
            integrity: SRI.to_string(),
            tarball_url: format!(
                "https://registry.npmjs.org/{}/-/{}-1.0.0.tgz",
                id.name_str(),
                id.name_str()
            ),
            deps: Vec::new(),
            peer_deps: peers,
            direct: false,
            dev: false,
        });
    }
    ResolvedGraph { packages }
}

fn finalize(graph: &ResolvedGraph) -> (LockfileV4, String) {
    let mut document = build_v4_from_resolved_graph(
        graph,
        "https://registry.npmjs.org",
        EcosystemTag::Web,
        "web",
        "mgc/test",
    )
    .expect("test fixture graph must build");
    document.metadata.generated_at = GENERATED_AT.to_string();
    let text = finalize_v4_document(&mut document).expect("finalize must succeed");
    (document, text)
}

fn shuffled(graph: &ResolvedGraph) -> ResolvedGraph {
    let mut shuffled = graph.clone();
    shuffled.packages.reverse();
    for package in &mut shuffled.packages {
        package.deps.reverse();
        package.peer_deps.reverse();
    }
    shuffled
}

/// Same graph resolved twice with different input order → identical bytes.
/// (Cùng graph resolve hai lần với thứ tự input khác nhau → byte y hệt.)
#[test]
fn shuffled_resolution_produces_identical_bytes() {
    let graph = dependency_graph();
    let (_, first) = finalize(&graph);
    let (_, second) = finalize(&shuffled(&graph));
    assert_eq!(
        first, second,
        "the same graph must serialize to byte-identical lockfiles regardless of input order"
    );
}

/// `lockfile_hash` is populated and recomputable from the written file.
/// (`lockfile_hash` được điền và tính lại được từ chính file đã ghi.)
#[test]
fn lockfile_hash_is_present_and_matches_the_written_payload() {
    let (document, text) = finalize(&dependency_graph());
    assert!(
        !document.metadata.lockfile_hash.is_empty(),
        "finalize must populate lockfile_hash"
    );
    assert!(
        document.metadata.lockfile_hash.starts_with("blake3-"),
        "unexpected digest shape: {}",
        document.metadata.lockfile_hash
    );
    let parsed =
        mgc_lockfile::canonical::parse_v4_document(&text).expect("written file must parse");
    assert_eq!(
        parsed.metadata.lockfile_hash,
        document.metadata.lockfile_hash
    );
    assert_eq!(
        parsed.metadata.lockfile_hash,
        payload_digest(&parsed.payload()),
        "a reader must be able to recompute the recorded digest"
    );
    let report = mgc_lockfile::policy::verify_v4_math(&parsed).expect("verify must succeed");
    assert!(report.digest_ok);
}

/// Re-finalizing the same graph keeps the hash stable across runs.
/// (Finalize lại cùng graph giữ hash ổn định giữa các lần chạy.)
#[test]
fn lockfile_hash_is_stable_across_repeated_runs() {
    let graph = dependency_graph();
    let mut hashes = Vec::new();
    for _ in 0..4 {
        let (document, _) = finalize(&graph);
        hashes.push(document.metadata.lockfile_hash);
    }
    assert!(
        hashes.windows(2).all(|pair| pair[0] == pair[1]),
        "lockfile_hash must not drift between identical runs: {hashes:?}"
    );
}

/// Changing one package version changes the hash; changing only input order
/// does not. (Đổi version một package thì hash đổi; chỉ đổi thứ tự input
/// thì không.)
#[test]
fn changing_a_package_changes_the_hash() {
    let graph = dependency_graph();
    let (baseline, _) = finalize(&graph);

    let mut changed = graph.clone();
    changed
        .packages
        .iter_mut()
        .find(|package| package.id.name_str() == "react")
        .expect("fixture must contain react")
        .id = PackageId::parse("react@2.0.0").expect("test fixture id must parse");
    let (changed, _) = finalize(&changed);

    assert_ne!(
        baseline.metadata.lockfile_hash, changed.metadata.lockfile_hash,
        "a different package version must change lockfile_hash"
    );
    assert_eq!(
        baseline.metadata.lockfile_hash,
        finalize(&shuffled(&graph)).0.metadata.lockfile_hash,
        "input order alone must not change lockfile_hash"
    );
}
