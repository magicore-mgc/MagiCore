# Install mutation / Giao dịch install

`mutation.rs` owns the per-project mutation after offline and owner preflight: mutation gateway, manifest staging/write, journal, rollback, lock replay and result completion. Offline lock/trust/cache checks live in [`offline_guard`](offline_guard/README.md); package-count and core ownership checks live in the validation owner. The current nearest unit suite has no install transaction test; record this as coverage debt for the later test phase.

`mutation.rs` quản lý mutation từng project sau offline/owner preflight: gateway, ghi manifest, journal, rollback và hoàn tất. Offline policy và kiểm tra ownership nằm ở owner riêng. Bộ test hiện tại chưa có test transaction; ghi nhận cho phase test sau.
