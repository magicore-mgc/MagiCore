//! Shared contract and safe default behavior for registry protocol engines.
//! Hợp đồng dùng chung và hành vi mặc định fail-closed cho engine registry.

use std::sync::Arc;

use async_trait::async_trait;
use futures_util::stream::{self, StreamExt, TryStreamExt};
use mgc_types::{MgError, MgResult, Version, VersionRange};
use tokio::sync::Semaphore;

use super::super::{
    digest::{sha256_hex, store_ref_for_blake3},
    graph_resolution::{
        GRAPH_RESOLVE_CONCURRENCY, GRAPH_ROOT_CONCURRENCY, resolve_graph_with_semaphore,
    },
};
use crate::protocols::{RegistryRoot, ResolvedEntry};

/// Contract for a native registry protocol engine (Phase 2).
/// Hợp đồng cho engine registry protocol native (Phase 2).
#[async_trait]
pub trait RegistryProtocol: Send + Sync {
    /// Resolve `name` within `range` against this protocol's registry.
    /// Resolve `name` trong khoảng `range` trên registry của protocol này.
    async fn resolve(&self, name: &str, range: &str) -> MgResult<ResolvedEntry>;

    /// Check a selected version against this protocol's requirement syntax.
    /// Root verification must use the same range semantics as resolution;
    /// registries such as Maven do not use semver ranges.
    /// Kiểm tra version được chọn theo cú pháp range của protocol này. Xác
    /// minh root phải cùng ngữ nghĩa với resolve; Maven không dùng semver.
    fn matches_requirement(&self, range: &str, version: &Version) -> bool {
        VersionRange::parse(range).is_ok_and(|requirement| requirement.matches(version))
    }

    /// Download the artifact bytes for a resolved entry.
    /// Tải byte artifact cho entry đã resolve.
    async fn download(&self, entry: &ResolvedEntry) -> MgResult<Vec<u8>>;

    /// Verify the downloaded artifact's sha256 against the declared digest.
    /// Fail-closed on mismatch AND on a missing digest: an artifact no
    /// registry hash vouches for is never installed (V1.2 zero-trust —
    /// engines with alternative integrity (sha1 markers, sumdb dirhash,
    /// git-commit pins) override this method; see go/maven/nuget/swift).
    /// Xác minh sha256 của artifact đã tải so với digest khai báo.
    /// Fail-closed khi lệch VÀ khi thiếu digest: artifact không có hash
    /// registry bảo lãnh không bao giờ được cài.
    fn verify(&self, entry: &ResolvedEntry, bytes: &[u8]) -> MgResult<()> {
        if entry.sha256.is_empty() {
            return Err(MgError::Integrity(format!(
                "refusing artifact without digest for {}@{} (registry provided no sha256 — re-resolve after the registry publishes checksums)",
                entry.name, entry.version
            )));
        }
        let actual = sha256_hex(bytes);
        if !actual.eq_ignore_ascii_case(&entry.sha256) {
            return Err(MgError::Integrity(format!(
                "sha256 mismatch for {}@{}: expected {}, got {}",
                entry.name, entry.version, entry.sha256, actual
            )));
        }
        Ok(())
    }

    /// Compute the mgc-store CAS reference (blake3) for artifact bytes.
    /// The ref matches `IntegrityHash::cas_path` under the store root, so a
    /// downstream `ContentStore::import_bytes` yields the same address.
    /// Tính CAS ref (blake3) của mgc-store cho byte artifact. Ref khớp
    /// `IntegrityHash::cas_path` dưới gốc store, nên `ContentStore::import_bytes`
    /// phía sau sinh cùng địa chỉ.
    fn store_ref(&self, bytes: &[u8]) -> String {
        store_ref_for_blake3(blake3::hash(bytes).to_hex().as_ref())
    }

    /// Full transitive resolution: resolve `name` and every reachable
    /// (already-filtered) dependency once, dedup by package name (the first
    /// chosen version wins, later edges reuse it), in BFS order. Any resolve
    /// error propagates (fail-closed — no silent drop of a subtree).
    /// Resolve bắc cầu đầy đủ: resolve `name` và mọi dependency khả chạm
    /// (đã lọc) đúng một lần, khử trùng theo tên (version chọn đầu thắng,
    /// cạnh sau tái sử dụng), theo thứ tự BFS. Mọi lỗi resolve lan lên
    /// (fail-closed — không âm thầm bỏ cây con).
    async fn resolve_graph(&self, name: &str, range: &str) -> MgResult<Vec<ResolvedEntry>> {
        resolve_graph_with_semaphore(
            self,
            name,
            range,
            Arc::new(Semaphore::new(GRAPH_RESOLVE_CONCURRENCY)),
        )
        .await
    }

    /// Resolve several project roots as one graph. Protocols with a native
    /// multi-constraint solver should override this; the default keeps the
    /// existing fail-closed behavior when independently resolved roots pick
    /// different versions of one transitive package.
    /// Resolve nhiều root của project thành một graph. Protocol có solver
    /// native đa-ràng-buộc nên override; mặc định giữ hành vi fail-closed
    /// khi các root resolve riêng chọn phiên bản transitive khác nhau.
    async fn resolve_graph_roots(
        &self,
        roots: &[(String, String)],
    ) -> MgResult<Vec<ResolvedEntry>> {
        let mut entries = Vec::new();
        let mut chosen = std::collections::HashMap::<String, ResolvedEntry>::new();
        let semaphore = Arc::new(Semaphore::new(GRAPH_RESOLVE_CONCURRENCY));
        let owned_roots = roots.to_vec();
        let mut root_graphs = stream::iter(owned_roots.into_iter().map(|(name, range)| {
            let semaphore = Arc::clone(&semaphore);
            async move { resolve_graph_with_semaphore(self, &name, &range, semaphore).await }
        }))
        .buffered(GRAPH_ROOT_CONCURRENCY);

        while let Some(root_entries) = root_graphs.try_next().await? {
            for entry in root_entries {
                if let Some(existing) = chosen.get(&entry.name) {
                    if existing != &entry {
                        return Err(MgError::DependencyConflict(format!(
                            "registry resolution for {} is inconsistent across root dependencies ({} vs {}); refusing to write an ambiguous lock graph",
                            entry.name, existing.version, entry.version
                        )));
                    }
                } else {
                    chosen.insert(entry.name.clone(), entry.clone());
                    entries.push(entry);
                }
            }
        }
        Ok(entries)
    }

    /// Resolve roots with registry-specific feature requests. Protocols that
    /// do not model features retain the existing root resolver behavior;
    /// feature-aware protocols override this method.
    /// Resolve root kèm feature riêng; protocol chưa hỗ trợ feature dùng lại
    /// resolver gốc, protocol hiểu feature sẽ override.
    async fn resolve_graph_roots_with_options(
        &self,
        roots: &[RegistryRoot],
    ) -> MgResult<Vec<ResolvedEntry>> {
        let roots = roots
            .iter()
            .map(|root| (root.name.clone(), root.range.clone()))
            .collect::<Vec<_>>();
        self.resolve_graph_roots(&roots).await
    }
}
