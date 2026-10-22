//! Writes one source table using canonical field and array order.
//! Ghi một bảng source theo thứ tự field và mảng canonical.

use crate::canonical::toml_fields::{push_str_field, push_string_array};
use crate::v4::SourceRef;
pub(in crate::canonical) fn push_source(out: &mut String, source: &SourceRef) {
    out.push_str("[[sources]]\n");
    push_str_field(out, "id", &source.id);
    push_str_field(out, "url", &source.url);
    push_str_field(out, "ecosystem", source.ecosystem.as_str());
    out.push_str(&format!("priority = {}\n", source.priority));
    push_string_array(
        out,
        "claims",
        &source
            .claims
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
    );
    out.push_str(&format!("trusted = {}\n", source.trusted));
    push_string_array(
        out,
        "allow_hosts",
        &source
            .allow_hosts
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
    );
    push_string_array(
        out,
        "allow_cidrs",
        &source
            .allow_cidrs
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
    );
    push_string_array(
        out,
        "allow_protocols",
        &source
            .allow_protocols
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
    );
}
