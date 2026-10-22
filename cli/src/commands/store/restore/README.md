# Restore

## Responsibility

This directory is the coordinator and wiring boundary for safe project Web-store restore. It validates the selected backup before taking the project lock, preserves current data, then applies only the backup components that exist. A database-only backup leaves the current virtual store untouched; a vstore-only backup leaves the current database untouched.

## Trách nhiệm

Thư mục này điều phối và nối các owner restore Web-store an toàn theo project. Backup được xác thực trước khi lấy khóa project; sau đó state hiện tại được bảo toàn rồi chỉ áp dụng các thành phần thực sự có trong backup. Backup chỉ có database sẽ giữ nguyên virtual store hiện tại; backup chỉ có vstore sẽ giữ nguyên database hiện tại.

Project-state capture and safety-destination planning now live in `project_state/` and `safety_destination/`; backup validation, database restore, preservation, and virtual-store restore remain in their existing owners. Each behavior owner has nearest tests and a bilingual README. `RestoreError` is shared by the coordinator/state/safety phase because those diagnostics form one CLI boundary; low-level copy/validation owners retain their narrower typed errors. `mod.rs` only declares owners and exports the CLI entry point. On a database or virtual-store restore error after preservation succeeds, the diagnostic includes the safety-backup path so the user can restore the pre-change snapshot. Restore is not an atomic multi-component transaction: an I/O failure may leave one component restored and another unchanged/partial. The path-based filesystem boundary also has a documented concurrent replacement race; source tests are not runtime evidence until executed.

Chụp project state và lập safety destination nay nằm trong `project_state/` và `safety_destination/`; validation backup, restore database, preserve state và restore virtual store vẫn ở owner cũ. Mỗi owner hành vi có test gần nhất và README song ngữ. `RestoreError` được chia sẻ ở coordinator/state/safety vì các diagnostic cùng thuộc một biên CLI; owner copy/validation mức thấp giữ error typed hẹp hơn. `mod.rs` chỉ khai báo owner và export điểm vào CLI. Nếu restore DB/vstore lỗi sau khi đã bảo toàn state, diagnostic hiện safety-backup path để người dùng khôi phục snapshot trước thay đổi. Restore không phải transaction nhiều thành phần nguyên tử: lỗi I/O có thể khiến một phần đã restore còn phần khác chưa đổi/dở dang. Biên filesystem dùng path còn race thay thế đồng thời đã ghi nhận; test source chưa phải bằng chứng runtime trước khi chạy.
