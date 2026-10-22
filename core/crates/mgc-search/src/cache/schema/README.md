# Search cache schema / Schema cache tìm kiếm

This owner creates the existing search_cache and user_choices tables and their indexes.
Owner này tạo hai bảng search_cache, user_choices cùng các index hiện hành.

It does not migrate, prune, vacuum, or alter the schema. Expired-row cleanup and database size policy remain separate work.
Owner này không migrate, dọn, vacuum hoặc đổi schema. Dọn bản ghi hết hạn và chính sách kích thước database là việc riêng.
