# Platform data directories

## Purpose

Owns `MGC_DATA_DIR` resolution and the managed tool/data roots (`mgc_data_dir`, `global_tools_dir`, `global_bin_dir`, `project_data_dir`). Relative overrides resolve against the home directory; per-project data is keyed by a canonical-path BLAKE3 digest. `../mod.rs` re-exports these names unchanged.

## Phạm vi

Owner này sở hữu việc resolve `MGC_DATA_DIR` và các root tool/dữ liệu do MGC quản lý (`mgc_data_dir`, `global_tools_dir`, `global_bin_dir`, `project_data_dir`). Override tương đối được resolve theo home; dữ liệu từng project khoá bằng BLAKE3 trên path canonical. `../mod.rs` re-export nguyên tên.
