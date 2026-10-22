# App manifest owners

EN: This module dispatches App manifest parsing and writing to format-specific owners. `mod.rs` declares the existing Flutter, Gradle, React Native and Swift modules and reexports stable public paths. `manifest.rs` coordinates the API; `parse_manifest/` and `write_manifest/` own language dispatch, Multi fallback and unsupported diagnostics.

VI: Module này điều phối parse/ghi manifest App đến owner theo định dạng. `mod.rs` khai báo các module Flutter, Gradle, React Native, Swift và reexport đường dẫn công khai ổn định. `manifest.rs` điều phối API; `parse_manifest/` và `write_manifest/` sở hữu dispatch language, fallback Multi và diagnostics unsupported.

## Compatibility and limits

The Multi probe order remains `pubspec.yaml` → Gradle files → `Package.swift` → `package.json`; checks still use `Path::exists()`. Existing unsupported language results and exact diagnostics remain. This refactor does not add manifest support for MAUI/Tauri and does not change format-owner behavior. Existing integration tests stay at their current paths; nearest-owner test placement is deferred until the whole-source pass closes.

Thứ tự dò Multi vẫn là `pubspec.yaml` → file Gradle → `Package.swift` → `package.json`; phép kiểm tra tiếp tục dùng `Path::exists()`. Kết quả unsupported và diagnostics cũ được giữ nguyên. Refactor này không thêm hỗ trợ manifest MAUI/Tauri và không đổi hành vi format owner. Integration tests giữ nguyên path; test gần owner chờ đến khi source pass toàn repo hoàn tất.
