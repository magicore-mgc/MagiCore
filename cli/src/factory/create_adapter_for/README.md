# Root-bound adapter construction

Owns project-root adapter selection for every `Ecosystem`, including feature-disabled errors, explicit global plugin precedence, registry chain forwarding, fallbacks, detection failures, and project-bridge registration. Adapter errors continue upward without being wrapped or converted.

# Dựng adapter gắn với project root

Sở hữu chọn adapter theo mọi `Ecosystem`, gồm lỗi core không được build, ưu tiên plugin global tường minh, chuyển tiếp registry chain/fallback, lỗi nhận diện và đăng ký bridge project. Lỗi adapter tiếp tục được truyền lên, không bọc hoặc đổi thành success.
