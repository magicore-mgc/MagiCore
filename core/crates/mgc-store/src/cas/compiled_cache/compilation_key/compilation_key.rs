//! Validated compilation context key and stable serialized identity.
//! Key ngữ cảnh biên dịch đã validate và identity serialize ổn định.

use super::super::{COMPILED_CACHE_SCHEMA_VERSION, Loader};
use crate::cas::integrity::{IntegrityHash, validate_blake3_hex};
use crate::cas::store::StoreError;
use serde::Serialize;
use std::path::PathBuf;

/// Versioned identity of ONE compilation (P0-1): everything the output
/// depends on must be part of the key, or two different compilations can
/// collide onto one cache entry.
///
/// INVARIANT (P0-1, Tech Lead vòng-5/6/7): ALL fields are PRIVATE. The
/// only construction path outside the crate is `CompilationKey::new`,
/// which validates every input; deserialization goes through the same
/// validation (`validate()`), so a hand-crafted record JSON with a junk
/// key can NEVER occupy a cache address. Struct-literal bypass and field
/// tampering are compile-time errors (see tests/compile_fail.rs).
///
/// Identity CÓ PHIÊN BẢN của MỘT lần biên dịch (P0-1): mọi thứ mà output
/// phụ thuộc vào phải nằm trong key, nếu không 2 lần biên dịch khác nhau có
/// thể đụng nhau trên 1 entry cache.
///
/// INVARIANT (P0-1): MỌI field là PRIVATE. Đường dựng duy nhất ngoài crate
/// là `CompilationKey::new` — validate mọi đầu vào; deserialize đi qua
/// cùng bước validate (`validate()`) nên record JSON tự chế với key rác
/// KHÔNG BAO GIỜ chiếm được địa chỉ cache. Struct-literal bypass và sửa
/// field trực tiếp là lỗi compile-time (xem tests/compile_fail.rs).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompilationKey {
    /// On-disk schema of the cache entry layout.
    /// Schema layout của entry cache trên đĩa.
    schema_version: u32,
    /// BLAKE3 hex of the source bytes (the ONLY content input).
    /// BLAKE3 hex của bytes source (đầu vào nội dung DUY NHẤT).
    source_digest: String,
    /// Loader semantics: the same text compiles differently per loader.
    /// Semantics loader: cùng text biên dịch khác nhau theo loader.
    loader: Loader,
    /// Compiler identity (e.g. "esbuild-rs").
    /// Danh tính compiler (vd "esbuild-rs").
    compiler: String,
    /// Compiler version — outputs of two compiler versions never share an
    /// entry.
    /// Phiên bản compiler — output của 2 phiên bản compiler không bao giờ
    /// dùng chung entry.
    compiler_version: String,
    /// BLAKE3 hex of the canonical compile-options JSON (platform, target,
    /// format, jsx mode, sourcemap, minify, define, ...).
    /// BLAKE3 hex của JSON options biên dịch chuẩn hóa (platform, target,
    /// format, jsx mode, sourcemap, minify, define, ...).
    options_digest: String,
}

impl CompilationKey {
    /// Build a validated compilation key. `options_json` must be a valid
    /// JSON object produced canonically by the caller (sorted keys) — its
    /// digest is embedded, the raw string is not stored.
    ///
    /// Dựng compilation key đã validate. `options_json` phải là JSON object
    /// hợp lệ do caller sinh chuẩn hóa (key đã sort) — digest của nó được
    /// nhúng, chuỗi thô không lưu.
    pub fn new(
        source_digest: &str,
        loader: Loader,
        compiler: &str,
        compiler_version: &str,
        options_json: &str,
    ) -> Result<Self, StoreError> {
        // Fail-closed digest validation — the source digest must be a real
        // 64-hex BLAKE3 value before it can address a cache entry.
        // (Validate digest fail-closed — digest source phải là BLAKE3 hex
        // 64 thật trước khi được làm địa chỉ entry cache.)
        validate_blake3_hex(source_digest).map_err(|e| StoreError::InvalidHash(e.input))?;
        if compiler.is_empty() || compiler_version.is_empty() {
            return Err(StoreError::InvalidHash(format!(
                "compilation key requires non-empty compiler/version (compiler='{compiler}')"
            )));
        }
        // The options must be well-formed JSON — a malformed string here is
        // a caller bug, fail loudly instead of hashing garbage.
        // (Options phải là JSON hợp lệ — chuỗi sai dạng là bug của caller,
        // hét to thay vì hash rác.)
        serde_json::from_str::<serde_json::Value>(options_json).map_err(|e| StoreError::Io {
            path: PathBuf::from("<compilation-options>"),
            msg: format!("options_json is not valid JSON: {e}"),
        })?;
        let options_digest = IntegrityHash::from_bytes(options_json.as_bytes(), false)
            .as_hex()
            .to_string();
        Ok(Self {
            schema_version: COMPILED_CACHE_SCHEMA_VERSION,
            source_digest: source_digest.to_string(),
            loader,
            compiler: compiler.to_string(),
            compiler_version: compiler_version.to_string(),
            options_digest,
        })
    }

    /// Re-validate EVERY field of a parsed key (P0-1): a key that came
    /// from a record's JSON must satisfy the same invariants as one built
    /// by `new()` — digests are real BLAKE3 hex, loader is in the closed
    /// set (already enforced by the `Loader` deserializer), and the
    /// identity fields are non-empty. Records failing this are corrupt,
    /// never served.
    ///
    /// Kiểm tra lại MỌI field của key đã parse (P0-1): key đến từ JSON
    /// record phải thỏa cùng invariant như key dựng bằng `new()` — digest
    /// là BLAKE3 hex thật, loader thuộc tập đóng (deserializer `Loader` đã
    /// cưỡng chế), và các field identity khác rỗng. Record fail chỗ này là
    /// hỏng, không bao giờ phục vụ.
    fn validate(&self) -> Result<(), StoreError> {
        validate_blake3_hex(&self.source_digest).map_err(|e| StoreError::InvalidHash(e.input))?;
        validate_blake3_hex(&self.options_digest).map_err(|e| StoreError::InvalidHash(e.input))?;
        if self.compiler.is_empty() || self.compiler_version.is_empty() {
            return Err(StoreError::InvalidHash(
                "compilation key has empty compiler identity".to_string(),
            ));
        }
        Ok(())
    }

    // ── Read-only accessors (P0-1): the fields are private; these are
    // the only way outside code can READ the key — no mutation path.
    // (Accessor chỉ-đọc (P0-1): field private; đây là cách duy nhất code
    // ngoài ĐỌC key — không có đường sửa.)

    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    pub fn loader(&self) -> Loader {
        self.loader
    }

    pub fn compiler(&self) -> &str {
        &self.compiler
    }

    pub fn compiler_version(&self) -> &str {
        &self.compiler_version
    }

    pub fn options_digest(&self) -> &str {
        &self.options_digest
    }

    /// Canonical serialized form (serde struct field order is fixed →
    /// deterministic byte-for-byte for the same key).
    /// (Dạng serialize chuẩn hóa — thứ tự field struct serde cố định → tất
    /// định từng byte cho cùng key.)
    fn canonical(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    /// Cache address for this key: BLAKE3 of the canonical key JSON. Two
    /// keys differing in ANY field → different address (the P0-1 collision
    /// is structurally impossible).
    ///
    /// Địa chỉ cache cho key này: BLAKE3 của JSON key chuẩn hóa. 2 key khác
    /// nhau ở BẤT KỲ field nào → địa chỉ khác (collision P0-1 về cấu trúc
    /// không thể xảy ra).
    pub fn master_digest(&self) -> IntegrityHash {
        IntegrityHash::from_bytes(self.canonical().as_bytes(), false)
    }
}

/// Deserialization contract (P0-1): a key parsed from record JSON goes
/// through `Loader`'s closed-set deserializer AND `validate()` — junk
/// digests or empty identity fields make the WHOLE record fail to parse,
/// which `get()` treats as corrupt (quarantine + CacheCorrupt), never a
/// serve.
///
/// Hợp đồng deserialize (P0-1): key parse từ record JSON đi qua
/// deserializer tập đóng của `Loader` VÀ `validate()` — digest rác hay
/// field identity rỗng làm CẢ record fail parse, và `get()` coi là hỏng
/// (quarantine + CacheCorrupt), không bao giờ phục vụ.
impl<'de> serde::Deserialize<'de> for CompilationKey {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        /// Wire shape mirroring the private fields (parse-then-validate).
        /// (Shape wire phản chiếu field private (parse-rồi-validate).)
        #[derive(serde::Deserialize)]
        struct CompilationKeyWire {
            schema_version: u32,
            source_digest: String,
            loader: Loader,
            compiler: String,
            compiler_version: String,
            options_digest: String,
        }
        let wire = CompilationKeyWire::deserialize(deserializer)?;
        let key = Self {
            schema_version: wire.schema_version,
            source_digest: wire.source_digest,
            loader: wire.loader,
            compiler: wire.compiler,
            compiler_version: wire.compiler_version,
            options_digest: wire.options_digest,
        };
        key.validate()
            .map_err(|e| serde::de::Error::custom(e.to_string()))?;
        Ok(key)
    }
}
