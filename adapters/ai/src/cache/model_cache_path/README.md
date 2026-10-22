# Model cache path owner

This owner validates a model identifier before mapping it into the registry cache layout. Hugging Face uses the native `models--owner--repository` repository directory; other registries and `Registry::Local` preserve the existing `owner--repository` mapping. Path separators or filesystem-reserved forms that could address a path outside the cache root are rejected.

Owner này kiểm tra model ID trước khi map vào cache registry. Hugging Face dùng thư mục repository native `models--owner--repository`; registry khác và `Registry::Local` giữ mapping cũ `owner--repository`. ID có cú pháp path hoặc dạng filesystem dành riêng bị từ chối để không trỏ ra ngoài cache root.

The stable export is `model_cache_path`; its error returns through `MgResult` while retaining the typed validation error in the source chain. Hugging Face entry decoding is shared by listing and prune, so control directories do not become model IDs. Nearest source cases are in `test/model_cache_path.rs` and are not runtime evidence until executed.

Export ổn định là `model_cache_path`; lỗi trả về qua `MgResult` và giữ nguyên typed validation error trong source chain. Listing và prune dùng chung logic decode entry Hugging Face để thư mục control không bị xem là model ID. Test gần owner nằm tại `test/model_cache_path.rs`, hiện chưa chạy nên chưa phải runtime evidence.
