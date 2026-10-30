//! Derive the real registry source of a lane instead of a fabricated one.
//! Suy ra registry source thật của lane thay vì bịa ra một nguồn giả.
//!
//! SEC-LOOP-04: the builder used to stamp every pin with `src-native`,
//! `claims: ["*"]`, `trusted: false` regardless of the registry that actually
//! served the bytes. A consumer verifying the signed lock then attested
//! "all packages came from one wildcard source", which is wrong whenever the
//! configured registry is not the built-in npm chain. The id is derived from
//! the registry URL (the same `src-<blake3>` scheme the v3→v4 importer uses),
//! and the claim is the URL's own host — not `*`.
//! (SEC-LOOP-04: builder từng đóng dấu mọi pin là `src-native`, `claims: ["*"]`,
//! `trusted: false` bất kể registry thật phục vụ byte. Bên verify lock đã ký
//! khi đó cam kết "tất cả từ một nguồn wildcard" — sai khi registry cấu hình
//! không phải chain npm mặc định. Id nay suy từ URL registry (đúng scheme
//! `src-<blake3>` của importer v3→v4) và claim là host của chính URL.)

use crate::ecosystem_tag::EcosystemTag;
use crate::v4::SourceRef;

/// Stand-in host used when a registry URL has no parseable authority.
/// Host thay thế khi URL registry không có authority parse được.
const UNPARSEABLE_HOST_FALLBACK: &str = "unparseable-registry";

/// Priority of a lane's single source; equal to the importer's real-source rank.
/// (Priority của nguồn duy nhất của lane; bằng hạng nguồn thật của importer.)
pub(crate) const LANE_SOURCE_PRIORITY: u64 = 100;

/// Build the one source record that really served this lane's packages.
/// (Dựng record nguồn duy nhất thật sự phục vụ package của lane này.)
pub fn lane_source(registry_url: &str, ecosystem: EcosystemTag) -> SourceRef {
    let url = normalize_registry_url(registry_url);
    let authority = parse_authority(&url);
    SourceRef {
        id: source_id_for(&url),
        claims: vec![
            authority
                .as_ref()
                .map_or_else(|| UNPARSEABLE_HOST_FALLBACK.to_string(), |a| a.host.clone()),
        ],
        url,
        ecosystem,
        priority: LANE_SOURCE_PRIORITY,
        // Trust is a project decision (`mgc.toml [[sources]] trusted`), never
        // invented by the writer: a lock that claims `trusted = true` for a
        // public registry would let a consumer's source-selection accept it
        // over a real private claim.
        // (Trust là quyết định của project, không do writer bịa ra.)
        trusted: false,
        allow_hosts: Vec::new(),
        allow_cidrs: Vec::new(),
        allow_protocols: vec![authority.map(|a| a.scheme).unwrap_or("https").to_string()],
    }
}

/// Stable `src-<12 hex>` identity, identical to the v3→v4 importer scheme.
/// (Identity `src-<12 hex>` ổn định, trùng scheme của importer v3→v4.)
pub fn source_id_for(registry_url: &str) -> String {
    use mgc_crypto::blake3_signer::Blake3Hasher;
    let digest = Blake3Hasher::hash_string(&normalize_registry_url(registry_url)).to_hex();
    format!("src-{}", &digest[..12.min(digest.len())])
}

struct Authority {
    scheme: &'static str,
    host: String,
}

/// Split `scheme://host[:port][/path]` without a URL crate (this crate has
/// no `url` dependency and must not grow one for one field).
/// (Tách `scheme://host[:port][/path]` không cần crate URL.)
fn parse_authority(url: &str) -> Option<Authority> {
    let (scheme, rest) = url.split_once("://")?;
    let scheme = match scheme.to_ascii_lowercase().as_str() {
        "http" => "http",
        "https" => "https",
        _ => return None,
    };
    let authority = rest.split(['/', '?', '#']).next()?;
    // Strip userinfo: credentials must never become a lockfile claim.
    // (Bỏ userinfo: credential không bao giờ được thành claim trong lock.)
    let host_port = authority.rsplit('@').next()?;
    let host = host_port.split(':').next()?.trim();
    if host.is_empty() {
        return None;
    }
    Some(Authority {
        scheme,
        host: host.to_ascii_lowercase(),
    })
}

/// Drop a trailing slash so `https://r/` and `https://r` share one identity.
/// (Bỏ slash cuối để hai dạng URL chia sẻ một identity.)
pub(crate) fn normalize_registry_url(registry_url: &str) -> String {
    registry_url.trim().trim_end_matches('/').to_string()
}
