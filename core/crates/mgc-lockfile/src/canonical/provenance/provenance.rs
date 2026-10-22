//! Converts the accepted v4 provenance wire shape into the shared Provenance type.
//! Chuyển wire shape provenance v4 được chấp nhận sang type Provenance dùng chung.

use crate::schema::Provenance;
use serde::Deserialize;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct V4Provenance {
    source_kind: String,
    #[serde(default)]
    tool: Option<String>,
    #[serde(default)]
    imported_from: Option<String>,
}

impl From<V4Provenance> for Provenance {
    fn from(value: V4Provenance) -> Self {
        Self {
            source_kind: value.source_kind,
            tool: value.tool,
            imported_from: value.imported_from,
        }
    }
}

pub(in crate::canonical) fn deserialize_v4_provenance<'de, D>(
    deserializer: D,
) -> Result<Option<Provenance>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<V4Provenance>::deserialize(deserializer).map(|provenance| provenance.map(Into::into))
}
