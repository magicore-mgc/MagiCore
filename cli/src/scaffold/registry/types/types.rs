//! Dist-tag response wire type.
//! Kiểu wire của response dist-tag.

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct DistTags {
    #[serde(flatten)]
    pub(crate) tags: std::collections::HashMap<String, String>,
}
