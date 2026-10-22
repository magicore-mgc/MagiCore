//! Stable registry identity used in lock provenance.
//! Định danh registry ổn định dùng trong provenance lock.

/// Registry tag for lock provenance (`swift://{host}` from the configured
/// base; unconfigured → the honest `swift://unconfigured` marker).
/// Tag registry cho provenance lock (`swift://{host}` từ base đã cấu hình;
/// chưa cấu hình → marker trung thực `swift://unconfigured`).
pub(in crate::adapter::dependency_resolver) fn swift_registry_tag(
    protocol: &mgc_resolver::protocols::SwiftRegistryProtocol,
) -> String {
    let host = protocol
        .registry_host()
        .unwrap_or_else(|| "unconfigured".to_string());
    format!("swift://{host}")
}
