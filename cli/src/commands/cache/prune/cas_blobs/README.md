# CAS refcount pruning

Owns cAS refcount pruning.

## Phạm vi

Folder này sở hữu prune blob theo refcount SQLite và từ chối khi DB/refcount không đáng tin. `cas_blobs.rs` chứa implementation; `mod.rs` chỉ nối module và export API nội bộ cần thiết.
