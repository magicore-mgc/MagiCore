//! Writes an authenticated dependency-edge table and nested variants.
//! Ghi bảng cạnh phụ thuộc đã xác thực cùng variant lồng nhau.

use crate::canonical::{package_table::push_variant, toml_fields::push_str_field};
use crate::v4::Edge;
pub(in crate::canonical) fn push_edge(out: &mut String, edge: &Edge) {
    out.push_str("[[package.edges]]\n");
    out.push_str("[package.edges.target_key]\n");
    push_str_field(out, "ecosystem", edge.target_key.ecosystem.as_str());
    push_str_field(out, "name", &edge.target_key.name);
    push_str_field(out, "version", &edge.target_key.version);
    push_str_field(out, "source_id", &edge.target_key.source_id);
    push_str_field(out, "range", &edge.range);
    push_str_field(
        out,
        "kind",
        match edge.kind {
            crate::v4::EdgeKind::Normal => "normal",
            crate::v4::EdgeKind::Dev => "dev",
            crate::v4::EdgeKind::Optional => "optional",
            crate::v4::EdgeKind::Peer => "peer",
        },
    );
    match &edge.origin {
        crate::v4::EdgeOrigin::Manifest => {
            push_str_field(out, "origin", "manifest");
        }
        crate::v4::EdgeOrigin::Transitive { from_key } => {
            out.push_str("[package.edges.origin.transitive]\n");
            push_str_field(out, "ecosystem", from_key.ecosystem.as_str());
            push_str_field(out, "name", &from_key.name);
            push_str_field(out, "version", &from_key.version);
            push_str_field(out, "source_id", &from_key.source_id);
        }
    }
    if let Some(marker) = &edge.marker {
        push_str_field(out, "marker", marker);
    }
    // Full edge identities must be signed, including target and origin variants.
    // Ký đầy đủ danh tính cạnh, gồm variant của target và origin.
    push_variant(
        out,
        "package.edges.target_key.variant",
        &edge.target_key.variant,
    );
    if let crate::v4::EdgeOrigin::Transitive { from_key } = &edge.origin {
        push_variant(
            out,
            "package.edges.origin.transitive.variant",
            &from_key.variant,
        );
    }
}
