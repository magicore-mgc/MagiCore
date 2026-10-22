//! NuGet v3 native engine (.NET packages).
//! Engine native NuGet v3 (package .NET).
//!
//! V3 protocol: the service index (`{index}/index.json`) advertises the
//! `RegistrationsBaseUrl/3.6.0` (or Versioned) and `PackageBaseAddress/3.0.0`
//! resources. Versions come from the flat container
//! `{PackageBaseAddress}/{id-lower}/index.json`; the authoritative integrity
//! hash lives in the CATALOG leaf (`catalogEntry.@id` → catalog document's
//! `packageHash`, base64 SHA-512 of the nupkg bytes) — the registration
//! projection does NOT carry it (verified against live nuget.org: zero of
//! 84 newtonsoft.json leaves and zero serilog page leaves have the field).
//! Pages without inline leaves are followed via their `@id` URLs.
//! hash toàn vẹn chính thống nằm ở leaf CATALOG (`catalogEntry.@id` →
//! document catalog) — projection registration KHÔNG mang nó. Page không
//! có leaf inline được theo qua URL `@id`.
//! (base64 SHA-512 of the nupkg bytes — verified with the same algorithm the
//! server declares, fail-closed otherwise); `listed: false` entries are
//! never selected. Dependencies live in the nuspec INSIDE the nupkg, but the
//! flat container also serves `{PackageBaseAddress}/{id}/{version}/{id}.nuspec`
//! — fetched there so no zip handling is needed during resolve. Selection
//! mirrors NuGet's own minimum-version resolution: the LOWEST matching
//! listed version wins.
//! Protocol v3: service index (`{index}/index.json`) công bố resource
//! `RegistrationsBaseUrl/3.6.0` (hoặc Versioned) và `PackageBaseAddress/3.0.0`.
//! Version lấy từ flat container `{PackageBaseAddress}/{id-lower}/index.json`;
//! hash toàn vẹn chính thống nằm ở leaf CATALOG (`catalogEntry.@id` —
//! base64 SHA-512 của byte nupkg; projection registration không mang nó);
//! entry `listed: false` không bao giờ được chọn. Dep nằm trong nuspec BÊN TRONG nupkg, nhưng flat
//! container cũng phục vụ `{PackageBaseAddress}/{id}/{version}/{id}.nuspec`
//! — tải từ đó để resolve không cần xử lý zip. Selection phản chiếu
//! minimum-version resolution của chính NuGet: version thấp nhất khớp và
//! listed thắng.

use crate::protocols::nuget::find_resource;
use mgc_types::{MgError, MgResult};
use serde_json::Value;

const DEFAULT_INDEX_URL: &str = "https://api.nuget.org/v3/index.json";

/// Native NuGet v3 engine.
/// Engine native NuGet v3.
#[derive(Debug, Clone)]
pub struct NuGetProtocol {
    pub(in crate::protocols::nuget) registrations_base: String,
    pub(in crate::protocols::nuget) package_base: String,
    pub(in crate::protocols::nuget) client: mgc_http::HttpClient,
    /// Consumer target framework moniker (e.g. `net8.0` from the project's
    /// `<TargetFramework>`) — selects ONE group from divergent multi-TFM
    /// dependency sets. `None` (default) keeps the fail-closed no-merge
    /// behavior.
    /// (TFM của consumer — chọn MỘT group khi dep set phân kỳ.)
    pub(in crate::protocols::nuget) consumer_tfm: Option<String>,
}

impl NuGetProtocol {
    /// Build with explicit registration + flat-container bases (testability).
    /// Dựng với base registration + flat container tường minh (cho test).
    pub fn with_bases(registrations_base: &str, package_base: &str) -> Self {
        Self {
            registrations_base: registrations_base.trim_end_matches('/').to_string(),
            package_base: package_base.trim_end_matches('/').to_string(),
            client: mgc_http::HttpClient::default(),
            consumer_tfm: None,
        }
    }

    /// Pin the consumer target framework for multi-TFM group selection
    /// (builder — chain after `with_bases` / `from_env`).
    /// (Ghim TFM consumer để chọn group multi-TFM.)
    pub fn with_consumer_tfm(mut self, tfm: &str) -> Self {
        let tfm = tfm.trim();
        self.consumer_tfm = (!tfm.is_empty()).then(|| tfm.to_string());
        self
    }

    /// Build from environment: `MGC_NUGET_INDEX_URL` (service index whose
    /// resources are probed), defaulting to the nuget.org service index. A
    /// probe failure degrades to degenerate bases with a LOUD warning — every
    /// subsequent resolve fails closed instead of pretending to work.
    /// Dựng từ môi trường: `MGC_NUGET_INDEX_URL` (service index được dò
    /// resource), mặc định là service index của nuget.org. Dò lỗi thì hạ xuống
    /// base suy biến kèm cảnh báo ỒN ÀO — mọi resolve sau đó fail-closed thay
    /// vì giả vờ hoạt động.
    pub async fn from_env() -> Self {
        let index_url = std::env::var("MGC_NUGET_INDEX_URL")
            .ok()
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| DEFAULT_INDEX_URL.to_string());
        Self::from_service_index(&index_url)
            .await
            .unwrap_or_else(|e| {
                eprintln!(
                    "WARNING: NuGet service index {index_url} probe failed ({e}); \
                 no resource bases resolved — resolve calls will fail closed"
                );
                // Degenerate bases: both URLs collapse to the index URL so any
                // resolve attempt fails with a clear network error (fail-closed).
                // (Base suy biến: cả hai URL dồn về index URL nên mọi lần resolve
                // fail với lỗi mạng rõ ràng (fail-closed).)
                Self::with_bases(&index_url, &index_url)
            })
    }

    /// Probe a service index for the registration + flat-container bases
    /// (public: tests and adapters probe explicit indexes).
    /// Dò service index để lấy base registration + flat container (public:
    /// test và adapter dò index tường minh).
    pub async fn from_service_index(index_url: &str) -> MgResult<Self> {
        let client = mgc_http::HttpClient::default();
        let resp = client
            .get(index_url)
            .await
            .map_err(|e| MgError::Network(format!("GET {index_url} failed: {e}")))?;
        let status = resp.status();
        let body = resp
            .text()
            .await
            .map_err(|e| MgError::Network(format!("read {index_url} body failed: {e}")))?;
        if !status.is_success() {
            return Err(MgError::Network(format!(
                "GET {index_url} returned {status}"
            )));
        }
        let doc: Value = serde_json::from_str(&body)
            .map_err(|e| MgError::Other(format!("parse nuget service index failed: {e}")))?;
        let mut registrations = None;
        let mut package_base = None;
        // Preferred order: 3.6.0 → Versioned → plain RegistrationsBaseUrl.
        // (Thứ tự ưu tiên: 3.6.0 → Versioned → RegistrationsBaseUrl thường.)
        for ty in [
            "RegistrationsBaseUrl/3.6.0",
            "RegistrationsBaseUrl/Versioned",
            "RegistrationsBaseUrl",
        ] {
            if let Some(id) = find_resource(&doc, ty) {
                registrations = Some(id);
                break;
            }
        }
        if let Some(id) = find_resource(&doc, "PackageBaseAddress/3.0.0") {
            package_base = Some(id);
        }
        let registrations = registrations.ok_or_else(|| {
            MgError::Other("service index has no RegistrationsBaseUrl resource".to_string())
        })?;
        let package_base = package_base.ok_or_else(|| {
            MgError::Other("service index has no PackageBaseAddress resource".to_string())
        })?;
        Ok(Self::with_bases(&registrations, &package_base))
    }
}
