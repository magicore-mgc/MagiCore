# Capability errors / Lỗi capability

This owner represents capability-specific failure cases as typed errors. `UnknownCore` replaces the previous generic error constructor at this module boundary while retaining its exact display string and the existing public `anyhow::Result` signatures.

Owner này biểu diễn lỗi riêng của capability bằng type. `UnknownCore` thay constructor lỗi tổng quát tại boundary module nhưng giữ nguyên câu `Display` và signature public `anyhow::Result` hiện có.
