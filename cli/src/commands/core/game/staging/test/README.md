# Staging tests / Test staging

This folder tests link refusal for both `.magicore` and `tmp-optimizer`, stale staging replacement, and removal of a newly created empty `.magicore` parent. The tests assert that linked targets remain untouched.

Folder này kiểm tra từ chối link ở cả `.magicore` và `tmp-optimizer`, thay staging cũ, và xóa parent `.magicore` rỗng vừa tạo. Test xác nhận dữ liệu ở target được link không bị đụng tới.

These are source-only assertions for the current refactor pass; execution is deferred until repository source closure and the storage constraint allow it.

Đây mới là assertion trong source; chưa chạy đến khi đóng source toàn repo và điều kiện dung lượng cho phép.
