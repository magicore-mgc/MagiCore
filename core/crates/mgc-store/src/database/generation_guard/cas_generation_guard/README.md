# CAS generation guard

Owns the public RAII token guard, drop-time abort behavior, disarm operation, and generation accessor.

## Trách nhiệm

Quản lý guard token RAII công khai, hành vi abort khi drop, thao tác disarm và accessor generation.

The parent `generation_guard/` module re-exports `CasGenerationGuard` and owns construction from `Database`. Preserve its public path and the warning-only drop failure behavior.

Module cha `generation_guard/` re-export `CasGenerationGuard` và sở hữu constructor từ `Database`. Giữ nguyên public path và hành vi chỉ cảnh báo khi drop cleanup thất bại.
