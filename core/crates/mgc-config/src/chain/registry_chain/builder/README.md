# Registry-chain builder / Bộ dựng registry-chain

## Purpose / Mục đích

This owner builds the ordered registry candidates shared by the legacy and checked APIs. It owns the environment override, project registry ordering, npmrc/default fallback, URL identity rules, and host extraction. It does not load `.npmrc`, validate credentials, or make network requests.

Owner này tạo danh sách registry theo thứ tự dùng chung cho API legacy và checked. Nó quản lý env override, thứ tự registry dự án, fallback npmrc/default, quy tắc nhận diện URL và lấy host. Nó không tự đọc `.npmrc`, kiểm tra credential hay gọi mạng.

## Boundaries / Ranh giới

- `builder.rs` contains the implementation; `mod.rs` only exports the constants and the crate-internal helper.
- `test/builder.rs` contains nearby tests for URL identity behavior. These tests are authored but remain `NOT_RUN` until the repository-wide source-first refactor closes.
- Keep legacy best-effort behavior and checked-path error handling in their respective owners. Do not move secret loading or credential-scope decisions here.
- Preserve `mgc_config::chain` public paths and the URL comparison behavior unless a separately approved behavior change says otherwise.

- `builder.rs` chứa implementation; `mod.rs` chỉ export constants và helper nội bộ crate.
- `test/builder.rs` chứa test gần code cho quy tắc nhận diện URL. Test đã được viết nhưng vẫn `NOT_RUN` đến khi hoàn tất lượt refactor source-first toàn repo.
- Giữ xử lý best-effort legacy và lỗi của checked API tại owner tương ứng. Không chuyển việc đọc secret hoặc quyết định credential scope vào đây.
- Giữ nguyên public path `mgc_config::chain` và hành vi so sánh URL, trừ khi có phê duyệt riêng cho thay đổi hành vi.
