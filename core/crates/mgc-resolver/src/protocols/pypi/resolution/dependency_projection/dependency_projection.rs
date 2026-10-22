//! Convert PyPI Requires-Dist metadata to dependency edges and markers.
//! Chuyển metadata Requires-Dist PyPI thành dependency edge và marker.

use crate::protocols::pypi::{
    consumer_python, marker_applies, parse_pep508, requires_requested_extras,
};
use mgc_types::{MgError, MgResult, Version};

pub(in crate::protocols::pypi::resolution) fn project_dependencies(
    requires_dist: Option<Vec<String>>,
    name: &str,
    version: &Version,
    markers: &mut Vec<String>,
) -> MgResult<Vec<(String, String)>> {
    let Some(requires) = requires_dist else {
        return Ok(Vec::new());
    };
    let consumer = consumer_python();
    let mut deps = Vec::new();
    for spec in requires {
        let Some((dep_name, dep_range, marker)) = parse_pep508(&spec) else {
            return Err(MgError::Other(format!(
                "invalid PEP 508 Requires-Dist entry for {name} {version}: '{spec}'; refusing to omit it from the dependency graph"
            )));
        };
        if marker
            .as_deref()
            .is_some_and(|value| value.contains("extra"))
        {
            markers.push(format!("marker:{}", marker.unwrap_or_default()));
            continue;
        }
        if requires_requested_extras(&spec) {
            return Err(MgError::Unsupported {
                core: "python",
                capability: "dependency extras",
                guidance: format!(
                    "{name} {version} requires '{spec}', but extra propagation is not implemented; refusing to resolve an incomplete graph"
                ),
            });
        }
        if let Some(marker) = marker {
            if marker_applies(&marker, consumer) == Some(false) {
                markers.push(format!("marker-excluded:{marker}"));
                continue;
            }
            markers.push(format!("marker:{marker}"));
        }
        deps.push((dep_name, dep_range));
    }
    Ok(deps)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
