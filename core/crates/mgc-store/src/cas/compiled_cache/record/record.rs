//! Self-validating disk record and closed validation outcomes.
//! Record trên đĩa tự kiểm tra và kết quả validation tập đóng.

use super::super::output_digest::output_digest_of;
use super::super::{COMPILED_CACHE_SCHEMA_VERSION, CompilationKey, CompiledModule};
use serde::{Deserialize, Serialize};

/// Outcome of record self-validation (P0-3) — typed, not stringly.
/// (Kết quả tự kiểm tra record (P0-3) — typed, không chuỗi hóa.)
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::cas::compiled_cache) enum VerifyOutcome {
    /// Record is authentic for this key — safe to serve.
    /// (Record xác thực đúng key này — an toàn để phục vụ.)
    Authentic,
    /// Record was written by an older schema — a legit old entry, NOT
    /// corruption: report a miss so the caller recompiles and replaces it.
    /// (Record do schema cũ ghi — entry hợp lệ của thời trước, KHÔNG phải
    /// hỏng: báo miss để caller biên dịch lại và thay nó.)
    StaleSchema,
    /// Verification failed (key/digest mismatch) — poison or tampering.
    /// Carries the human-readable reason for the forensic report.
    /// (Verify fail (lệch key/digest) — độc hoặc gian lận. Mang lý do đọc
    /// được cho báo cáo điều tra.)
    Failed(String),
}

/// Self-validating on-disk record (P0-3): payload + the digest that
/// proves the payload was produced for exactly this key by exactly this
/// compiler. `get()` refuses to serve anything that fails verification.
/// NOT self-AUTHENTICATING: the digest is unkeyed BLAKE3 — it defends
/// against accidental corruption and torn writes, NOT against a
/// same-user malicious writer (see the threat-model header at the top).
///
/// Record trên đĩa TỰ KIỂM TRA (P0-3): payload + digest chứng minh payload
/// được sinh cho đúng key này bởi đúng compiler này. `get()` từ chối phục
/// vụ bất cứ gì rơi khỏi bước verify. KHÔNG phải tự XÁC THỰC: digest là
/// BLAKE3 không khóa — chống hỏng ngẫu nhiên và ghi đứt, KHÔNG chống
/// writer cùng user cố tình (xem threat-model đầu file).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(in crate::cas::compiled_cache) struct CompiledRecord {
    /// `#[serde(default)]`: records from an older schema (or hand-crafted
    /// garbage) parse with 0 → treated as STALE, reported as a miss and
    /// replaced — never served.
    /// (`#[serde(default)]`: record từ schema cũ (hoặc rác cố tình) parse
    /// ra 0 → coi là STALE, báo miss và bị thay — không bao giờ phục vụ.)
    #[serde(default)]
    schema_version: u32,
    key: CompilationKey,
    /// BLAKE3 hex over `js || 0x00 || source_map.unwrap_or("")` — the full
    /// output identity.
    /// BLAKE3 hex trên `js || 0x00 || source_map.unwrap_or("")` — toàn bộ
    /// identity của output.
    output_digest: String,
    js: String,
    source_map: Option<String>,
}

impl CompiledRecord {
    pub(in crate::cas::compiled_cache) fn from_module(
        key: &CompilationKey,
        module: &CompiledModule,
    ) -> Self {
        let output_digest = output_digest_of(&module.js, module.source_map.as_deref());
        Self {
            schema_version: COMPILED_CACHE_SCHEMA_VERSION,
            key: key.clone(),
            output_digest,
            js: module.js.clone(),
            source_map: module.source_map.clone(),
        }
    }

    pub(in crate::cas::compiled_cache) fn to_module(&self) -> CompiledModule {
        CompiledModule {
            js: self.js.clone(),
            source_map: self.source_map.clone(),
        }
    }

    pub(in crate::cas::compiled_cache) fn output_digest(&self) -> &str {
        &self.output_digest
    }

    /// Self-validation check (P0-3): embedded schema + key equality +
    /// output digest must ALL hold before this record may be served.
    ///
    /// Kiểm tra tự kiểm tra (P0-3): schema nhúng + bằng nhau key + digest
    /// output phải ĐỦ điều kiện trước khi record này được phục vụ.
    pub(in crate::cas::compiled_cache) fn verify_against(
        &self,
        expected_key: &CompilationKey,
    ) -> VerifyOutcome {
        if self.schema_version != COMPILED_CACHE_SCHEMA_VERSION {
            return VerifyOutcome::StaleSchema;
        }
        if &self.key != expected_key {
            return VerifyOutcome::Failed(format!(
                "key mismatch: entry holds loader '{}' compiler '{}'/'{}', requested loader '{}' compiler '{}'/'{}'",
                self.key.loader(),
                self.key.compiler(),
                self.key.compiler_version(),
                expected_key.loader(),
                expected_key.compiler(),
                expected_key.compiler_version()
            ));
        }
        if output_digest_of(&self.js, self.source_map.as_deref()) != self.output_digest {
            return VerifyOutcome::Failed("output digest mismatch (poisoned payload)".to_string());
        }
        VerifyOutcome::Authentic
    }
}
