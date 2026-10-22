# Resolve and materialize / Resolve và cài package

`materialize.rs` owns lock replay/fresh resolution, age-gate arming, adapter materialization, progress/summary output and post-install hooks. It is called inside the mutation transaction so tail errors remain rollback-routed. It uses the shared lock owner and `anyhow` errors. This nearest source contains no materialization behavior test; track coverage for the later test phase.

`materialize.rs` quản lý phát lại lock/resolve mới, age gate, cài package, summary và post-install hooks. Hàm chạy bên trong transaction để lỗi cuối vẫn rollback. Chưa có test hành vi materialize ở bộ test này; ghi nợ coverage cho phase sau.
