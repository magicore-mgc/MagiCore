# Metadata fetch

The public facade orchestrates cache lookup, cached-record reuse policy, conditional ETag refresh, and full registry fetch with bounded stale fallback. Decision owners are separate modules; existing test registration stays next to the facade.

## Tải metadata

Facade public điều phối tra cứu cache, chính sách tái sử dụng record, refresh ETag có điều kiện, và fetch registry đầy đủ kèm fallback stale có giới hạn. Các quyết định nằm trong module riêng; đăng ký test hiện hữu vẫn ở cạnh facade.
