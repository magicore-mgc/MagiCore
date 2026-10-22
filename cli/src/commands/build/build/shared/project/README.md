# Build project selection / Chọn project cho build

## Responsibility / Trách nhiệm

Own project-root discovery, safe regular-manifest presence checks, legacy plain-Rust selection and rejection of non-native compatibility modes. Reads project metadata through `mgc-config` and does not modify manifests.

Sở hữu tìm project root, kiểm tra manifest file thường an toàn, chọn lane Rust thuần legacy và từ chối compatibility mode không native. Đọc metadata qua `mgc-config`, không sửa manifest.

## Errors and tests / Lỗi và test

Filesystem/configuration failures propagate to the build boundary; missing project uses the existing build diagnostic. Tests currently remain in `cli/src/test/build_test.rs`. Typed local error ownership is a tracked follow-up, not claimed by this move-only slice.

Lỗi filesystem/config được propagate tới boundary build; thiếu project dùng diagnostic build hiện tại. Test vẫn nằm ở `cli/src/test/build_test.rs`. Error typed cục bộ là đầu việc tiếp theo, không được tính là hoàn tất trong lượt di chuyển này.
