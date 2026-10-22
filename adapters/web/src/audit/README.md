# Web audit / Audit Web

This directory owns Web audit entry points and preserves the public `crate::audit::*` surface. `audit.rs` is the stable coordinator; `pin_source/` selects verified v4 or checked legacy pins, `advisory_query/` owns the npm HTTP/error path, and `report/` projects clean/Partial/failed coverage; `v4_pins/` validates and selects reachable Web pins from a v4 lock; `audit_fix/` writes a revised manifest and lock only after resolution succeeds. `types.rs` contains the crate-private pin tuple shared by those owners and the crate-visible pin helper signature.

Folder này sở hữu entry point audit Web và giữ nguyên API `crate::audit::*`. `audit.rs` là coordinator ổn định; `pin_source/` chọn pin v4 đã verify hoặc legacy đã kiểm tra, `advisory_query/` sở hữu HTTP/error npm, và `report/` tạo trạng thái clean/Partial/failed; `v4_pins/` xác thực/chọn pin Web reachable từ lock v4; `audit_fix/` chỉ ghi manifest và lock sau khi resolve thành công. `types.rs` chứa tuple pin nội bộ crate dùng chung giữa các owner và chữ ký helper pin có visibility cấp crate.

The lockfile remains the sole audit source. Rival lockfiles are remediation inputs only; v4 digest, full edge identity, and root reachability stay fail-closed. Registry failure remains a scanner failure/UNVERIFIED result, malformed advisory data remains an error, and JSR-only coverage remains Partial.

Lockfile vẫn là nguồn audit duy nhất. Lockfile đối thủ chỉ là input remediation; digest v4, full edge identity và reachability root giữ fail-closed. Registry lỗi vẫn là scanner failure/UNVERIFIED, dữ liệu advisory sai vẫn trả lỗi, còn chỉ có JSR vẫn là Partial.
