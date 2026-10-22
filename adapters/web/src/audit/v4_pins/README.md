# v4 audit pin selection / Chọn pin audit v4

This owner reads only a verified v4 `mgc.lock`, resolves the Web owner roots, validates full package identities and edge targets, traverses only reachable packages, and splits npm-auditable pins from JSR partial coverage. Missing/legacy locks return `None` to the legacy path; malformed v4 input remains an error.

Owner này chỉ đọc `mgc.lock` v4 đã verify, xác định root của Web, kiểm tra full package identity và edge target, duyệt package reachable, rồi tách pin npm-auditable khỏi phần JSR partial. Lock thiếu/legacy trả `None` cho legacy path; v4 sai vẫn trả lỗi.

`types::AuditPinSplit` is shared with the audit coordinator. Tests under `test/` preserve root, reachability, tamper, full-edge, and legacy behavior.

`types::AuditPinSplit` dùng chung với coordinator audit. Test trong `test/` giữ behavior root, reachability, tamper, full-edge và legacy.
