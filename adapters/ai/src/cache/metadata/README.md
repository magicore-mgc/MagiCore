# AI cache metadata

This stable module exports `CacheEntry`, `load_metadata` and `save_metadata`. The serialized field names and public signatures remain compatible. Entry shape/path metadata, access age, sidecar naming, safe load, atomic save and typed errors have separate owners.

Module ổn định này export `CacheEntry`, `load_metadata` và `save_metadata`. Tên field đã serialize và public signature được giữ tương thích. Shape/path entry, access age, tên sidecar, load an toàn, save atomic và lỗi typed có owner riêng.

Metadata is not yet wired into the AI model download lifecycle. Nearest tests live beside each owner; they remain NOT_RUN until the storage gate is met.

Metadata chưa được nối vào lifecycle tải model AI. Test gần từng owner; hiện NOT_RUN tới khi đạt điều kiện dung lượng.
