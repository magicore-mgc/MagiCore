//! Writes version, generator and source tables before every other canonical section.
//! Ghi version, generator và bảng source trước các section canonical còn lại.

use crate::canonical::{LockfilePayload, source_table::push_source, toml_fields::push_str_field};
pub(crate) fn append_payload_header(out: &mut String, payload: &LockfilePayload) {
    push_str_field(out, "version", &payload.version);
    if let Some(profile) = &payload.optimizer_profile {
        push_str_field(out, "optimizer_profile", profile);
    }
    out.push('\n');
    out.push_str("[metadata]\n");
    push_str_field(out, "generator", &payload.generator);
    out.push('\n');
    let mut sources = payload.sources.clone();
    sources.sort_by(|a, b| a.id.cmp(&b.id));
    for source in &sources {
        push_source(out, source);
        out.push('\n');
    }
}
