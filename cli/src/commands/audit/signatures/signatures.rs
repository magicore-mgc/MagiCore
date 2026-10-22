//! Coordinate a bounded, paginated signature audit without changing CLI output.
//! Điều phối audit chữ ký có giới hạn và phân trang, giữ nguyên output CLI.

use super::{
    bundle_reader::{BundleReadError, read_bundle},
    error::SignatureAuditError,
    package_scope::{PackageScopeError, effective_signature_package},
    registry_chain::{RegistryChainError, verified_bundle_key, verify_page},
    sigstore_attestation::{
        SigstoreAttestationError, production_verifier, verify_public_attestation,
    },
};
use anyhow::Result;
use mgc_crypto::Ed25519PublicKey;
use sigstore_verify::Verifier;
use std::time::Duration;

const REQUEST_TIMEOUT_SECS: u64 = 15;

pub async fn run(
    core: Option<&str>,
    package: &str,
    protocol: &str,
    registry: &str,
    token: Option<&str>,
    json: bool,
) -> Result<()> {
    let effective_package = effective_signature_package(core, protocol, package)
        .map_err(PackageScopeError::into_cli)?;
    let package = effective_package.as_str();
    let scoped_package = mgc_registry_server::trusted::scoped_package(protocol, package)
        .map_err(|_| SignatureAuditError::Invalid.into_cli())?;
    let mut registry_url = url::Url::parse(registry)
        .map_err(|_| SignatureAuditError::RegistryUrlInvalid.into_cli())?;
    if !registry_url.username().is_empty()
        || registry_url.password().is_some()
        || registry_url.fragment().is_some()
        || registry_url.query().is_some()
    {
        return Err(SignatureAuditError::RegistryUrlInvalid.into_cli());
    }
    let local_http = registry_url.scheme() == "http"
        && registry_url.host_str().is_some_and(|host| {
            host == "localhost"
                || host
                    .parse::<std::net::IpAddr>()
                    .is_ok_and(|address| address.is_loopback())
        });
    if registry_url.scheme() != "https" && !local_http {
        return Err(SignatureAuditError::RequiresHttps.into_cli());
    }
    let prefix = registry_url.path().trim_end_matches('/');
    registry_url.set_path(&format!("{prefix}/-/v1/trusted-publish/signatures"));

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .build()?;
    let mut after_sequence = 0u64;
    let mut snapshot_head: Option<u64> = None;
    let mut previous_hash: Option<String> = None;
    let mut public_key: Option<Ed25519PublicKey> = None;
    let mut key_id: Option<String> = None;
    let mut verified_count = 0u64;
    let mut public_verified_count = 0u64;
    let mut legacy_internal_only_count = 0u64;
    let mut sigstore_verifier: Option<Verifier> = None;
    loop {
        let mut page_url = registry_url.clone();
        page_url.set_query(None);
        {
            let mut query = page_url.query_pairs_mut();
            query
                .append_pair("package", package)
                .append_pair("protocol", protocol)
                .append_pair("after_sequence", &after_sequence.to_string())
                .append_pair(
                    "limit",
                    &mgc_registry_server::trusted::SIGNATURE_PAGE_SIZE.to_string(),
                );
            if let Some(head) = snapshot_head {
                query.append_pair("through_sequence", &head.to_string());
            }
        }
        let mut request = client.get(page_url);
        if let Some(token) = token {
            request = request.bearer_auth(token);
        }
        let response = request.send().await?;
        if !response.status().is_success() {
            return Err(SignatureAuditError::RequestFailed(response.status().as_u16()).into_cli());
        }
        let page = read_bundle(response)
            .await
            .map_err(BundleReadError::into_cli)?;
        if page.package != scoped_package
            || page.entries.len() > mgc_registry_server::trusted::SIGNATURE_PAGE_SIZE
        {
            return Err(SignatureAuditError::Invalid.into_cli());
        }
        if page.head_sequence == 0 {
            return Err(SignatureAuditError::MissingOrMismatched.into_cli());
        }
        if snapshot_head.is_some_and(|head| head != page.head_sequence) {
            return Err(SignatureAuditError::Invalid.into_cli());
        }
        snapshot_head = Some(page.head_sequence);

        let page_key = verified_bundle_key(&page).map_err(RegistryChainError::into_cli)?;
        if public_key
            .as_ref()
            .is_some_and(|expected: &Ed25519PublicKey| expected != &page_key)
            || key_id
                .as_ref()
                .is_some_and(|expected: &String| expected != &page.key_id)
        {
            return Err(SignatureAuditError::Invalid.into_cli());
        }
        let expected_sequence = after_sequence
            .checked_add(1)
            .ok_or_else(|| SignatureAuditError::Invalid.into_cli())?;
        previous_hash = verify_page(
            &scoped_package,
            protocol,
            &page.entries,
            &page.key_id,
            &page_key,
            expected_sequence,
            previous_hash,
        )
        .map_err(RegistryChainError::into_cli)?;
        for entry in &page.entries {
            match (&entry.payload.artifact_sha256, &entry.sigstore_bundle) {
                (Some(_), Some(bundle)) => {
                    if sigstore_verifier.is_none() {
                        sigstore_verifier = Some(
                            production_verifier()
                                .await
                                .map_err(SigstoreAttestationError::into_cli)?,
                        );
                    }
                    verify_public_attestation(
                        entry,
                        bundle,
                        sigstore_verifier
                            .as_ref()
                            .ok_or_else(|| SignatureAuditError::Invalid.into_cli())?,
                    )
                    .map_err(SigstoreAttestationError::into_cli)?;
                    public_verified_count = public_verified_count
                        .checked_add(1)
                        .ok_or_else(|| SignatureAuditError::Invalid.into_cli())?;
                }
                (None, None) => {
                    legacy_internal_only_count = legacy_internal_only_count
                        .checked_add(1)
                        .ok_or_else(|| SignatureAuditError::Invalid.into_cli())?;
                }
                _ => return Err(SignatureAuditError::Invalid.into_cli()),
            }
        }
        public_key = Some(page_key);
        key_id = Some(page.key_id);
        verified_count = verified_count
            .checked_add(page.entries.len() as u64)
            .ok_or_else(|| SignatureAuditError::Invalid.into_cli())?;
        if let Some(last_entry) = page.entries.last() {
            after_sequence = last_entry.payload.sequence;
        }
        if after_sequence == page.head_sequence {
            break;
        }
        if page.entries.is_empty() || after_sequence > page.head_sequence {
            return Err(SignatureAuditError::Invalid.into_cli());
        }
    }
    let key_id = key_id.ok_or_else(|| SignatureAuditError::MissingOrMismatched.into_cli())?;
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "schema_version": 1,
                "package": package,
                "protocol": protocol,
                "verified": true,
                "entry_count": verified_count,
                "public_sigstore_verified_count": public_verified_count,
                "legacy_internal_only_count": legacy_internal_only_count,
                "head_sequence": snapshot_head,
                "key_id": key_id,
            }))?
        );
    } else {
        println!(
            "Verified {verified_count} {protocol} provenance entries for {package} (registry key {key_id}); {public_verified_count} have public Sigstore verification, {legacy_internal_only_count} are legacy registry-only entries."
        );
    }
    Ok(())
}
