// Wires the quantize owner; behavior lives in quantize.rs.
// Nối owner quantize; xử lý nằm trong quantize.rs.
mod error;
mod quantize;
pub(super) use self::error::QuantizeError;
pub(super) use self::quantize::quantize;
