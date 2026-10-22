//! Converts the accepted v4 artifact wire shape into the shared ArtifactRef type.
//! Chuyển wire shape artifact v4 được chấp nhận sang type ArtifactRef dùng chung.

use crate::schema::ArtifactRef;
use serde::Deserialize;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct V4ArtifactRef {
    url: String,
    #[serde(default)]
    size_bytes: Option<u64>,
    content_hash: String,
    downloaded_from: String,
    /// SRI integrity (optional — absent on older v4 documents).
    /// Integrity SRI (tùy chọn — vắng trên tài liệu v4 cũ).
    #[serde(default)]
    integrity_sri: Option<String>,
}

impl From<V4ArtifactRef> for ArtifactRef {
    fn from(value: V4ArtifactRef) -> Self {
        Self {
            url: value.url,
            size_bytes: value.size_bytes,
            content_hash: value.content_hash,
            downloaded_from: value.downloaded_from,
            integrity_sri: value.integrity_sri,
        }
    }
}

pub(in crate::canonical) fn deserialize_v4_artifact<'de, D>(
    deserializer: D,
) -> Result<Option<ArtifactRef>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<V4ArtifactRef>::deserialize(deserializer)
        .map(|artifact| artifact.map(ArtifactRef::from))
}
