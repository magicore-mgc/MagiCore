# release_candidates / release_candidates

Loads the PyPI project JSON, filters releases by the requested PEP 440 range, orders newest first, and applies the existing minimum-release-age gate.

Tải project JSON PyPI, lọc release theo PEP 440, sắp xếp mới trước và áp dụng minimum-release-age gate hiện có.

The owner preserves parsing and age-gate diagnostics, exclusion counts, registry request path, and the original candidate ordering. It owns no HTTP policy beyond this existing metadata request.

Owner giữ nguyên diagnostic parse/age-gate, số lượng release bị loại, URL request registry và thứ tự candidate. Owner không tự đặt HTTP policy ngoài request metadata hiện có.
