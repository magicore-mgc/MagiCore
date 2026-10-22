//! Writes sorted package, edge, artifact and provenance tables.
//! Ghi package, cạnh, artifact và provenance đã sắp xếp.

use crate::canonical::{
    LockfilePayload,
    edge_table::push_edge,
    package_table::push_package_key,
    sort_key::{edge_sort_key, package_sort_key},
    toml_fields::{push_str_field, push_string_array},
};
pub(crate) fn append_package_section(out: &mut String, payload: &LockfilePayload) {
    let mut packages = payload.packages.clone();
    packages.sort_by_cached_key(package_sort_key);
    for package in &packages {
        out.push_str("[[package]]\n");
        if let Some(markers) = &package.markers {
            let mut markers = markers.clone();
            markers.sort();
            push_string_array(out, "markers", &markers);
        }
        push_package_key(out, &package.key);
        let mut edges = package.edges.clone();
        edges.sort_by_cached_key(edge_sort_key);
        for edge in &edges {
            push_edge(out, edge);
        }
        if let Some(artifact) = &package.artifact {
            out.push_str("[package.artifact]\n");
            push_str_field(out, "url", &artifact.url);
            if let Some(size) = artifact.size_bytes {
                out.push_str(&format!("size_bytes = {size}\n"));
            }
            push_str_field(out, "content_hash", &artifact.content_hash);
            push_str_field(out, "downloaded_from", &artifact.downloaded_from);
            // Only present when a lane supplied registry SRI — documents
            // without it digest exactly as before (no silent digest change).
            // Chỉ có khi lane cung cấp SRI — tài liệu thiếu field digest
            // y hệt như trước (không đổi digest ngầm).
            if let Some(sri) = artifact.integrity_sri.as_deref() {
                push_str_field(out, "integrity_sri", sri);
            }
        }
        if let Some(provenance) = &package.provenance {
            out.push_str("[package.provenance]\n");
            push_str_field(out, "source_kind", &provenance.source_kind);
            if let Some(tool) = &provenance.tool {
                push_str_field(out, "tool", tool);
            }
            if let Some(imported_from) = &provenance.imported_from {
                push_str_field(out, "imported_from", imported_from);
            }
        }
        if let Some(toolchain) = &package.toolchain {
            push_str_field(out, "toolchain", toolchain);
        }
        if let Some(policy) = &package.scripts_policy {
            push_str_field(out, "scripts_policy", policy);
        }
        if let Some(store_ref) = &package.store_ref {
            push_str_field(out, "store_ref", store_ref);
        }
        out.push('\n');
    }
}
