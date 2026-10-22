//! Parses MagiCore temporary names using the exact prefix and field schema.
//! Parse tên temporary MagiCore theo prefix và schema field chính xác.

// ── TempFileName parser (Gate 11-B / P0-7, vòng-11 audit) ─────────────
//
// The doctor's orphan-temp GC used a LOOSE heuristic (`has '-', ≥4 dash
// parts, last part all digits`) which matched foreign names like
// `customer-important-backup-123` and deleted them once stale. The
// parser below is the SINGLE shared authority: a name is MGC-owned iff
// its PREFIX is in the allowlist AND the remaining fields parse exactly
// as `<pid>-<tid_hex>-<nanos>-<counter>`. The doctor calls this instead
// of guessing by shape.
//
// (Parser TempFileName (P0-7): GC temp mồ côi của doctor từng dùng
// heuristic LỎNG (`có '-', ≥4 phần, phần cuối toàn số`) — khớp cả tên
// ngoài như `customer-important-backup-123` và xóa nó khi đủ cũ. Parser
// dưới đây là nguồn duy nhất: một tên là CỦA MGC khi PREFIX nằm trong
// allowlist VÀ các field còn lại parse ĐÚNG dạng
// `<pid>-<tid_hex>-<nanos>-<counter>`. Doctor gọi cái này thay vì đoán
// theo hình dạng.)

/// Where an MGC temp file may legally live — a name alone is not enough,
/// the doctor must also confirm the DIRECTORY matches the purpose.
/// (Nơi một temp file MGC được phép nằm — tên thôi chưa đủ, doctor phải
/// xác nhận cả THƯ MỤC khớp purpose.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TempPurpose {
    /// CAS write/import staging — lives under `<store>/cas/tmp/`.
    /// (Staging ghi/import CAS — nằm trong `<store>/cas/tmp/`.)
    CasTmp,
    /// Compiled-record atomic write — lives next to the record (records/).
    /// (Ghi nguyên tử record biên dịch — nằm cạnh record (records/).)
    CompiledRecord,
}

/// The exact prefixes `unique_tmp_path` is called with today. `mgc-export`
/// and `mgc-export-probe` stage INSIDE the DESTINATION directory (never
/// the CAS tmp dir) and are therefore NEVER swept by the CAS tmp GC —
/// they are listed here only so a stray one found in tmp/ is recognized
/// as misfiled MGC data, not foreign data.
/// (Các prefix mà `unique_tmp_path` được gọi hôm nay. `mgc-export` và
/// `mgc-export-probe` staging TRONG THƯ MỤC ĐÍCH (không bao giờ tmp của
/// CAS) nên KHÔNG BAO GIỜ bị GC tmp CAS quét — liệt kê ở đây chỉ để một
/// file lạc trong tmp/ được nhận ra là dữ liệu MGC bị đặt sai chỗ, không
/// phải dữ liệu ngoài.)
const MGC_TEMP_PREFIXES: &[&str] = &[
    "import-file",
    "write-bytes",
    "compiled",
    "mgc-export",
    "mgc-export-probe",
];

/// Prefixes whose temps legally live under the CAS tmp directory — the
/// ONLY set the doctor's CAS-tmp GC may consider for sweeping.
/// (Prefix mà temp hợp pháp nằm trong thư mục tmp của CAS — tập DUY
/// NHẤT mà GC tmp của doctor được xét để quét.)
const CAS_TMP_SWEEPABLE_PREFIXES: &[&str] = &["import-file", "write-bytes"];

/// A parsed MGC temp file name — proof the name matches the exact
/// `<prefix>-<pid>-<tid_hex>-<nanos>-<counter>` schema.
/// (Tên temp MGC đã parse — bằng chứng tên khớp ĐÚNG schema
/// `<prefix>-<pid>-<tid_hex>-<nanos>-<counter>`.)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TempFileName {
    pub prefix: String,
    pub pid: u32,
    pub tid_hex: String,
    pub nanos: u128,
    pub counter: u64,
}

impl TempFileName {
    /// Strict parser — a name is MGC-owned ONLY if the prefix is in the
    /// allowlist and every remaining field parses with the exact type
    /// and arity. `customer-important-backup-123` fails (prefix not in
    /// the list); `write-bytes-1234-ab-999-0` parses; a truncated or
    /// extra-field name fails. NOTE: tid is parsed as non-empty HEX.
    /// (Parser nghiêm ngặt — tên là CỦA MGC CHỈ KHI prefix nằm trong
    /// allowlist và mọi field còn lại parse đúng type và đúng số lượng.
    /// `customer-important-backup-123` fail (prefix ngoài list);
    /// `write-bytes-1234-ab-999-0` parse; tên thiếu/thừa field fail.
    /// LƯU Ý: tid parse dạng hex không rỗng.)
    pub fn parse(name: &str) -> Option<Self> {
        // Prefixes themselves contain dashes (`import-file`, `mgc-export`)
        // — split on '-' would shred them. Match the LONGEST allowlisted
        // prefix first (both export prefixes), then split the REMAINDER
        // into exactly 4 fields: pid, tid_hex, nanos, counter.
        // (Chính prefix chứa dấu gạch (`import-file`, `mgc-export`) —
        // split theo '-' sẽ xé chúng. Match prefix dài nhất trong
        // allowlist trước (cả 2 prefix export), rồi split PHẦN CÒN LẠI
        // đúng 4 field: pid, tid_hex, nanos, counter.)
        let prefix = MGC_TEMP_PREFIXES
            .iter()
            .filter(|p| name.starts_with(&format!("{p}-")))
            .max_by_key(|p| p.len())?;
        let rest = &name[prefix.len() + 1..];
        // Split keeping empty fields so `a--b` is rejected, not silently
        // field-skipped.
        // (Split giữ field rỗng để `a--b` bị từ chối, không lướt qua.)
        let parts: Vec<&str> = rest.split('-').collect();
        if parts.len() != 4 {
            return None;
        }
        let pid: u32 = parts[0].parse().ok()?;
        let tid_hex = parts[1];
        if tid_hex.is_empty() || !tid_hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return None;
        }
        let nanos: u128 = parts[2].parse().ok()?;
        let counter: u64 = parts[3].parse().ok()?;
        Some(Self {
            prefix: prefix.to_string(),
            pid,
            tid_hex: tid_hex.to_string(),
            nanos,
            counter,
        })
    }

    /// May the CAS-tmp GC sweep this parsed name? Only temps that
    /// legitimately stage under CAS tmp/ (`import-file`, `write-bytes`)
    /// — and nothing else, even other MGC-prefixed names.
    /// (GC tmp CAS có được quét tên đã parse này? Chỉ temp hợp pháp
    /// staging dưới tmp CAS (`import-file`, `write-bytes`) — không gì
    /// khác, kể cả tên có prefix MGC khác.)
    pub fn cas_tmp_sweepable(&self) -> bool {
        CAS_TMP_SWEEPABLE_PREFIXES.contains(&self.prefix.as_str())
    }
}

// Tests live in tests/ (RULE §5) — see tests/temp_name.rs.
// (Test nằm trong tests/ (RULE §5) — xem tests/temp_name.rs.)
