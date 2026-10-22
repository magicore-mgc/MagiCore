# Materialized package owner / Owner package đã materialize

Checks installed package identity first. If no source marker is provided, the prior identity-only success remains; with a source marker it requires a readable equal marker and matching extracted content. Optional marker writing remains a no-op when the source marker is absent.

Kiểm tra identity package đã cài trước. Nếu không có source marker thì giữ kết quả identity-only thành công như trước; nếu có thì cần marker đọc được, bằng nhau và nội dung extract khớp. Ghi marker tùy chọn vẫn no-op khi thiếu source marker.

Tests cover the absent-source-marker write path; existing cache/install integration tests cover the broader lifecycle and stay in place. No local error enum is added because errors continue from the shared install/marker APIs unchanged.

Test bao phủ nhánh ghi khi source marker vắng mặt; test tích hợp cache/install cũ tiếp tục bao phủ lifecycle rộng hơn. Không thêm error enum cục bộ vì lỗi tiếp tục truyền nguyên trạng từ API install/marker dùng chung.
