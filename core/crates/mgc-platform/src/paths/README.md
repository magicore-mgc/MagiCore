# Platform paths

Owns global/project data roots and project-root discovery. `mod.rs` only wires and re-exports four owners: `data_dir/` (MGC_DATA_DIR + managed tool roots), `global_paths/` (machine-wide layout), `project_paths/` (`.magicore/` layout) and `project_root/` (marker walk-up). Shell PATH persistence is delegated to `shell/`; the old `mgc_platform::paths` API remains compatible through re-exports. Global data-root semantics were not changed by this extraction.

## Vietnamese

Owner này quản lý data root toàn cục/project và phát hiện root project. `mod.rs` chỉ wiring và re-export bốn owner: `data_dir/` (`MGC_DATA_DIR` + root tool), `global_paths/` (layout toàn máy), `project_paths/` (layout `.magicore/`) và `project_root/` (đi ngược marker). Lưu PATH của shell nằm ở `shell/`; API `mgc_platform::paths` cũ vẫn tương thích qua re-export. Ngữ nghĩa data-root toàn cục không đổi.
