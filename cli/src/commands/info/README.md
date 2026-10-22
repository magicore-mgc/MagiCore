# Package info command / Lệnh xem thông tin package

`mod.rs` preserves the public `commands::info::{run, versions_json}` paths. `info.rs` coordinates registry lookup and output. The `core_label/`, `local_version/`, `output/`, and `error/` owners keep classification, project declaration lookup, presentation/JSON projection, and command errors separate.

`Core Support` is only a keyword heuristic over package name and description. `Installed` currently displays the matching dependency range from the nearest discovered `package.json`; it does not resolve `node_modules`, a lockfile, or another core's installed package. Registry metadata still comes from the Web/npm-compatible adapter, including when the common CLI route is used. The folder does not claim general cross-core registry support.

## Tiếng Việt

`mod.rs` giữ nguyên đường dẫn API `commands::info::{run, versions_json}`. `info.rs` điều phối truy vấn registry và output. Các owner `core_label/`, `local_version/`, `output/`, `error/` tách phân loại, đọc khai báo project, trình bày/JSON và lỗi lệnh.

`Core Support` chỉ là heuristic từ keyword trong tên và mô tả package. Trường `Installed` hiện hiển thị range dependency trong `package.json` gần nhất được tìm thấy; nó không đọc `node_modules`, lockfile hay package đã cài của core khác. Metadata vẫn lấy từ adapter Web/npm-compatible kể cả khi gọi qua CLI chung. Folder này không tuyên bố hỗ trợ registry mọi core.

See [ProgressReport](ProgressReport.md) and [Checklist](Checklist.md) for source and runtime status.
