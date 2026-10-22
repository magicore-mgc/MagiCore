# Deploy command owner

Owns the shared `deploy` route's core selection. It preserves CICD deployment when that lane is compiled, reports the existing not-in-build error when CICD is requested without its feature, and otherwise routes to the Cloud lane as before. The shared project detector remains the authority.

## Tiếng Việt

Owner chọn core cho route `deploy` dùng chung. Owner giữ route CICD khi feature được compile, trả lỗi not-in-build hiện có nếu user yêu cầu CICD nhưng thiếu feature, và các trường hợp còn lại đi vào Cloud như trước. Bộ nhận diện project dùng chung vẫn là nguồn quyết định.
