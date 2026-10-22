# Remove mapping owner

This owner converts normalized remove input into one of the nine `CoreCommand::Remove*` variants. It preserves the Web-only reinstall field, package list and compatibility-runtime value; it does not choose policy or execute package operations.

Owner này đổi input remove đã chuẩn hóa thành một trong chín variant `CoreCommand::Remove*`. Nó giữ field cài lại chỉ có ở Web, danh sách package và giá trị compatibility-runtime; không chọn policy hoặc thực thi package operation.

The nearest tests live under `test/` and cover exact core names, all nine output variants, package/compatibility forwarding and Web's reinstall field.

Test gần owner nằm trong `test/`, bao phủ tên core chính xác, đủ chín output variant, chuyển tiếp package/compatibility và field cài lại riêng của Web.
