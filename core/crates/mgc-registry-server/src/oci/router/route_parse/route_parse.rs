//! OCI route-shape parsing: split `/v2/{rest}` and validate section tails.
//! Parse hình dạng route OCI: tách `/v2/{rest}` và validate tail theo section.

/// Split `/v2/{rest}` into `(name, section, tail)`.
/// tail excludes the section, e.g. `/blobs/uploads/` -> tail `"/"`.
pub(crate) fn split_oci(rest: &str) -> Option<(&str, &str, &str)> {
    let mut selected: Option<(usize, &str, &str, &str)> = None;
    for section in ["blobs/uploads", "blobs", "manifests", "tags/list"] {
        let marker = format!("/{section}");
        for (index, _) in rest.rmatch_indices(&marker) {
            let end = index + marker.len();
            if end < rest.len() && rest.as_bytes()[end] != b'/' {
                continue;
            }
            let name = &rest[..index];
            let tail = &rest[end..];
            if name.is_empty() || !valid_oci_route_tail(section, tail) {
                continue;
            }
            let replace = selected
                .as_ref()
                .is_none_or(|(best_index, best_section, _, _)| {
                    index > *best_index
                        || (index == *best_index && section.len() > best_section.len())
                });
            if replace {
                selected = Some((index, section, name, tail));
            }
        }
    }
    selected.map(|(_, section, name, tail)| (name, section, tail))
}

fn valid_oci_route_tail(section: &str, tail: &str) -> bool {
    let is_single_segment = |value: &str| {
        value
            .strip_prefix('/')
            .is_some_and(|segment| !segment.is_empty() && !segment.contains('/'))
    };
    match section {
        "blobs/uploads" => tail.is_empty() || tail == "/" || is_single_segment(tail),
        "blobs" | "manifests" => is_single_segment(tail),
        "tags/list" => tail.is_empty(),
        _ => false,
    }
}
