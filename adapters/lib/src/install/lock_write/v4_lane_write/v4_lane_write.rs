//! Merge an owner-scoped V4 lane and verify its canonical digest.
//! Merge lane V4 theo owner và xác minh digest canonical.

use mgc_types::{MgError, MgResult};

/// v4 lane write: convert this lane's resolved rows losslessly, merge with
/// the v4 base on disk (sibling lanes preserved), finalize digest +
/// round-trip proof, and atomically replace the lock.
/// (Ghi v4 của lane: convert row đã resolve lossless, gộp với base v4 trên
/// đĩa (giữ lane khác), hoàn thiện digest + chứng minh round-trip và thay
/// atomic lock.)
pub(in crate::install::lock_write) fn write_canonical_lock_v4(
    lock_path: &std::path::Path,
    lock_packages: Vec<mgc_lockfile::Package>,
    owner_core: &str,
    replace_ecosystems: &[mgc_lockfile::EcosystemTag],
    direct_package_ids: &[String],
) -> MgResult<()> {
    use mgc_lockfile::migrate::{lane_rows_to_v4, read_v4_merge_base, verify_v4_merge_base};
    use mgc_lockfile::v4_graph::{merge_v4_lane_packages, v4_docs_match_material};
    use mgc_lockfile::writer::finalize_v4_document;
    let generator = format!("mgc/{}", env!("CARGO_PKG_VERSION"));
    let mut lane_rows = lock_packages;
    for package in &mut lane_rows {
        package.owner_core = Some(owner_core.to_string());
    }
    let lane_rows: Vec<mgc_lockfile::Package> = lane_rows
        .into_iter()
        .filter(|package| replace_ecosystems.contains(&package.ecosystem))
        .collect();
    let (lane_packages, lane_sources) =
        lane_rows_to_v4(lane_rows, &generator).map_err(|error| {
            MgError::Other(format!("v4 lane write refused (lossy lane rows): {error}"))
        })?;
    let (base, had_legacy_rows) =
        read_v4_merge_base(lock_path).map_err(|error| MgError::Other(error.to_string()))?;
    // Sibling rows are preserved verbatim: the base must verify first, or a
    // tampered lock would flow into the new document unchecked.
    // (Row khác được giữ nguyên văn: base phải xác minh trước, nếu không lock
    // bị sửa sẽ chảy vào lock mới mà không ai kiểm.)
    if let Some(base_doc) = &base {
        verify_v4_merge_base(base_doc).map_err(|error| MgError::Other(error.to_string()))?;
    }
    let mut lane_roots: Vec<String> = direct_package_ids
        .iter()
        .map(|id| canonicalize_v4_root_id(replace_ecosystems, id))
        .collect();
    lane_roots.sort();
    lane_roots.dedup();
    let base_material = base.clone();
    let (mut doc, dropped) = merge_v4_lane_packages(
        base,
        lane_packages,
        lane_sources,
        lane_roots,
        owner_core,
        &generator,
    );
    if had_legacy_rows || dropped {
        // A non-v4 base cannot contribute lossless rows: report, never silent.
        // (Base không v4 không đóng góp lossless được: báo rõ, không lặng.)
        eprintln!(
            "WARNING: non-v4 mgc.lock rows were replaced by the v4 lane write (sibling pins need `mgc migrate lock --to v4` or re-install under v4 schema)"
        );
    }
    // Byte-identical replay: a merge that changes no material row skips the
    // rewrite (parity with the v3 unchanged short-circuit).
    // (Replay giữ nguyên byte: merge không đổi row material nào thì bỏ qua
    // ghi lại (parity với short-circuit v3).)
    if let Some(base_doc) = &base_material
        && v4_docs_match_material(base_doc, &doc)
        && !dropped
    {
        return Ok(());
    }
    // Signed v4 on disk is never rewritten without explicit re-signing.
    // (v4 đã ký trên đĩa không bao giờ bị ghi đè nếu không ký lại tường minh.)
    if let Some(base_doc) = &base_material {
        mgc_lockfile::atomic::ensure_v4_unsigned_for_rewrite(base_doc)
            .map_err(|error| MgError::Other(error.to_string()))?;
    }
    doc.metadata.generated_at = chrono::Utc::now().to_rfc3339();
    let text = finalize_v4_document(&mut doc).map_err(|error| MgError::Other(error.to_string()))?;
    mgc_lockfile::atomic::atomic_write_v4_bytes(lock_path, text.as_bytes())
        .map_err(|error| MgError::Other(error.to_string()))?;
    Ok(())
}

/// Match V4 root identities to the canonical package keys built for a lane.
/// Gắn identity root V4 với package key canonical của lane.
fn canonicalize_v4_root_id(ecosystems: &[mgc_lockfile::EcosystemTag], id: &str) -> String {
    let [ecosystem] = ecosystems else {
        return id.to_string();
    };
    if *ecosystem != mgc_lockfile::EcosystemTag::NuGet {
        return id.to_string();
    }
    let Some((name, version)) = id.rsplit_once('@') else {
        return id.to_string();
    };
    format!(
        "{}@{}",
        mgc_lockfile::v4::canonical_name(*ecosystem, name),
        mgc_lockfile::v4::canonical_version(version)
    )
}
