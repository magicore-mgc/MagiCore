//! Expand requested Cargo features into active dependency edges.
//! Mở rộng feature Cargo thành các cạnh dependency hoạt động.

use super::super::super::super::{ActivatedCrateFeatures, CrateIndexEntry};
use super::{FeatureActivationError, FeatureActivationResult};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub(in crate::protocols::crates::features::features) fn activate_crate_features(
    entry: &CrateIndexEntry,
    requested_features: &BTreeSet<String>,
    default_features: bool,
) -> FeatureActivationResult<ActivatedCrateFeatures> {
    let mut features = entry.features.clone();
    for (name, values) in &entry.features2 {
        features
            .entry(name.clone())
            .or_default()
            .extend(values.iter().cloned());
    }
    let optional_aliases = entry
        .deps
        .iter()
        .filter(|dependency| dependency.optional && dependency.kind.as_deref() != Some("dev"))
        .map(|dependency| dependency.name.clone())
        .collect::<BTreeSet<_>>();
    let explicitly_named_optional = features
        .values()
        .flatten()
        .filter_map(|value| value.strip_prefix("dep:"))
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    let implicit_optional = optional_aliases
        .difference(&explicitly_named_optional)
        .cloned()
        .collect::<BTreeSet<_>>();

    let mut activation = ActivatedCrateFeatures::default();
    let mut pending = requested_features.iter().cloned().collect::<VecDeque<_>>();
    if default_features {
        pending.push_back("default".to_string());
    }
    let mut expanded = BTreeSet::new();
    let mut weak_features = BTreeMap::<String, BTreeSet<String>>::new();

    while let Some(feature) = pending.pop_front() {
        if !expanded.insert(feature.clone()) {
            continue;
        }
        let Some(values) = features.get(&feature) else {
            if implicit_optional.contains(&feature) {
                activation.optional_dependencies.insert(feature);
                continue;
            }
            if feature == "default" && default_features {
                continue;
            }
            return Err(FeatureActivationError::UndefinedFeature {
                package: entry.name.clone(),
                version: entry.vers.clone(),
                feature,
            });
        };
        for value in values {
            if let Some(alias) = value.strip_prefix("dep:") {
                if !entry.deps.iter().any(|dependency| dependency.name == alias) {
                    return Err(FeatureActivationError::MissingDependency {
                        package: entry.name.clone(),
                        version: entry.vers.clone(),
                        feature: feature.clone(),
                        alias: alias.to_string(),
                    });
                }
                activation.optional_dependencies.insert(alias.to_string());
                continue;
            }
            if let Some((dependency, child_feature)) = value.split_once('/') {
                let weak = dependency.ends_with('?');
                let alias = dependency.strip_suffix('?').unwrap_or(dependency);
                if alias.is_empty()
                    || child_feature.is_empty()
                    || !entry.deps.iter().any(|edge| edge.name == alias)
                {
                    return Err(FeatureActivationError::InvalidDependencyFeature {
                        package: entry.name.clone(),
                        version: entry.vers.clone(),
                        feature: feature.clone(),
                        value: value.clone(),
                    });
                }
                if weak {
                    weak_features
                        .entry(alias.to_string())
                        .or_default()
                        .insert(child_feature.to_string());
                } else {
                    activation.optional_dependencies.insert(alias.to_string());
                    activation
                        .dependency_features
                        .entry(alias.to_string())
                        .or_default()
                        .insert(child_feature.to_string());
                }
                continue;
            }
            if features.contains_key(value) {
                pending.push_back(value.clone());
            } else if implicit_optional.contains(value) {
                activation.optional_dependencies.insert(value.clone());
            } else {
                return Err(FeatureActivationError::UnknownFeatureReference {
                    package: entry.name.clone(),
                    version: entry.vers.clone(),
                    feature: feature.clone(),
                    value: value.clone(),
                });
            }
        }
    }

    for (alias, child_features) in weak_features {
        let dependency_is_active = entry
            .deps
            .iter()
            .any(|dependency| dependency.name == alias && !dependency.optional)
            || activation.optional_dependencies.contains(&alias);
        if dependency_is_active {
            activation
                .dependency_features
                .entry(alias)
                .or_default()
                .extend(child_features);
        } else if entry
            .deps
            .iter()
            .any(|dependency| dependency.name == alias && dependency.optional)
        {
            activation.lock_only_optional_dependencies.insert(alias);
        }
    }
    Ok(activation)
}
