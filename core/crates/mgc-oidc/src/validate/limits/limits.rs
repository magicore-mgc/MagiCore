//! Named OIDC validation/cache limits — các giới hạn OIDC có tên tập trung.
//! Giá trị và cách áp dụng được giữ nguyên từ module trước khi tách.

/// Clock-skew tolerance applied to expiry checks.
/// Khoảng dung sai lệch đồng hồ khi kiểm tra hết hạn.
pub const EXPIRY_LEEWAY_SECS: u64 = 60;
/// Maximum accepted workload JWT lifetime — tuổi thọ tối đa của JWT workload.
pub const MAX_TOKEN_LIFETIME_SECS: u64 = 15 * 60;
/// JWKS cache limits and refresh cadence — giới hạn cache JWKS và chu kỳ refresh.
pub const JWKS_CACHE_TTL_SECS: u64 = 300;
pub const JWKS_REFRESH_INTERVAL_SECS: u64 = 30;
pub const MAX_CACHED_ISSUERS: usize = 32;
/// Maximum encoded JWT accepted by the validation entry points.
/// Kích thước JWT tối đa mà các điểm xác minh chấp nhận.
pub const MAX_JWT_BYTES: usize = 16 * 1024;
