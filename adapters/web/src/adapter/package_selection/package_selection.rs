//! `package_selection.rs` — Package version and range selection / chọn version và range package.
use std::path::Path;

use super::super::WebAdapter;
use crate::exact_version_from_range;
use crate::native;
use crate::update::preferred_registry_version;
use mgc_types::{MgResult, PackageName, Version, VersionRange};

impl WebAdapter {
    pub fn metadata_versions(metadata: &native::npm_registry::PackageMetadata) -> Vec<Version> {
        let mut versions: Vec<Version> = metadata
            .versions
            .keys()
            .filter_map(|k| Version::parse(k).ok())
            .collect();
        versions.sort();
        versions
    }

    // COMPAT facade (S10a/P03) — real owner lives in `mgc-js-service::resolve`.
    // Moved verbatim (S10a, behavior byte-identical): `is_dist_tag_spec`.
    // Facade tương thích (S10a/P03) — owner thật nằm ở `mgc-js-service::resolve`.
    // Callers / Bên gọi — web-internal: `pin_dist_tags` (`resolution/resolution.rs`); test
    // Dist-tag test owner: `test/resolution_tests/dist_tag_resolution/dist_tag_resolution.rs`.
    // Owner test dist-tag — đường dẫn test cho nhánh này.
    // TODO(S10-cleanup): remove this facade after S10 migrates the last callers.
    /// True when a spec is a registry dist-tag (`latest`, `next`, …) rather
    /// than semver: all-alphabetic words only. Anything with a digit or a
    /// range operator keeps the semver path (unchanged behavior + errors).
    /// (Nhận diện dist-tag: chỉ chữ cái mới là tag.)
    pub(crate) fn is_dist_tag_spec(spec: &str) -> bool {
        mgc_js_service::resolve::is_dist_tag_spec(spec)
    }
    /// Pin dist-tag specs (`latest`, `next`, `beta`, …) through the
    /// packument BEFORE solving: tags are registry aliases, not semver,
    /// and the semver matcher rejects them ("no version matches
    /// 'latest'"). Unknown tags fail closed listing available tags.
    /// (Ghim dist-tag qua packument TRƯỚC khi solve — tag là alias
    /// registry, không phải semver.)
    pub(in crate::adapter) async fn pin_dist_tags(
        &self,
        wanted: Vec<(PackageName, String)>,
    ) -> MgResult<Vec<(PackageName, String)>> {
        let mut out = Vec::with_capacity(wanted.len());
        for (name, range) in wanted {
            if !Self::is_dist_tag_spec(&range) {
                out.push((name, range));
                continue;
            }
            let meta = self.provider.metadata(&name).await.map_err(|e| {
                mgc_types::MgError::Network(format!(
                    "dist-tag '{range}' needs {} metadata: {e}",
                    name.as_str()
                ))
            })?;
            match meta.dist_tags.get(range.trim()) {
                Some(pinned) => {
                    // A pinned tag is an explicit version choice — it must
                    // still satisfy the age gate (a young `latest` fails
                    // here instead of sneaking past semver matching).
                    // (Version ghim từ tag vẫn phải qua cổng tuổi.)
                    crate::provider::check_pinned_version(
                        &name,
                        &meta,
                        pinned,
                        self.provider.age_policy(),
                    )
                    .map_err(mgc_types::MgError::Other)?;
                    out.push((name, pinned.clone()))
                }
                None => {
                    let mut known: Vec<&str> = meta.dist_tags.keys().map(String::as_str).collect();
                    known.sort();
                    known.truncate(10);
                    return Err(mgc_types::MgError::Other(format!(
                        "unknown dist-tag '{range}' for '{}' (known tags: {})",
                        name.as_str(),
                        known.join(", ")
                    )));
                }
            }
        }
        Ok(out)
    }

    /// Infer the range to record, preferring a range the project already
    /// declares.
    /// Suy ra range cần ghi, ưu tiên range project đã khai.
    ///
    /// `project_root` is the caller's project because the adapter itself is
    /// stateless about which directory an operation targets.
    /// (`project_root` do caller truyền vì adapter không giữ trạng thái thư mục
    /// cho từng thao tác.)
    pub(in crate::adapter) async fn infer_add_range(
        &self,
        project_root: &Path,
        name: &PackageName,
        explicit_range: Option<&VersionRange>,
        exact: bool,
    ) -> MgResult<(VersionRange, Option<Version>)> {
        let should_fetch = match explicit_range {
            Some(range) => {
                let raw = range.as_str();
                raw == "latest" || raw == "*" || raw.is_empty()
            }
            None => true,
        };

        if should_fetch {
            let registry = native::npm_registry::NpmRegistry::new(&self.registry_url);
            let latest = self.latest_version_string(name, &registry).await?;
            let selected = Version::parse(&latest).ok();
            // `update` calls `prepare_add` with no range. An installed pin such
            // as `^1.0.0` must stay authoritative: saving `^{latest}` widened it
            // (recorded `^1.1.0` while `mgc.lock` still held 1.0.0) and made the
            // package silently follow `latest` on every later install.
            // (`update` gọi `prepare_add` không kèm range. Pin đã cài như
            // `^1.0.0` phải giữ quyền quyết định: lưu `^{latest}` nới rộng pin
            // (manifest ghi `^1.1.0` trong khi `mgc.lock` vẫn 1.0.0) và khiến
            // package âm thầm đi theo `latest` ở mọi lần cài sau.)
            let existing = self.existing_declared_range(project_root, name).await?;
            let saved = match existing.as_deref() {
                Some(raw) if !raw.trim().is_empty() => raw.trim().to_string(),
                _ => {
                    if exact {
                        latest
                    } else {
                        format!("^{latest}")
                    }
                }
            };
            return Ok((VersionRange::parse(&saved)?, selected));
        }

        if let Some(range) = explicit_range {
            let raw = if exact {
                range
                    .as_str()
                    .trim_start_matches('^')
                    .trim_start_matches('~')
            } else {
                range.as_str()
            };
            let parsed_range = VersionRange::parse(raw)?;
            return Ok((
                parsed_range.clone(),
                exact_version_from_range(&parsed_range),
            ));
        }

        unreachable!()
    }

    /// Range the project already declares for this package, if any.
    /// Range project đã khai cho package này, nếu có.
    async fn existing_declared_range(
        &self,
        project_root: &Path,
        name: &PackageName,
    ) -> MgResult<Option<String>> {
        let manifest = crate::manifest::parse_manifest(project_root)
            .map_err(|error| mgc_types::MgError::Other(error.to_string()))?;
        let declared = manifest
            .find_dep(name.as_str())
            .map(|dependency| dependency.range.as_str().to_string());
        Ok(declared)
    }

    async fn latest_version_string(
        &self,
        name: &PackageName,
        _registry: &native::npm_registry::NpmRegistry,
    ) -> MgResult<String> {
        let metadata = self
            .provider
            .metadata(name)
            .await
            .map_err(|err| mgc_types::MgError::Network(err.to_string()))?;

        preferred_registry_version(&metadata)
            .ok_or_else(|| {
                mgc_types::MgError::Other(format!(
                    "unable to infer latest version for '{}'",
                    name.as_str()
                ))
            })
            .and_then(|latest| {
                // An inferred `latest` is an explicit choice — it must pass
                // the age gate like any pin (a young latest fails here, not
                // downstream as "no version matches").
                // (Latest suy ra cũng phải qua cổng tuổi.)
                crate::provider::check_pinned_version(
                    name,
                    &metadata,
                    &latest,
                    self.provider.age_policy(),
                )
                .map_err(mgc_types::MgError::Other)?;
                Ok(latest)
            })
    }

    pub fn preferred_saved_range(current: &VersionRange, latest: &str) -> MgResult<VersionRange> {
        crate::update::preferred_saved_range(current, latest)
    }
}
