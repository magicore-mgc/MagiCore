# Workspace install entry / Điểm vào install workspace

This owner preserves the crate-visible `install_into_root_ws` API for mixed-core monorepo callers and delegates to the transactional per-project owner. It does not add an independent error type or alter error propagation. The nearest suite currently has no wrapper-specific behavior case; keep that coverage gap for the later test phase.

Owner giữ API `install_into_root_ws` cho caller monorepo đa core và chuyển tiếp tới transaction từng project. Không tạo error type riêng hoặc đổi đường truyền lỗi. Test wrapper còn thiếu, ghi nhận cho phase test sau.
