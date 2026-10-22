# process_supervision

This owner coordinates Web dev lifecycle hooks, single/multi-target execution, cancellation/reaping and guarded MGC child launch. `mod.rs` only wires and re-exports same-name owners. Error values retain the existing `anyhow::Result` boundary because failures are heterogeneous resolver/process/runtime errors; wrapping them in a new local enum could change existing display and source-chain behavior. Existing postdev-hook failure remains a warning by design.

# process_supervision — trách nhiệm

Owner này điều phối lifecycle hook Web dev, chạy một/nhiều target, hủy/thu hồi task và khởi chạy process con có guard. `mod.rs` chỉ nối và export các owner cùng tên. Lỗi tiếp tục dùng boundary `anyhow::Result` vì gồm nhiều loại resolver/process/runtime; bọc bằng enum mới có thể đổi display và source chain hiện hành. Lỗi postdev vẫn được cảnh báo best-effort như behavior cũ.
