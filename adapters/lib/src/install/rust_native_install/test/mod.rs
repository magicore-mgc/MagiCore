//! Native install tests and shared fixture wiring.
//! Test install native và wiring fixture dùng chung.

#[path = "../../test/support.rs"]
mod support;

pub(super) fn resolved_rust(
    name: &str,
    version: &str,
    archive: &[u8],
) -> mgc_types::ResolvedPackage {
    support::resolved_rust(name, version, archive)
}

pub(super) fn test_crate_archive(name: &str, version: &str) -> Vec<u8> {
    support::test_crate_archive(name, version)
}
