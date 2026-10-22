# Computation cache / Cache computation

Owns per-package source fingerprints and local build-freshness metadata. This is not the shared artifact CAS.

Quản lý fingerprint source và metadata freshness cục bộ theo package; đây không phải CAS artifact dùng chung. The cache record remains at `<package>/.magicore/.mgc_build_cache.json`.

`mod.rs` wires responsibility owners, `computation_cache.rs` coordinates the freshness check, and each child owns one hashing, path, record, traversal, or persistence responsibility. Security and memory limitations observed in S226 remain unchanged and are listed in the S226 tracking report.
