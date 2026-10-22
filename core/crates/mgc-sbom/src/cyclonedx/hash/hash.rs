//! CycloneDX hash DTO — DTO hash CycloneDX

use serde::{Deserialize, Serialize};

/// Hash — Hash
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hash {
    /// Algorithm (SHA-256, BLAKE3, etc.) — Thuật toán
    pub alg: String,
    /// Content (hex) — Nội dung (hex)
    pub content: String,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
