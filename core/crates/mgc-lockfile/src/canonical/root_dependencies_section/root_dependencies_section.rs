//! Writes direct root pins and per-owner roots with their established sorting rules.
//! Ghi pin root trực tiếp và root theo owner theo quy tắc sort hiện có.

use crate::canonical::{
    LockfilePayload, toml_escaping::escape_toml_string, toml_fields::push_string_array,
};
pub(crate) fn append_root_dependencies_section(out: &mut String, payload: &LockfilePayload) {
    if !payload.root_dependencies.is_empty() {
        let mut roots = payload.root_dependencies.clone();
        roots.sort();
        push_string_array(out, "root_dependencies", &roots);
        out.push('\n');
    }

    if !payload.root_dependencies_by_owner.is_empty() {
        out.push_str("[root_dependencies_by_owner]\n");
        for (owner, roots) in &payload.root_dependencies_by_owner {
            let mut roots = roots.clone();
            roots.sort();
            roots.dedup();
            let rendered: Vec<String> = roots.iter().map(|root| escape_toml_string(root)).collect();
            out.push_str(&escape_toml_string(owner));
            out.push_str(" = [");
            out.push_str(&rendered.join(", "));
            out.push_str("]\n");
        }
        out.push('\n');
    }
}
