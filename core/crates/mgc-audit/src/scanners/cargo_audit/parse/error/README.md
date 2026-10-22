# cargo-audit parser errors

## Purpose

Owns typed internal parse failures, source-chains serde errors, and renders legacy `MgError::Other` text at the public boundary.

## Phạm vi

Sở hữu lỗi parse typed nội bộ, giữ source của lỗi serde và render nguyên văn `MgError::Other` cũ ở biên public.

No retry policy is added. Recovery stays with the parser caller; the public `MgResult` contract is preserved.
