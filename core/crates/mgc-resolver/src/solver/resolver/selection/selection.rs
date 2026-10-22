use super::super::Resolver;
use crate::solver::resolver::DedupePref;
use mgc_types::{PackageName, Version, VersionRange};
use std::collections::HashMap;
use std::sync::Arc;

impl Resolver {
    /// Pick the version to use: under PreferExisting, reuse the installed
    /// version when it satisfies the constraint (dedupe — 02 §2.1).
    pub(in crate::solver::resolver) fn select_version(
        &self,
        name: &str,
        constraint: &VersionRange,
        spec: &str,
        versions: &[Version],
    ) -> Option<Version> {
        // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
        // Poisoned preference memo falls back to normal selection
        // (PreferLatest) instead of panicking the resolve.
        let prefer_existing = matches!(
            self.dedupe_pref.read(),
            Ok(guard) if *guard == DedupePref::PreferExisting
        );
        if prefer_existing
            && let Some(existing) = self
                .existing_versions
                .read()
                .ok()
                .and_then(|guard| guard.get(name).cloned())
            && constraint.matches(&existing)
        {
            return Some(existing);
        }
        Self::select_best_version(versions, constraint, spec)
    }

    pub(in crate::solver::resolver) fn select_best_version(
        versions: &[Version],
        constraint: &VersionRange,
        spec: &str,
    ) -> Option<Version> {
        let mut matches: Vec<Version> = versions
            .iter()
            .filter(|v| constraint.matches(v))
            .cloned()
            .collect();
        if matches.is_empty() {
            return None;
        }

        let allows_prerelease = spec.contains('-');
        if !allows_prerelease {
            let stable: Vec<Version> = matches
                .iter()
                .filter(|v| v.pre.is_none())
                .cloned()
                .collect();
            if let Some(best) = stable.into_iter().max() {
                return Some(best);
            }
        }

        matches.sort();
        matches.into_iter().max()
    }

    pub(in crate::solver::resolver) fn versions_to_map(
        entries: Vec<(PackageName, Vec<Version>)>,
    ) -> HashMap<String, Arc<[Version]>> {
        entries
            .into_iter()
            .map(|(name, versions)| (name.as_str().to_string(), Arc::<[Version]>::from(versions)))
            .collect()
    }
}
