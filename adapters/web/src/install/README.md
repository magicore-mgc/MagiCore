# Web install subsystem / Hệ thống install Web

This directory wires install owners for fetching, integrity, archive extraction, materialization, lock markers and lifecycle policy. Network access, verification, filesystem publication and process execution remain separate concerns with local tests.

Thư mục này nối các owner install cho fetch, integrity, giải nén archive, materialization, lock marker và policy lifecycle. Truy cập network, xác minh, ghi filesystem và chạy process là các concern riêng với test tại chỗ.

Tarball integrity is owned by `integrity/`: byte-buffer and file-reader inputs share one SRI policy, and reader hashing uses bounded memory. The compatibility byte API remains available through this owner.

Integrity tarball thuộc `integrity/`: input byte-buffer và file-reader dùng chung một chính sách SRI, còn hashing reader có bộ nhớ giới hạn. API byte tương thích vẫn được giữ qua owner này.


Download orchestration is owned by `download/`: its facade preserves `install::download::*`, while `prefetch/`, `dependency_pipeline/`, `task_concurrency/`, and local error/aggregation owners hold behavior. Cache/network/integrity integration remains on the native Web install route; the facade is not a standalone package-manager command.

Điều phối download thuộc `download/`: facade giữ `install::download::*`; các owner `prefetch/`, `dependency_pipeline/`, `task_concurrency/`, aggregation và error cục bộ chứa từng trách nhiệm. Cache/network/integrity vẫn chạy qua native Web install; facade không phải lệnh package-manager độc lập.
