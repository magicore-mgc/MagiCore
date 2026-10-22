# Open Blob Owner

**Purpose:** Verify the already-open CAS file, retain the existing bounded memo policy, and return the same handle at offset zero for export staging.

**Mục đích:** Xác minh file CAS đã mở, giữ chính sách memo có giới hạn và trả lại cùng handle ở offset 0 để export staging.

The handle identity, rehash, memo lookup and subsequent export all refer to the same opened object. A memo hit still returns the opened handle; the stage digest remains the final export integrity gate. This owner does not close parent-path, in-place mutation, or staged-temp races.

Identity, rehash, memo và export tiếp theo cùng trỏ tới một object đã mở. Memo hit vẫn trả handle đó; digest của file stage là cổng integrity cuối cho export. Owner này chưa xử lý race ở thư mục cha, sửa nội dung tại chỗ hoặc pathname của file stage.
