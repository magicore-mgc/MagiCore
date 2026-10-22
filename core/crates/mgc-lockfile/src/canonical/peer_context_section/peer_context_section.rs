//! Writes sorted peer contexts and their stable key/value encoding.
//! Ghi peer context đã sắp xếp và encoding key/value ổn định.

use crate::canonical::{LockfilePayload, toml_escaping::escape_toml_string};
pub(crate) fn append_peer_context_section(out: &mut String, payload: &LockfilePayload) {
    if !payload.peer_contexts.is_empty() {
        out.push_str("[peer_contexts]\n");
        for (digest, peers) in &payload.peer_contexts {
            let mut sorted = peers.clone();
            sorted.sort();
            let rendered: Vec<String> = sorted
                .iter()
                .map(|(name, version)| {
                    format!(
                        "[{}, {}]",
                        escape_toml_string(name),
                        escape_toml_string(version)
                    )
                })
                .collect();
            out.push_str(&escape_toml_string(digest));
            out.push_str(" = [");
            out.push_str(&rendered.join(", "));
            out.push_str("]\n");
        }
        out.push('\n');
    }
}
