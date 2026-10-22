# Core catalog / Catalog core

This owner defines the canonical nine CLI core names and maps each name to the adapter's static `CAPABILITIES` declaration only when that adapter feature is compiled. It does not construct adapters, detect projects or validate whether a declared lifecycle works on a host.

Owner này định nghĩa chín tên core CLI chuẩn và chỉ ánh xạ sang `CAPABILITIES` tĩnh của adapter khi feature tương ứng được compile. Nó không dựng adapter, detect project hay xác nhận lifecycle khai báo có chạy trên máy hay không.

An unknown or feature-disabled selected name returns the local typed error with the legacy display text. The all-core coordinator catches that error only to omit adapters absent from the current binary, preserving the existing fail-closed matrix behavior.

Tên được chọn nhưng không biết hoặc adapter bị tắt feature trả lỗi typed cục bộ với câu chữ cũ. Coordinator all-core chỉ bắt lỗi đó để bỏ adapter không có trong binary hiện tại, giữ nguyên hành vi fail-closed của matrix.
