# Hugging Face client errors / Lỗi client Hugging Face

Owns typed request, HTTP status, JSON decoding and invalid-model-ID failures.
Quản lý lỗi request, HTTP status, giải mã JSON và model ID rỗng theo kiểu riêng.

The owner keeps safe operation context and a sanitized `reqwest::Error` source, then maps to the existing shared `MgError` at API boundaries.
Owner giữ ngữ cảnh operation an toàn cùng source `reqwest::Error` đã gỡ URL, sau đó ánh xạ sang `MgError` chung tại ranh giới API.
