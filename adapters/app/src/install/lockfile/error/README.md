# App lock writer errors / Lỗi bộ ghi lock App

Owns stable codes, operation context, retry classification and source chaining for App lock writes. It maps to the existing `MgError` only at the crate-visible lock-writer boundary.
Quản lý mã ổn định, ngữ cảnh operation, phân loại retry và source chain của thao tác ghi lock App. Lỗi chỉ được ánh xạ sang `MgError` hiện có tại boundary crate-visible.
