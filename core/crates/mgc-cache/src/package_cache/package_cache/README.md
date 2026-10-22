# Package cache state / Trạng thái cache package

Owns cache construction, default-root selection, v2 package identity paths and safe legacy-path eligibility.
Sở hữu khởi tạo cache, chọn root mặc định, path định danh package v2 và điều kiện path cũ an toàn.

V2 keys are full BLAKE3 digests, so package IDs cannot traverse directories or collide through separator replacement. Existing safe legacy entries remain readable until pruned; new stores use v2 only.
Key v2 là digest BLAKE3 đầy đủ nên package ID không thể thoát thư mục hoặc va chạm do thay separator. Entry cũ an toàn vẫn đọc được đến khi prune; store mới chỉ dùng v2.
