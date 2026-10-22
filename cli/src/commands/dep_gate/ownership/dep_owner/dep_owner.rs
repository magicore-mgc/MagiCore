//! Lifecycle ownership result — kết quả quyền sở hữu lifecycle.
//! Separates native lifecycle, real scaffold inventory, and unsupported operations.
//! Tách lifecycle native, inventory scaffold thật, và operation chưa hỗ trợ.

/// Ownership of one (core, ecosystem, manifest format, operation) cell —
/// the ONLY source for gate decisions.
/// Capability matrices and docs must derive from this table, never
/// hardcode their own copy.
/// Quyền sở hữu của một ô (core, ecosystem, manifest format, operation) —
/// nguồn DUY NHẤT cho quyết định gate. Matrix capability và docs phải suy
/// ra từ bảng này, không hardcode bản sao riêng.
pub enum DepOwner {
    /// MGC owns the full lifecycle — proceeds in native mode; compat is still refused.
    /// MGC thực thi lifecycle ở chế độ native; cờ compat vẫn bị từ chối.
    Native,
    /// No package lifecycle, but a REAL read-only inventory command
    /// exists (hardware list) — passes the gate in native mode, reported
    /// as "scaffold-only" (never "mgc-native") in capabilities. The outer
    /// gate still rejects every explicit compatibility value.
    /// Đây chỉ là inventory, không phải lifecycle package; compat luôn bị chặn.
    ScaffoldOnly,
    /// No lifecycle at all — always fails closed, even under compat.
    /// Không có lifecycle; luôn fail-closed, kể cả khi truyền compat.
    Unsupported,
}
