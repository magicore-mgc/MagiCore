//! Keep dependency-edge kind wire names stable.
//! Giữ ổn định tên wire của loại cạnh phụ thuộc.

#![allow(clippy::unwrap_used)]

use super::*;

#[test]
fn edge_kinds_keep_lowercase_wire_values() {
    assert_eq!(
        serde_json::to_string(&EdgeKind::Normal).expect("test operation must succeed"),
        r#""normal""#
    );
    assert_eq!(
        serde_json::to_string(&EdgeKind::Dev).expect("test operation must succeed"),
        r#""dev""#
    );
    assert_eq!(
        serde_json::to_string(&EdgeKind::Optional).expect("test operation must succeed"),
        r#""optional""#
    );
    assert_eq!(
        serde_json::to_string(&EdgeKind::Peer).expect("test operation must succeed"),
        r#""peer""#
    );
}
