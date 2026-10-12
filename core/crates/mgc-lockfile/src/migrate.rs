//! Lockfile version migrations (v1 → v2, v2 → v3)
//! Migration phiên bản lockfile (v1 → v2, v2 → v3)

use crate::schema::{LOCKFILE_SCHEMA_VERSION, Provenance, SOURCE_KIND_REGISTRY_IMPORT};
use crate::{Lockfile, LockfileError, LockfileResult, Package};
use serde::{Deserialize, Serialize};

/// Lockfile v1 structure (legacy) — Cấu trúc lockfile v1 (legacy)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockfileV1 {
    pub version: String,
    #[serde(rename = "package")]
    pub packages: Vec<PackageV1>,
}

/// Package v1 structure (no integrity field) — Cấu trúc package v1 (không có integrity)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageV1 {
    pub name: String,
    pub version: String,
    pub resolved: String,
    #[serde(default)]
    pub dependencies: Vec<String>,
}

/// Detect lockfile version from TOML string — Phát hiện version lockfile từ chuỗi TOML
pub fn detect_lockfile_version(toml_str: &str) -> LockfileResult<u8> {
    // Try to parse version field
    let parsed: toml::Value = toml::from_str(toml_str)?;

    let version = parsed
        .get("version")
        .and_then(|v| v.as_str())
        .ok_or_else(|| LockfileError::ParseError("missing version field".to_string()))?;

    match version {
        "1" => Ok(1),
        "2" => Ok(2),
        "3" => Ok(3),
        version if crate::v4::is_v4_schema_version(version) => Ok(4),
        _ => Err(LockfileError::ParseError(format!(
            "unknown version: {}",
            version
        ))),
    }
}

/// Migrate lockfile v1 to v2 — Migrate lockfile v1 sang v2
///
/// Emits the v2 shape on purpose (version pinned to "2") so the step stays
/// composable; `auto_upgrade_lockfile` chains it into `migrate_v2_to_v3`.
/// Cố ý xuất shape v2 (version ghim "2") để bước này ghép được;
/// `auto_upgrade_lockfile` sẽ nối tiếp bằng `migrate_v2_to_v3`.
pub fn migrate_v1_to_v2(lockfile_v1: LockfileV1) -> LockfileResult<Lockfile> {
    let mut lockfile_v2 = Lockfile::new();
    // Pin the intermediate schema version — the name of this function is a
    // contract (v1 → v2), the final bump happens in migrate_v2_to_v3.
    // Ghim version schema trung gian — tên hàm là hợp đồng (v1 → v2),
    // bước nâng cuối nằm ở migrate_v2_to_v3.
    lockfile_v2.version = "2".to_string();

    // Migrate packages
    for pkg_v1 in lockfile_v1.packages {
        let pkg_v2 = Package {
            name: pkg_v1.name.clone(),
            version: pkg_v1.version.clone(),
            resolved: pkg_v1.resolved.clone(),
            // L7 FIX: Use BLAKE3 for placeholder (still placeholder but cryptographic)
            // Production: download tarball and hash, or warn user to re-install
            integrity: format!("blake3-{}", blake3_placeholder_hash(&pkg_v1.resolved)),
            dependencies: pkg_v1.dependencies.clone(),
            // v3 fields keep their defaults (ecosystem=other, provenance=None);
            // migrate_v2_to_v3 fills provenance right after.
            // Field v3 giữ mặc định (ecosystem=other, provenance=None);
            // migrate_v2_to_v3 sẽ điền provenance ngay sau đó.
            ..Default::default()
        };

        lockfile_v2.add_package(pkg_v2);
    }

    Ok(lockfile_v2)
}

/// Migrate lockfile v2 to v3 — Migrate lockfile v2 sang v3
///
/// v2 files carry no ecosystem data, so after migration every package has
/// `ecosystem = other` (the serde default applied during the v2 parse) and
/// `provenance = registry-import` where provenance was empty. Ecosystem is
/// NOT force-reset: running the migration on a v3 lock must not clobber
/// existing ecosystem data.
/// File v2 không mang dữ liệu ecosystem, nên sau migration mọi package có
/// `ecosystem = other` (mặc định serde khi parse v2) và
/// `provenance = registry-import` ở chỗ provenance còn trống. KHÔNG ép
/// reset ecosystem: chạy migration trên lock v3 không được xoá dữ liệu
/// ecosystem sẵn có.
pub fn migrate_v2_to_v3(mut lockfile: Lockfile) -> Lockfile {
    lockfile.version = LOCKFILE_SCHEMA_VERSION.to_string();

    for pkg in &mut lockfile.packages {
        if pkg.provenance.is_none() {
            pkg.provenance = Some(Provenance {
                source_kind: SOURCE_KIND_REGISTRY_IMPORT.to_string(),
                tool: None,
                imported_from: None,
            });
        }
    }

    lockfile
}

// Convert the native V3 SHA-256 hex representation without calculating a new hash.
// Chỉ đổi mã hóa của cùng 32 byte digest; không tạo bằng chứng artifact mới.
fn native_sha256_sri(integrity: &str) -> Option<String> {
    use base64::Engine;
    let raw = integrity.strip_prefix("sha256-")?;
    if raw.len() != 64 {
        return None;
    }
    let digest = hex::decode(raw).ok()?;
    Some(format!(
        "sha256-{}",
        base64::engine::general_purpose::STANDARD.encode(digest)
    ))
}

/// Preserve supported SRI or normalize the native V3 SHA-256 hex encoding.
/// Encoding conversion retains the existing digest; it never calculates a hash.
/// Unsupported digests are refused with warnings. A missing artifact uses the
/// pin's own URL/registry; pins without an artifact URL remain unavailable.
/// Giữ SRI hợp lệ hoặc đổi mã hóa digest SHA-256 hex V3 mà không tạo hash mới.
/// Marker không hỗ trợ vẫn cảnh báo; artifact chỉ dùng URL/registry sẵn có của pin.
fn carry_install_sri(
    package: &Package,
    warnings: &mut Vec<String>,
) -> Option<crate::schema::ArtifactRef> {
    let mut artifact = package
        .artifact
        .clone()
        .unwrap_or_else(|| crate::schema::ArtifactRef {
            url: package.resolved.clone(),
            size_bytes: None,
            content_hash: String::new(),
            downloaded_from: package.registry.clone().unwrap_or_default(),
            integrity_sri: None,
        });
    if artifact.url.is_empty() {
        warnings.push(format!(
            "pin {}@{} has no artifact URL: v4 install could never fetch it",
            package.name, package.version
        ));
        return None;
    }
    if crate::policy::check_install_sri(&package.integrity).is_ok() {
        artifact.integrity_sri = Some(package.integrity.clone());
    } else if let Some(sri) = native_sha256_sri(&package.integrity) {
        artifact.integrity_sri = Some(sri);
    } else if !package.integrity.is_empty() {
        warnings.push(format!(
            "pin {}@{} has SRI '{}' the installer cannot enforce: dropped from v4 artifact (re-resolve to refresh)",
            package.name, package.version, package.integrity
        ));
    }
    Some(artifact)
}

/// Migrate lockfile v3 to v4 — Migrate lockfile v3 sang v4.
///
/// Returns the v4 document PLUS every lossy decision as warnings (never
/// silent):
/// - `(name, version)` identity splits by registry: same name+version
///   from two registries becomes two PackageKeys (the v3 merge lie,
///   undone).
/// - v3 `dependencies: [names]` carry no range/version: an edge is kept
///   only when exactly one locked package bears the name (range `"*"`,
///   transitive origin); zero or several matches → edge dropped +
///   warning (no fabricated target).
/// - v3 `peers: [names]` carry no versions: peer digests cannot be
///   computed → peers are dropped + warning (a digest from thin air
///   would be fake data).
/// - packages without a registry land on the explicit `"unknown"`
///   source (labeled, never a guessed URL).
/// - v3 `integrity` (installer SRI) is carried into
///   `artifact.integrity_sri` only when it parses as installer-enforced
///   SRI; unsupported values are dropped with a warning, empty stays
///   empty (v4 install refuses SRI-less pins — the failure belongs to
///   the artifact, not the migration).
pub fn migrate_v3_to_v4(
    lockfile: Lockfile,
) -> LockfileResult<(crate::canonical::LockfileV4, Vec<String>)> {
    use crate::canonical::{LockfileV4, LockfileV4Metadata, PackageV4};
    use crate::v4::{Edge, EdgeKind, EdgeOrigin, PackageKey, SourceRef, VariantKey};

    let mut warnings = Vec::new();
    let mut sources: Vec<SourceRef> = Vec::new();
    let mut source_ids: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();

    // One source entry per distinct registry URL; registry-less pins
    // share the labeled "unknown" source.
    // (Một entry nguồn cho mỗi URL registry khác nhau.)
    for package in &lockfile.packages {
        let Some(registry) = package.registry.as_deref() else {
            continue;
        };
        if source_ids.contains_key(registry) {
            continue;
        }
        let id = format!("src-{}", &blake3_short_hex(registry)[..12]);
        source_ids.insert(registry.to_string(), id.clone());
        sources.push(SourceRef {
            id,
            url: registry.to_string(),
            ecosystem: package.ecosystem,
            priority: 100,
            claims: vec!["*".to_string()],
            trusted: false,
            allow_hosts: Vec::new(),
            allow_cidrs: Vec::new(),
            allow_protocols: vec!["https".to_string()],
        });
    }
    let unknown_used = lockfile.packages.iter().any(|p| p.registry.is_none());
    if unknown_used {
        sources.push(SourceRef {
            id: "unknown".to_string(),
            url: String::new(),
            ecosystem: crate::EcosystemTag::Other,
            priority: crate::v4::UNKNOWN_SOURCE_PRIORITY,
            claims: Vec::new(),
            trusted: false,
            allow_hosts: Vec::new(),
            allow_cidrs: Vec::new(),
            allow_protocols: Vec::new(),
        });
        warnings.push(
            "some pins have no registry: they share source \"unknown\" — re-resolve to attribute them"
                .to_string(),
        );
    }

    let mut packages_v4 = Vec::new();
    for package in &lockfile.packages {
        let source_id = package
            .registry
            .as_deref()
            .and_then(|r| source_ids.get(r))
            .cloned()
            .unwrap_or_else(|| "unknown".to_string());
        let key = PackageKey {
            ecosystem: package.ecosystem,
            name: crate::v4::canonical_name(package.ecosystem, &package.name),
            version: crate::v4::canonical_version(&package.version),
            source_id,
            variant: VariantKey {
                peer_context: None,
                feature_set: package.extras.clone(),
                target: None,
            },
        };
        let mut edges = Vec::new();
        for dep_name in &package.dependencies {
            let hits: Vec<&Package> = lockfile
                .packages
                .iter()
                .filter(|p| &p.name == dep_name)
                .collect();
            match hits.as_slice() {
                [hit] => edges.push(Edge {
                    target_key: PackageKey {
                        ecosystem: hit.ecosystem,
                        name: crate::v4::canonical_name(hit.ecosystem, &hit.name),
                        version: crate::v4::canonical_version(&hit.version),
                        source_id: hit
                            .registry
                            .as_deref()
                            .and_then(|r| source_ids.get(r))
                            .cloned()
                            .unwrap_or_else(|| "unknown".to_string()),
                        variant: VariantKey::default(),
                    },
                    range: "*".to_string(),
                    kind: EdgeKind::Normal,
                    origin: EdgeOrigin::Transitive {
                        from_key: Box::new(key.clone()),
                    },
                    marker: None,
                }),
                [] => warnings.push(format!(
                    "dropped edge {} -> {}: no locked package bears that name",
                    package.name, dep_name
                )),
                _ => warnings.push(format!(
                    "dropped edge {} -> {}: {} locked packages share that name (ambiguous without versions)",
                    package.name,
                    dep_name,
                    hits.len()
                )),
            }
        }
        if let Some(peers) = package.peers.as_ref()
            && !peers.is_empty()
        {
            warnings.push(format!(
                "dropped {} peer(s) of {}: v3 peers carry no versions, peer digests cannot be computed",
                peers.len(),
                package.name
            ));
        }
        if package.integrity.is_empty() {
            warnings.push(format!(
                "pin {}@{} has no integrity: v4 verify will fail on it until re-fetched",
                package.name, package.version
            ));
        }
        packages_v4.push(PackageV4 {
            key,
            edges,
            markers: package.markers.clone(),
            artifact: carry_install_sri(package, &mut warnings),
            provenance: package.provenance.clone(),
            toolchain: package.toolchain.clone(),
            scripts_policy: package.scripts_policy.clone(),
            store_ref: package.store_ref.clone(),
        });
    }

    let mut root_dependencies = lockfile.root_dependencies.clone();
    root_dependencies.sort();
    let v4 = LockfileV4 {
        version: crate::v4::LOCKFILE_SCHEMA_V4.to_string(),
        metadata: LockfileV4Metadata {
            generated_at: String::new(),
            generator: lockfile.metadata.generator.clone(),
            lockfile_hash: String::new(),
            signature: None,
        },
        sources,
        peer_contexts: Default::default(),
        root_dependencies,
        root_dependencies_by_owner: lockfile.root_dependencies_by_owner.clone(),
        packages: packages_v4,
        workspace: lockfile.workspace.clone(),
        optimizer_profile: lockfile.optimizer_profile.clone(),
    };
    Ok((v4, warnings))
}

/// Short BLAKE3 hex for generated source ids — Hex BLAKE3 ngắn cho id
/// nguồn sinh ra.
fn blake3_short_hex(input: &str) -> String {
    use mgc_crypto::blake3_signer::Blake3Hasher;
    Blake3Hasher::hash_string(input).to_hex()
}
/// Parse lockfile v1 from TOML string — Parse lockfile v1 từ chuỗi TOML
pub fn parse_lockfile_v1(toml_str: &str) -> LockfileResult<LockfileV1> {
    let lockfile_v1: LockfileV1 = toml::from_str(toml_str)?;

    if lockfile_v1.version != "1" {
        return Err(LockfileError::ParseError(format!(
            "expected version 1, got {}",
            lockfile_v1.version
        )));
    }

    Ok(lockfile_v1)
}

/// Auto-upgrade lockfile to the latest v3 schema (v1/v2 → v3) — Tự động nâng cấp lockfile lên schema v3 mới nhất (v1/v2 → v3).
///
/// v4 is NEVER an automatic target: the major bump changes identity,
/// digest coverage, and signature shape, so migration stays an explicit
/// `mgc migrate lock --to v4` decision. A v4 document here is an error
/// pointing at the v4 reader.
/// (v4 KHÔNG BAO GIỜ là đích tự động: bump major đổi identity, phủ
/// digest và shape chữ ký, nên migration là quyết định tường minh.)
pub fn auto_upgrade_lockfile(toml_str: &str) -> LockfileResult<Lockfile> {
    let version = detect_lockfile_version(toml_str)?;

    match version {
        1 => {
            let lockfile_v1 = parse_lockfile_v1(toml_str)?;
            // Chain both steps so old locks land on the canonical v3 graph.
            // Nối cả hai bước để lock cũ về đồ thị canonical v3.
            Ok(migrate_v2_to_v3(migrate_v1_to_v2(lockfile_v1)?))
        }
        2 => {
            let lockfile = crate::parser::parse_lockfile(toml_str)?;
            Ok(migrate_v2_to_v3(lockfile))
        }
        3 => crate::parser::parse_lockfile(toml_str),
        4 => Err(LockfileError::ParseError(
            "v4 lockfile: use the v4 reader (migration is explicit, never automatic)".to_string(),
        )),
        _ => Err(LockfileError::ParseError(format!(
            "unsupported version: {}",
            version
        ))),
    }
}

/// L7 FIX: BLAKE3 placeholder hash for migration (cryptographic, not DefaultHasher)
/// Still placeholder — production should warn user to re-install packages
fn blake3_placeholder_hash(s: &str) -> String {
    use mgc_crypto::blake3_signer::Blake3Hasher;
    let hash = Blake3Hasher::hash_string(s);
    hash.to_base64()
}

/// Lossy migration refusal — the v3 rows cannot become v4 without dropping
/// evidence the installer needs (edges, peers, sources, SRI). The caller
/// keeps the v3 lock untouched and reports these warnings loudly.
/// (Từ chối migration hao hụt — row v3 không thành v4 được mà không rớt bằng
/// chứng installer cần. Caller giữ nguyên lock v3 và báo warnings rõ.)
#[derive(Debug, Clone)]
pub struct LossyMigration {
    /// Every dropped piece of evidence — Mọi bằng chứng bị rớt.
    pub warnings: Vec<String>,
}

impl std::fmt::Display for LossyMigration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "v3 lock cannot become v4 losslessly ({} gap(s)): {}; resolve or re-install to fill them, or stay on v3",
            self.warnings.len(),
            self.warnings.join("; ")
        )
    }
}

impl std::error::Error for LossyMigration {}

/// Migrate a merged v3 lock to a finalized v4 document text, refusing
/// lossy migrations fail-closed (shared by every adapter write path so the
/// gate cannot drift between lanes).
/// (Migrate lock v3 đã merge thành text v4 hoàn thiện, từ chối hao hụt
/// fail-closed (dùng chung mọi đường ghi adapter để gate không lệch giữa
/// các lane).)
pub fn finalize_migrated_v4(
    legacy: Lockfile,
    generator: &str,
    generated_at: &str,
) -> Result<String, LossyMigration> {
    let (mut v4, warnings) = migrate_v3_to_v4(legacy).map_err(|error| LossyMigration {
        warnings: vec![error.to_string()],
    })?;
    if !warnings.is_empty() {
        return Err(LossyMigration { warnings });
    }
    v4.metadata.generator = generator.to_string();
    v4.metadata.generated_at = generated_at.to_string();
    v4.metadata.lockfile_hash = crate::canonical::payload_digest(&v4.payload());
    let text = crate::canonical::write_v4_document(&v4).map_err(|_| LossyMigration {
        warnings: vec!["v4 serialization failed".to_string()],
    })?;
    let round_trip = crate::canonical::parse_v4_document(&text).map_err(|_| LossyMigration {
        warnings: vec!["v4 round-trip parse failed".to_string()],
    })?;
    if round_trip != v4 {
        return Err(LossyMigration {
            warnings: vec!["v4 serializer drift".to_string()],
        });
    }
    Ok(text)
}

/// Convert one lane's freshly resolved v3 rows into v4 packages + sources,
/// refusing lossy rows fail-closed (shared by every adapter write path).
/// The lane rows — not a merged multi-core lock — are converted, so a
/// lane can never silently inherit another lane's lossy history.
/// (Chuyển row v3 vừa resolve của một lane thành package + source v4,
/// từ chối row hao hụt fail-closed (dùng chung mọi đường ghi adapter).
/// Chỉ convert row của lane, không bao giờ lặng lẽ thừa hưởng hao hụt
/// của lane khác.)
pub fn lane_rows_to_v4(
    packages: Vec<Package>,
    generator: &str,
) -> Result<(Vec<crate::canonical::PackageV4>, Vec<crate::v4::SourceRef>), LossyMigration> {
    let lane_lock = Lockfile {
        version: LOCKFILE_SCHEMA_VERSION.to_string(),
        metadata: crate::schema::LockfileMetadata {
            generated_at: String::new(),
            generator: generator.to_string(),
            lockfile_hash: String::new(),
            signer: None,
            dependency_ownership: Vec::new(),
        },
        root_dependencies: Vec::new(),
        root_dependencies_by_owner: Default::default(),
        packages,
        workspace: None,
        optimizer_profile: None,
    };
    let (v4, warnings) = migrate_v3_to_v4(lane_lock).map_err(|error| LossyMigration {
        warnings: vec![error.to_string()],
    })?;
    if !warnings.is_empty() {
        return Err(LossyMigration { warnings });
    }
    Ok((v4.packages, v4.sources))
}

/// Read a v4 merge base for lane writes: `Some` document when the lock on
/// disk is already v4; `None` with `had_legacy_rows` telling whether
/// sibling pins were left behind by a non-v4 base (the caller warns loudly
/// instead of silently dropping them).
/// (Đọc base v4 để lane merge: `Some` khi lock đĩa đã v4; `None` kèm
/// `had_legacy_rows` cho biết có pin khác bị bỏ lại vì base không v4
/// (caller cảnh báo rõ thay vì lặng lẽ rớt).)
pub fn read_v4_merge_base(
    path: &std::path::Path,
) -> LockfileResult<(Option<crate::canonical::LockfileV4>, bool)> {
    let bytes = match crate::parser::read_lockfile_bytes(path) {
        Ok(bytes) => bytes,
        Err(LockfileError::IoError(error)) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok((None, false));
        }
        Err(error) => return Err(error),
    };
    let text = std::str::from_utf8(&bytes).map_err(|error| {
        LockfileError::ParseError(format!("lockfile is not valid UTF-8: {error}"))
    })?;
    match crate::parser::parse_document(text)? {
        crate::parser::LockDocument::V4(doc) => Ok((Some(doc), false)),
        crate::parser::LockDocument::Legacy(lock) => Ok((None, !lock.packages.is_empty())),
    }
}

/// Verify a v4 merge base before an adapter preserves its sibling rows:
/// digest math must hold (Warn policy, no trust roots — same contract as
/// the web lane reader). A tampered base fails instead of being copied
/// verbatim into the new lock.
/// (Xác minh base v4 trước khi adapter giữ row khác của nó: toán digest
/// phải đúng (policy Warn, không trust root — cùng hợp đồng với reader
/// lane web). Base bị sửa thì fail thay vì copy nguyên vào lock mới.)
pub fn verify_v4_merge_base(doc: &crate::canonical::LockfileV4) -> LockfileResult<()> {
    let report = crate::policy::verify_v4_math(doc)?;
    crate::policy::enforce_policy(&report, crate::policy::LockPolicyMode::Warn, &[])?;
    Ok(())
}
