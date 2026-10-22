//! Writes package identity and variant tables in canonical order.
//! Ghi bảng định danh package và variant theo thứ tự canonical.

use crate::canonical::toml_fields::{push_str_field, push_string_array};
use crate::v4::PackageKey;
pub(in crate::canonical) fn push_package_key(out: &mut String, key: &PackageKey) {
    out.push_str("[package.key]\n");
    push_str_field(out, "ecosystem", key.ecosystem.as_str());
    push_str_field(out, "name", &key.name);
    push_str_field(out, "version", &key.version);
    push_str_field(out, "source_id", &key.source_id);
    push_variant(out, "package.key.variant", &key.variant);
}

pub(in crate::canonical) fn push_variant(
    out: &mut String,
    table: &str,
    variant: &crate::v4::VariantKey,
) {
    if variant.peer_context.is_some() || variant.feature_set.is_some() || variant.target.is_some() {
        out.push('[');
        out.push_str(table);
        out.push_str("]\n");
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
}
