Process shutdown owner. `mod.rs` exports the stable request API; `shutdown.rs` owns the process-wide flag and OS handler installation; `error.rs` classifies registration failures with native codes/source chains; `test/` covers the local error contract; `docs/` tracks this owner.

Owner dừng process. `mod.rs` export API request ổn định; `shutdown.rs` sở hữu cờ toàn process và cài handler theo OS; `error.rs` phân loại lỗi đăng ký, giữ mã native/source; `test/` kiểm tra hợp đồng lỗi cục bộ; `docs/` theo dõi owner.
