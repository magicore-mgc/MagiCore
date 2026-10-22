// Compiled-module cache keyed by a VERSIONED compilation key (P0-1/P0-2/P0-3
// audit vòng-4). The old design keyed entries by the source content hash
// only — the same bytes compiled as .ts vs .tsx, or by two compiler
// versions, collided onto one entry and could serve WRONG JavaScript to
// another project. The new design:
//
// P0-1 — `CompilationKey` carries the FULL compilation context: schema
// version, source digest, loader, compiler identity+version, and a digest of
// the canonical options JSON. The cache path is derived from the BLAKE3 of
// the canonical key JSON, so every distinct compilation context gets a
// distinct entry.
//
// P0-2 — race contract is now DETERMINISTIC on every OS: publication uses
// `hard_link` (fails with AlreadyExists when the entry exists on both Unix
// and Windows — first writer wins, no rename-overwrite last-wins). A loser
// NEVER silently reports success: it reads the winner, verifies it, and
// either dedups (identical output digest → Ok), replaces it (stale/corrupt
// winner → quarantine/remove + retry link once), or fails with a typed
// `CacheConflict` (divergent output for the same key is nondeterministic
// compilation or tampering — never faked Ok).
//
// P0-3 — records are SELF-VALIDATING: every stored JSON embeds the
// schema version, the full compilation key, and the BLAKE3 digest of the
// output. `get()` re-verifies all three BEFORE serving; a well-formed JSON
// whose payload was swapped ("poisoned") fails the digest check, is moved to
// `quarantine/` for forensics, and returns a hard `CacheCorrupt` error —
// poison is never served to the dev server.
// THREAT MODEL (P0-4, Tech Lead vòng-6/7): this cache is self-validating
// against ACCIDENTAL CORRUPTION and INCOMPLETE WRITES only. It is NOT
// authenticated against a malicious writer running as the SAME user: such
// a process can already rewrite binaries, configs and project sources, and
// can recompute every digest in a forged record. Remote/untrusted caches
// would need signatures, MAC keys or provenance by namespace — out of
// scope for the local store by design.
//
// Cache module đã biên dịch, khóa theo compilation key CÓ PHIÊN BẢN (P0-1/
// P0-2/P0-3 audit vòng-4). Thiết kế cũ chỉ khóa theo hash nội dung source —
// cùng bytes compile theo .ts với .tsx, hoặc bằng 2 phiên bản compiler, bị
// dồn vào 1 entry và có thể phục vụ JavaScript SAI cho project khác. Thiết
// kế mới:
//
// P0-1 — `CompilationKey` mang ĐẦY ĐỦ ngữ cảnh biên dịch: schema version,
// digest source, loader, identity+version compiler, và digest của JSON
// options chuẩn hóa. Path cache suy ra từ BLAKE3 của JSON key chuẩn hóa —
// mọi ngữ cảnh biên dịch khác nhau có entry khác nhau.
//
// P0-2 — hợp đồng race giờ TẤT ĐỊNH trên mọi OS: publish dùng `hard_link`
// (thất bại AlreadyExists khi entry tồn tại trên cả Unix lẫn Windows —
// writer đầu thắng, không có last-wins bằng rename-đè). Bên thua KHÔNG BAO
// GIỜ báo thành công im lặng: đọc winner, verify winner, rồi hoặc dedup
// (digest output giống nhau → Ok), hoặc thay thế (winner stale/hỏng →
// quarantine/remove + retry link đúng 1 lần), hoặc fail với lỗi typed
// `CacheConflict` (output phân kỳ cùng key là biên dịch không tất định
// hoặc bị can thiệp — không bao giờ giả Ok).
//
// P0-3 — record TỰ KIỂM TRA VỚI CORRUPTION NGẪU NHIÊN (self-validating,
// KHÔNG phải self-authenticating): mọi JSON lưu nhúng schema version,
// compilation key đầy đủ, và digest BLAKE3 của output. `get()` verify lại
// cả ba TRƯỚC khi phục vụ; JSON hợp lệ nhưng payload bị đổi ("đầu độc")
// rơi khỏi check digest, bị chuyển vào `quarantine/` để điều tra, và trả
// lỗi cứng `CacheCorrupt` — độc không bao giờ tới dev server.
// THREAT MODEL (P0-4): cache này tự KIỂM TRA (self-validating, chống hỏng
// ngẫu nhiên + ghi đứt giữa chừng), KHÔNG TỰ XÁC THỰC (self-authenticating)
// chống writer ác ý cùng user — process đó đã có thể sửa binary, config,
// source project, và recompute mọi digest trong record giả mạo. Cache
// remote/không tin cậy cần chữ ký, khóa MAC hay provenance theo namespace —
// ngoài scope của store local theo thiết kế.

/// Current on-disk record schema. Bump when the record layout changes —
/// entries written by an older schema are treated as STALE (cache miss →
/// recompile → replace), never served.
///
/// v2 (P1-1, Tech Lead vòng-5/6/7): output digest switched from the
/// ambiguous `js || 0x00 || sm` concatenation (where `None ≡ Some("")` and
/// NUL bytes inside payloads created collisions) to a domain-separated
/// length-prefixed encoding.
///
/// Schema record trên đĩa hiện tại. Tăng khi layout record đổi — entry ghi
/// bởi schema cũ được coi là STALE (cache miss → biên dịch lại → thay),
/// không bao giờ phục vụ.
///
/// v2 (P1-1): digest output đổi từ nối chuỗi mơ hồ `js || 0x00 || sm`
/// (trong đó `None ≡ Some("")` và byte NUL trong payload tạo collision)
/// sang mã hóa length-prefixed tách miền.
pub const COMPILED_CACHE_SCHEMA_VERSION: u32 = 2;

mod compilation_key;
mod compiled_cache;
mod compiled_module;
mod loader;
mod output_digest;
mod publication;
mod record;
use record::{CompiledRecord, VerifyOutcome};
mod temp_guard;

pub use compilation_key::CompilationKey;
pub use compiled_cache::CompiledCache;
pub use compiled_module::CompiledModule;
pub use loader::Loader;
