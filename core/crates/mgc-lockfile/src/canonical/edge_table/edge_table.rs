//! Writes an authenticated dependency-edge array item with full edge identity.
//! Ghi item mảng cạnh phụ thuộc đã xác thực với đầy đủ danh tính cạnh.

use crate::canonical::toml_escaping::escape_toml_string;
use crate::canonical::toml_fields::{push_str_field, push_string_array};
use crate::v4::{Edge, EdgeOrigin, PackageKey, VariantKey};

/// Emit ONE `[[package.edges]]` item with every edge field at item level
/// (target key, range, kind, origin, marker). Nested `[...]` headers are
/// avoided on purpose: the parser binds a bare scalar to the most recently
/// opened table, so a nested header would swallow every scalar after it.
/// (Ghi MỘT item `[[package.edges]]` với mọi field ở cấp item. Không dùng
/// header lồng `[...]`: parser gắn scalar vào bảng mở gần nhất nên header
/// lồng sẽ nuốt mọi scalar sau nó.)
pub(in crate::canonical) fn push_edge(out: &mut String, edge: &Edge) {
    out.push_str("[[package.edges]]\n");
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
        // The unit variant must be spelled as its exact Rust name: this TOML
        // reader matches the variant before applying any rename.
        // (Variant unit phải ghi đúng tên Rust: reader TOML khớp variant
        // trước khi áp rename.)
        EdgeOrigin::Manifest => push_str_field(out, "origin", "Manifest"),
        EdgeOrigin::Transitive { from_key } => {
            out.push_str("origin = { Transitive = { from_key = ");
            push_key_inline(out, from_key);
            out.push_str(" } }\n");
        }
    }
    if let Some(marker) = &edge.marker {
        push_str_field(out, "marker", marker);
    }
    // The target-key table closes the edge item: it declares the remaining
    // fields, so nothing may be emitted after it.
    // (Bảng target-key đóng item cạnh: nó khai các field còn lại nên không
    // được phát gì sau đó.)
    out.push_str("[package.edges.target_key]\n");
    push_str_field(out, "ecosystem", edge.target_key.ecosystem.as_str());
    push_str_field(out, "name", &edge.target_key.name);
    push_str_field(out, "version", &edge.target_key.version);
    push_str_field(out, "source_id", &edge.target_key.source_id);
    out.push_str("[package.edges.target_key.variant]\n");
    push_variant_fields(out, &edge.target_key.variant);
}

/// Render a full package key as one inline table (fixed field order,
/// including the variant so full identity stays signed).
/// Render package key trọn vẹn thành một bảng inline (thứ tự cố định, kèm
/// variant để toàn bộ danh tính vẫn được ký).
fn push_key_inline(out: &mut String, key: &PackageKey) {
    out.push_str("{ ecosystem = ");
    out.push_str(&escape_toml_string(key.ecosystem.as_str()));
    out.push_str(", name = ");
    out.push_str(&escape_toml_string(&key.name));
    out.push_str(", version = ");
    out.push_str(&escape_toml_string(&key.version));
    out.push_str(", source_id = ");
    out.push_str(&escape_toml_string(&key.source_id));
    out.push_str(", variant = ");
    push_variant_inline(out, &key.variant);
    out.push_str(" }");
}

/// Render a variant key as an inline table; feature sets are sorted so set
/// order can never change the authenticated bytes.
/// Render variant key thành bảng inline; tập feature đã sắp xếp nên thứ tự
/// tập không bao giờ đổi byte đã ký.
fn push_variant_inline(out: &mut String, variant: &VariantKey) {
    out.push_str("{ ");
    let mut first = true;
    if let Some(peer_context) = &variant.peer_context {
        push_inline_entry(
            out,
            &mut first,
            "peer_context",
            &escape_toml_string(peer_context),
        );
    }
    if let Some(feature_set) = &variant.feature_set {
        let mut sorted = feature_set.clone();
        sorted.sort();
        let rendered: Vec<String> = sorted.iter().map(|f| escape_toml_string(f)).collect();
        push_inline_entry(
            out,
            &mut first,
            "feature_set",
            &format!("[{}]", rendered.join(", ")),
        );
    }
    if let Some(target) = &variant.target {
        push_inline_entry(out, &mut first, "target", &escape_toml_string(target));
    }
    out.push_str(" }");
}

/// Emit the variant scalar fields (features sorted for determinism) through
/// the shared field writer, so a variant table and an inline variant can
/// never drift apart.
/// (Ghi field scalar của variant — feature đã sắp xếp — qua writer chung, để
/// bảng variant và variant inline không bao giờ lệch nhau.)
fn push_variant_fields(out: &mut String, variant: &VariantKey) {
    if let Some(peer_context) = &variant.peer_context {
        push_str_field(out, "peer_context", peer_context);
    }
    if let Some(feature_set) = &variant.feature_set {
        let mut sorted = feature_set.clone();
        sorted.sort();
        push_string_array(out, "feature_set", &sorted);
    }
    if let Some(target) = &variant.target {
        push_str_field(out, "target", target);
    }
}

/// Append `name = value` inside an inline table, comma-separated.
/// Nối `name = value` trong bảng inline, phân tách bằng dấu phẩy.
fn push_inline_entry(out: &mut String, first: &mut bool, name: &str, value: &str) {
    if !*first {
        out.push_str(", ");
    }
    *first = false;
    out.push_str(name);
    out.push_str(" = ");
    out.push_str(value);
}
