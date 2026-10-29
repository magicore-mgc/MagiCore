//! `ArtifactFetcher` for the Lib core.
//! `ArtifactFetcher` cho core Lib.
//!
//! TypeScript rides the embedded Web engine (npm tarballs). Every other
//! language routes through its own protocol in
//! `artifact_fetcher/native/<language>` — the same download → verify → CAS
//! sequence `mgc install` performs, exposed as a standalone capability.
//!
//! The previous revision failed closed for every native language with the
//! guidance "artifacts are fetched by the toolchain itself during its install".
//! That was untrue (MagiCore downloads `.crate`/wheel/go-mod files into its own
//! store) and it left `artifact-fetcher` claimed by exactly one language.
//! (TypeScript đi qua Web engine nhúng. Mọi ngôn ngữ khác đi qua protocol riêng
//! trong `artifact_fetcher/native/<language>` — đúng chuỗi download → verify →
//! CAS mà `mgc install` làm. Bản cũ fail-closed cho mọi ngôn ngữ native với lý
//! do "toolchain tự tải" — điều đó SAI và khiến chỉ một ngôn ngữ claim được
//! capability này.)

use crate::adapter::types::LibAdapter;
use crate::language::LibLanguage;
use async_trait::async_trait;
use mgc_types::capabilities::ArtifactFetcher;
use mgc_types::{MgResult, ResolvedGraph};

impl LibAdapter {
    /// The content store every native fetch imports into.
    ///
    /// Resolved through the install owner so a fetch writes bytes to exactly the
    /// store an install would read: a second store handle constructed here could
    /// point at a different root under a non-default `MGC_DATA_DIR`.
    /// (Store mà mọi fetch native import vào. Lấy qua owner install để fetch ghi
    /// đúng store mà install đọc — handle thứ hai dựng ở đây có thể trỏ sai root
    /// khi `MGC_DATA_DIR` không mặc định.)
    fn fetch_store(&self) -> MgResult<mgc_store::ContentStore> {
        mgc_store::ContentStore::new(mgc_store::default_store_root())
            .map_err(|error| mgc_types::MgError::Store(error.to_string()))
    }
}

#[async_trait]
impl ArtifactFetcher for LibAdapter {
    /// Claim is real only when the manifest claims it AND a protocol exists.
    ///
    /// The manifest already narrows Python projects owned by another manager and
    /// Java Gradle builds; probing `Ok` there would advertise a capability the
    /// manifest deliberately withholds, so the probe fails closed with the same
    /// reason the manifest omits it.
    /// (Claim chỉ thật khi manifest claim VÀ có protocol. Manifest đã thu hẹp
    /// Python do manager khác sở hữu và Java Gradle; probe `Ok` ở đó là quảng
    /// cáo capability manifest cố ý không cấp.)
    fn probe_artifact_fetcher(&self) -> MgResult<()> {
        if let Some(web) = &self.web {
            return web.probe_artifact_fetcher();
        }
        enabled_language(self.language)?;
        if crate::adapter::package_adapter::capability_profile::select_capabilities(self)
            .contains(&mgc_types::capabilities::Capability::ArtifactFetcher)
        {
            return Ok(());
        }
        Err(mgc_types::MgError::Unsupported {
            core: "lib",
            capability: "fetch",
            guidance: format!(
                "this project's {} lane is owned by another toolchain (foreign lockfile or \
                 Gradle build), so MagiCore does not fetch its artifacts; run the owning \
                 toolchain's install instead",
                self.language.ecosystem()
            ),
        })
    }

    async fn fetch(&self, graph: &ResolvedGraph) -> MgResult<()> {
        if let Some(web) = &self.web {
            return web.fetch(graph).await;
        }
        enabled_language(self.language)?;
        self.probe_artifact_fetcher()?;
        let store = self.fetch_store()?;
        super::native::fetch(self.language, graph, &store)
            .await
            .map(|_fetched| ())
    }
}

/// Reject languages whose standalone fetch has no protocol behind it.
///
/// A language without an entry here keeps the honest fail-closed error instead
/// of silently claiming a capability it cannot perform.
/// (Từ chối ngôn ngữ chưa có protocol. Ngôn ngữ không có ở đây giữ lỗi
/// fail-closed trung thực thay vì claim capability không làm được.)
fn enabled_language(language: LibLanguage) -> MgResult<()> {
    match language {
        LibLanguage::Rust
        | LibLanguage::Python
        | LibLanguage::Go
        | LibLanguage::Java
        | LibLanguage::DotNet
        | LibLanguage::Ts => Ok(()),
    }
}
