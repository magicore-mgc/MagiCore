/// Machine-readable deny reason — mã lý do từ chối đọc được bằng máy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DenyCode {
    /// Mutation verb in native mode (decision table row 4).
    /// Động từ mutation ở native mode (hàng 4 decision table).
    NativeMutationDeny,
    /// Profiled lane, PM invocation outside the active profile (row 6).
    /// Lane đã profile, invocation PM ngoài profile hiệu lực (hàng 6).
    OutsideActiveProfile,
    /// Unprofiled lane spawning a forbidden PM without an exact exception.
    /// Lane chưa profile spawn PM cấm mà không có exception chính xác.
    ForbiddenNameNoException,
    /// No exact read-only exception covers this invocation (row 5).
    /// Không có exception đọc nào khớp invocation này (hàng 5).
    MissingException,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
