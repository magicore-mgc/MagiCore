//! Shared Bun/Deno audit result types — kiểu kết quả audit Bun/Deno dùng chung.
//! This file owns the npm pins and explicitly skipped JSR identifiers.
//! File này sở hữu npm pin và identifier JSR được ghi rõ là bỏ qua.

use crate::scanners::bun_deno::NpmPin;

/// A JSR-only dependency that cannot ride the npm advisory pipeline.
/// Dependency chỉ có JSR nên không thể đi qua pipeline advisory npm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JrSkipped {
    pub specifier: String,
}

/// Result of reading a Bun/Deno lockfile: audit-ready npm pins plus the
/// honestly-skipped JSR set.
/// Kết quả đọc lockfile Bun/Deno: ghim npm sẵn sàng audit và tập JSR bị
/// bỏ qua có ghi nhận.
#[derive(Debug, Clone, Default)]
pub struct BunDenoRead {
    pub npm_pins: Vec<NpmPin>,
    pub jsr_skipped: Vec<JrSkipped>,
}
