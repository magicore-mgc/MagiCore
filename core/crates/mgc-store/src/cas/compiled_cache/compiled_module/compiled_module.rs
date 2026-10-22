//! Compiled JavaScript payload exchanged with cache callers.
//! Payload JavaScript đã biên dịch trao đổi với caller cache.

use serde::{Deserialize, Serialize};

/// Compiled output payload (what callers put in and get back).
/// (Payload output đã biên dịch — thứ caller đưa vào và nhận lại.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledModule {
    pub js: String,
    pub source_map: Option<String>,
}
