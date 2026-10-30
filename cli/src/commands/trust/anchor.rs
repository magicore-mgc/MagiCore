//! mgc trust anchor — Attest the project core identity
//! mgc trust anchor — Chứng thực identity core của project

/// Execute `mgc trust anchor` — Thực thi `mgc trust anchor`
pub fn execute(re_attest: bool, rotate: bool, key_id: Option<&str>) -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;
    let project_root =
        mgc_config::project::ProjectConfig::find_project_root(&cwd).unwrap_or_else(|| cwd.clone());
    let marker = mgc_config::project::ProjectConfig::read_core_marker(&project_root)?
        .ok_or_else(|| crate::error::trust_anchor_no_marker(&project_root))?;
    let globals = mgc_platform::paths::GlobalPaths::new()
        .map_err(|error| crate::error::trust_anchor_no_home(&error))?;
    let keyring_path = mgc_crypto::keyring::Keyring::default_path()
        .map_err(|error| crate::error::trust_anchor_failed(&error))?;

    if rotate && key_id.is_some() {
        return Err(crate::error::trust_anchor_rotate_needs_generated_key());
    }
    // SEC-12/N-02: pin the key *before* attesting, because the attestation
    // digest covers `mgc.toml` and verification refuses an unpinned project.
    // The key is written first, then the manifest that already lists it is
    // signed, which is the only order that leaves a project both pinned and
    // self-consistent.
    // (SEC-12/N-02: pin key **trước** khi attest vì digest attestation phủ
    // `mgc.toml` và verify từ chối project chưa pin. Ghi key trước, rồi ký
    // đúng manifest đã liệt kê key — thứ tự duy nhất để project vừa có pin vừa
    // nhất quán.)
    let public_key = mgc_config::attestation::keyring_public_key(&keyring_path, key_id)
        .map_err(|error| crate::error::trust_anchor_failed(&error))?;
    mgc_config::attestation::pin_key_in_manifest(&project_root, &public_key)
        .map_err(|error| crate::error::trust_anchor_failed(&error))?;
    let attestation = mgc_config::attestation::attest(
        &project_root,
        &marker,
        &keyring_path,
        &globals.identities,
        key_id,
        // Pinning just edited `mgc.toml`, so the attestation must be re-taken
        // over the pinned manifest; `re_attest` additionally covers a user's
        // deliberate edit to a project that was already anchored.
        // (Pin vừa sửa `mgc.toml`, nên attestation phải được tạo lại trên
        // manifest đã pin; `re_attest` bổ sung cho trường hợp user sửa có chủ
        // đích trên project đã anchor.)
        true || re_attest,
        rotate,
    )
    .map_err(|error| crate::error::trust_anchor_failed(&error))?;

    println!("✓ Core attested: {} ({})", marker, project_root.display());
    println!("  Key: {}", attestation.key_id);
    if !attestation.prev_keys.is_empty() {
        println!(
            "  Previous keys kept: {}",
            attestation
                .prev_keys
                .iter()
                .map(|key| key.key_id.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    println!("  Attestation lives outside the project — editing the project cannot forge it.");
    Ok(())
}

/// Best-effort attestation for `mgc init`: never fail project creation,
/// warn instead (the read path warns again on use).
/// (Chứng thực best-effort cho `mgc init`: không bao giờ làm hỏng tạo
/// project, chỉ cảnh báo.)
pub fn attest_new_project(project_root: &std::path::Path, core: &str) {
    let identities = match mgc_platform::paths::GlobalPaths::new() {
        Ok(paths) => paths.identities,
        Err(_) => return,
    };
    let keyring_path = match mgc_crypto::keyring::Keyring::default_path() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("WARNING: core attestation skipped ({error}) — run `mgc trust anchor` later");
            return;
        }
    };
    if let Err(error) = mgc_config::attestation::attest(
        project_root,
        core,
        &keyring_path,
        &identities,
        None,
        false,
        false,
    ) {
        eprintln!("WARNING: core attestation skipped ({error}) — run `mgc trust anchor` later");
    }
}
