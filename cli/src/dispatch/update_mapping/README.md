# Update mapping owner

This owner converts normalized update input into one of the nine `CoreCommand::Update*` variants. It forwards the package list, install choice and compatibility-runtime value without policy or execution decisions.

Owner này đổi input update đã chuẩn hóa thành một trong chín variant `CoreCommand::Update*`. Nó chuyển tiếp danh sách package, lựa chọn install và giá trị compatibility-runtime mà không quyết định policy hoặc thực thi.

Nearest tests cover exact core names, all nine variants, true/false install choices and present/absent compatibility values.

Test gần owner bao phủ tên core chính xác, đủ chín variant, lựa chọn install đúng/sai và giá trị compat có/không.
