#![allow(clippy::unwrap_used)]
//! Nearest-owner compile-options regression tests.
//! Test regression gần owner compile-options.

use super::super::{COMPILER_IDENTITY, COMPILER_VERSION};
use super::support::canonical_compile_options_ok;
use tempfile::tempdir;

/// Cross-project poison gate (Hướng B, P0-2 Tech Lead vòng-6/7): two
/// projects with IDENTICAL source bytes but DIFFERENT tsconfig semantics
/// must derive DIFFERENT compilation keys — the shared global compiled
/// cache can never serve one project's output to the other. The test runs
/// at the KEY level (canonical options JSON + CompilationKey), the exact
/// boundary where a cross-hit would occur.
///
/// Cổng chống độc chéo project (Hướng B, P0-2): hai project có bytes
/// source GIỐNG HỆT nhưng semantics tsconfig KHÁC phải ra compilation key
/// KHÁC — compiled cache toàn cục dùng chung không bao giờ được phục vụ
/// output của project này cho project kia. Test chạy ở tầng KEY (JSON
/// options chuẩn hóa + CompilationKey) — đúng biên nơi cross-hit xảy ra.
#[test]
fn different_tsconfig_projects_never_share_a_compilation_key() {
    let tmp = tempdir().unwrap();
    let project_a = tmp.path().join("a");
    let project_b = tmp.path().join("b");
    let source = "src/index.tsx";
    for project in [&project_a, &project_b] {
        std::fs::create_dir_all(project.join("src")).unwrap();
        std::fs::write(
            project.join(source),
            "export const App = () => <div>ok</div>;",
        )
        .unwrap();
    }
    // Same source bytes, DIFFERENT tsconfig semantics (jsx runtime + target).
    // (Bytes source giống, semantics tsconfig KHÁC (jsx runtime + target).)
    std::fs::write(
        project_a.join("tsconfig.json"),
        r#"{"compilerOptions":{"jsx":"react-jsx","target":"ES2020"}}"#,
    )
    .unwrap();
    std::fs::write(
        project_b.join("tsconfig.json"),
        r#"{"compilerOptions":{"jsx":"react","jsxFactory":"h","target":"ES5"}}"#,
    )
    .unwrap();

    let loader = mgc_store::cas::Loader::Tsx;
    let source_digest = blake3::Hasher::new()
        .update(b"export const App = () => <div>ok</div>;")
        .finalize()
        .to_hex()
        .to_string();

    let key_a = mgc_store::cas::CompilationKey::new(
        &source_digest,
        loader,
        COMPILER_IDENTITY,
        COMPILER_VERSION,
        &canonical_compile_options_ok(&project_a, &project_a.join(source)),
    )
    .unwrap();
    let key_b = mgc_store::cas::CompilationKey::new(
        &source_digest,
        loader,
        COMPILER_IDENTITY,
        COMPILER_VERSION,
        &canonical_compile_options_ok(&project_b, &project_b.join(source)),
    )
    .unwrap();
    assert_ne!(
        key_a, key_b,
        "same bytes + different tsconfig must NOT share a cache entry (P0-2 cross-project poison)"
    );

    // The tsconfig digest must invalidate on EDIT: same project, change
    // the tsconfig content → new key (no stale serve after a config edit).
    // (Digest tsconfig phải vô hiệu khi SỬA: cùng project, đổi nội dung
    // tsconfig → key mới (không serve stale sau khi sửa cấu hình).)
    std::fs::write(
        project_a.join("tsconfig.json"),
        r#"{"compilerOptions":{"jsx":"react-jsx","target":"ES2022"}}"#,
    )
    .unwrap();
    let key_a2 = mgc_store::cas::CompilationKey::new(
        &source_digest,
        loader,
        COMPILER_IDENTITY,
        COMPILER_VERSION,
        &canonical_compile_options_ok(&project_a, &project_a.join(source)),
    )
    .unwrap();
    assert_ne!(
        key_a, key_a2,
        "editing tsconfig.json must change the key (P0-2 stale-serve gate)"
    );
    // Restore the ORIGINAL content → the digest matches the original key
    // again (content-based digest — no spurious invalidation loop).
    // (Khôi phục nội dung GỐC → digest khớp key gốc (digest theo nội dung
    // — không lặp vô hiệu hóa giả).)
    std::fs::write(
        project_a.join("tsconfig.json"),
        r#"{"compilerOptions":{"jsx":"react-jsx","target":"ES2020"}}"#,
    )
    .unwrap();
    let key_a3 = mgc_store::cas::CompilationKey::new(
        &source_digest,
        loader,
        COMPILER_IDENTITY,
        COMPILER_VERSION,
        &canonical_compile_options_ok(&project_a, &project_a.join(source)),
    )
    .unwrap();
    assert_eq!(
        key_a, key_a3,
        "restoring tsconfig.json content must restore the original key (digest is content-based)"
    );
}
