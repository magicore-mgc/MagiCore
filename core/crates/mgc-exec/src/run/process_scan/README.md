# Process scan ownership

`types/` defines shared process identity state. `monitor/` owns Unix process-tree identity refresh, descendant traversal, process-group tracking and policy auditing; its typed root-visibility error is re-exported to supervision. `linux/` reads `/proc/<pid>/stat`; `macos/` reads BSD identity and enumerates PIDs; `windows/` owns PID/creation-time ancestry and bounded scan retries over Job Object snapshots. `mod.rs` is the internal API facade. Tests live beside the responsible owner.

# Ownership process scan

`types/` định nghĩa trạng thái định danh process dùng chung. `monitor/` sở hữu refresh identity cây process Unix, duyệt descendant, theo dõi process-group và audit policy; lỗi root visibility typed được re-export cho supervision. `linux/` đọc `/proc/<pid>/stat`; `macos/` đọc identity BSD và liệt kê PID; `windows/` sở hữu ancestry PID/thời điểm tạo và retry scan có giới hạn trên snapshot Job Object. `mod.rs` là facade API nội bộ. Test nằm cạnh owner phụ trách.
