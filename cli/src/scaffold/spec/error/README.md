# error

Owns the private typed errors shared by the spec parser and reference validator. Each error is rendered into the previous message-only `anyhow` representation, preserving display text and source-chain shape; public functions continue returning `anyhow::Result`.

Quản lý lỗi typed private dùng chung giữa parser spec và validator reference. Mỗi lỗi được chuyển về dạng message-only `anyhow` trước đây để giữ display text và hình dạng source chain; hàm public tiếp tục trả `anyhow::Result`.
