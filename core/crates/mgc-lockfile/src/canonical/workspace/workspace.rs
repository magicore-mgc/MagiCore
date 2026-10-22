//! Converts the accepted v4 workspace and cross-edge wire shape into shared schema types.
//! Chuyển wire shape workspace và cross-edge v4 sang type schema dùng chung.

use crate::schema::{CrossEdge, WorkspaceTopology};
use serde::Deserialize;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct V4WorkspaceTopology {
    #[serde(default)]
    members: Vec<String>,
    #[serde(default)]
    cross_core_edges: Vec<V4CrossEdge>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct V4CrossEdge {
    from_core: String,
    to_core: String,
    package: String,
}

impl From<V4WorkspaceTopology> for WorkspaceTopology {
    fn from(value: V4WorkspaceTopology) -> Self {
        Self {
            members: value.members,
            cross_core_edges: value
                .cross_core_edges
                .into_iter()
                .map(|edge| CrossEdge {
                    from_core: edge.from_core,
                    to_core: edge.to_core,
                    package: edge.package,
                })
                .collect(),
        }
    }
}

pub(in crate::canonical) fn deserialize_v4_workspace<'de, D>(
    deserializer: D,
) -> Result<Option<WorkspaceTopology>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<V4WorkspaceTopology>::deserialize(deserializer)
        .map(|workspace| workspace.map(WorkspaceTopology::from))
}
