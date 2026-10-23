//! Privacy predicates deciding which dev-server paths may be served.
//! Predicate privacy quyết định path nào được dev server phục vụ.

use std::path::{Component, Path};

pub(crate) fn is_public_well_known_request(path: &Path) -> bool {
    matches!(
        path.components().next(),
        Some(Component::Normal(component)) if component == ".well-known"
    )
}

pub(crate) fn is_public_well_known_target(path: &Path) -> bool {
    let mut components = path.components();
    matches!(
        (components.next(), components.next()),
        (Some(Component::Normal(public)), Some(Component::Normal(well_known)))
            if public == "public" && well_known == ".well-known"
    )
}

pub(crate) fn has_private_component(
    path: &Path,
    allow_well_known: bool,
    project_relative: bool,
) -> bool {
    let mut normal_component_index = 0;
    let mut first_component_is_public = false;

    for component in path.components() {
        let Component::Normal(value) = component else {
            continue;
        };
        let value = value.to_string_lossy();

        if value.starts_with('.') {
            if !allow_well_known || value != ".well-known" {
                return true;
            }

            let well_known_is_public = if project_relative {
                normal_component_index == 1 && first_component_is_public
            } else {
                normal_component_index == 0
            };
            if !well_known_is_public {
                return true;
            }
        }

        if normal_component_index == 0 {
            first_component_is_public = value == "public";
        }
        normal_component_index += 1;
    }

    false
}
