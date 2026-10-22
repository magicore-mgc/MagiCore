# App language detection

EN: This owner detects the App adapter language and classifies App project manifests. `mod.rs` keeps the stable internal/public exports; `language.rs` coordinates explicit config and ordered framework markers. Child owners hold language identity, explicit config, marker checks, MAUI discovery, regular-file validation and `manifest_is_app`.

VI: Owner này nhận diện language cho App adapter và phân loại manifest project App. `mod.rs` giữ export ổn định; `language.rs` điều phối config tường minh và thứ tự marker framework. Owner con giữ định danh language, config, marker, tìm MAUI, kiểm tra file thường và `manifest_is_app`.

## Detection contract

A parseable explicit `[app].language` takes precedence. If the key is absent or not a string, marker detection runs. If the key is a string but unknown, detection returns `None` without marker fallback. Fallback order is Flutter → Kotlin → Swift → Tauri → MAUI → React Native → Objective-C. Tauri requires both regular files; MAUI requires exactly one regular `.csproj` with the existing marker. Symlink/special-file markers remain rejected.

`AppLanguage::as_str()` and `ecosystem()` remain stable; the latter is a dependency-gate identity, including `rn`, `dotnet`, and `rust` mappings. This refactor does not add support for any lane or alter adapter capabilities.

## Error and test ownership

Detection helpers keep their current `Option`/`bool` contracts: absence, malformed config, unreadable markers and ambiguous MAUI projects remain non-matches rather than new public errors. No empty error module is added. Existing integration tests stay in place; nearest-owner test placement is deferred until the whole-project source phase closes, as requested. No runtime check is implied by this source refactor.

Config parseable khai báo `[app].language` được ưu tiên. Nếu key thiếu hoặc không phải chuỗi, chạy marker detection. Nếu key là chuỗi nhưng không biết, trả `None` và không fallback marker. Thứ tự fallback: Flutter → Kotlin → Swift → Tauri → MAUI → React Native → Objective-C. Tauri yêu cầu hai file thường; MAUI yêu cầu đúng một `.csproj` thường có marker hiện tại. Symlink/file đặc biệt tiếp tục bị từ chối.

`AppLanguage::as_str()` và `ecosystem()` giữ nguyên; `ecosystem()` là định danh dependency-gate, gồm mapping `rn`, `dotnet`, `rust`. Refactor này không thêm lane support hoặc đổi capability adapter.

Các helper detection giữ contract `Option`/`bool`: marker thiếu, config lỗi, marker không đọc được hoặc MAUI mơ hồ tiếp tục là không khớp thay vì tạo lỗi public mới. Không tạo error module rỗng. Integration tests giữ nguyên; test gần owner chờ đến khi source pass toàn repo hoàn tất theo yêu cầu. Refactor source này không hàm ý đã runtime-check.
