//! Writes workspace membership and cross-core edges in canonical order.
//! Ghi thành viên workspace và cạnh liên core theo thứ tự canonical.

use crate::canonical::{
    LockfilePayload,
    toml_fields::{push_str_field, push_string_array},
};
pub(crate) fn append_workspace_section(out: &mut String, payload: &LockfilePayload) {
    if let Some(workspace) = &payload.workspace {
        out.push_str("[workspace]\n");
        let mut members = workspace.members.clone();
        members.sort();
        push_string_array(out, "members", &members);

        let mut cross_core_edges = workspace.cross_core_edges.clone();
        cross_core_edges.sort_by(|left, right| {
            (&left.from_core, &left.to_core, &left.package).cmp(&(
                &right.from_core,
                &right.to_core,
                &right.package,
            ))
        });
        for edge in &cross_core_edges {
            out.push_str("[[workspace.cross_core_edges]]\n");
            push_str_field(out, "from_core", &edge.from_core);
            push_str_field(out, "to_core", &edge.to_core);
            push_str_field(out, "package", &edge.package);
            out.push('\n');
        }
        out.push('\n');
    }
}
