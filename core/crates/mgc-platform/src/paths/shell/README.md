# Shell PATH integration

Owns persistent installation of MGC's managed executable directory. `mod.rs` only wires and exports `shell/shell.rs`; `ensure_global_bin_on_path` and `ensure_bin_on_path` remain re-exported through `mgc_platform::paths`. Unix profile handling and Windows user environment updates are separate OS owners. This module does not install MGC binaries or modify unrelated PATH entries.

## Vietnamese

Owner này quản lý việc thêm thư mục executable do MGC quản lý vào PATH. `mod.rs` chỉ wiring và export `shell/shell.rs`; hai API `ensure_global_bin_on_path` và `ensure_bin_on_path` vẫn được export qua `mgc_platform::paths`. Xử lý profile Unix và môi trường user Windows nằm ở owner OS riêng. Module này không cài binary MGC và không sửa PATH ngoài entry do MGC quản lý.
