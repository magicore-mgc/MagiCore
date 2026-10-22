# Unix process termination / Dừng process Unix

`mod.rs` keeps the Unix facade. `termination.rs` coordinates the bounded TERM/grace/KILL loops and aggregates sweep failures. `process_signal_plan/` owns the root/group target policy and its local value type; `process_group_probe/` owns PID-to-PGID identity, liveness and descendant observation; `process_group_signal/` rechecks groups and performs signal syscalls. `error.rs` retains typed diagnostics and sources. Each behavior owner has nearest tests and a bilingual README.

The 100-ms grace window, PID reservation rule, ESRCH/EPERM behavior, per-target liveness recheck, root-reaped handling and bounded error retention are contract-sensitive. Windows Job Object lifecycle is maintained by the separate `windows_job/` owner. Real child lifecycle cases remain in runner/process-scan integration tests.

`mod.rs` giữ facade Unix. `termination.rs` điều phối các vòng TERM/grace/KILL có giới hạn và tổng hợp lỗi. `process_signal_plan/` sở hữu policy chọn root/group và value type local; `process_group_probe/` sở hữu identity PID→PGID, liveness và quan sát descendant; `process_group_signal/` kiểm tra lại group rồi gọi syscall signal. `error.rs` giữ diagnostics typed và source. Mỗi behavior owner có test gần nhất và README song ngữ.

The bounded Unix sweep carries the same root `ExecutionScope` into its repeated descendant scans, preserving DeviceControl classification while TERM/KILL cleanup is in progress.

Vòng Unix có giới hạn tiếp tục truyền cùng `ExecutionScope` của root vào các lượt scan descendant lặp lại, để phân loại DeviceControl không bị mất trong lúc dọn TERM/KILL.

Grace 100 ms, quy tắc giữ PID root, xử lý ESRCH/EPERM, probe liveness từng đích, trạng thái root reaped và giới hạn giữ lỗi là contract nhạy cảm. Lifecycle Windows Job Object thuộc owner `windows_job/` riêng. Case cleanup process thật tiếp tục nằm trong integration test runner/process-scan.
