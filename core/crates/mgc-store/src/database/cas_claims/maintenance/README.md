# CAS claim maintenance

## Responsibility

Owns database-wide live-reference enumeration and feature-gated destructive helpers used by crate tests. Generation-scoped writes remain in `cas_claims/` so the normal claim transaction has a narrow owner.

## Trách nhiệm

Quản lý truy vấn liệt kê reference đang sống trên database và helper xóa phá hủy chỉ bật cho test. Ghi claim theo generation ở owner `cas_claims/` để transaction thường có phạm vi hẹp.

## Boundaries

Do not change SQL, feature gates, result ordering, transaction behavior, visibility, or diagnostics. Existing tests remain at their registered crate paths until the repository-wide test organization phase.

## Ranh giới

Không đổi SQL, feature gate, thứ tự kết quả, transaction, visibility hoặc thông báo lỗi. Test hiện hữu giữ nguyên Cargo path đến phase tổ chức test toàn repository.
