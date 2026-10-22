# Trust pending scan lock owner

`scan_lock.rs` acquires the project writer lock and verifies that no pending removal journal blocks a consistent read. The RAII guard remains alive through the command report. Lock policy itself remains owned by `mgc-lockfile`.

The nearest test verifies refusal while another writer owns the lock. Runtime execution is deferred.

# Owner khóa quét trust pending

`scan_lock.rs` lấy project writer lock và xác nhận không có pending removal journal cản trở snapshot đọc nhất quán. Guard RAII được giữ đến hết báo cáo. Policy khóa thuộc `mgc-lockfile`.

Test gần owner kiểm tra từ chối khi writer khác giữ khóa. Runtime chưa chạy.
