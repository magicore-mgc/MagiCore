# Generic cache-file pruning

Owns generic cache-file pruning.

## Phạm vi

Folder này sở hữu prune file theo predicate và giữ file có hardlink ngoài. `files.rs` chứa implementation; `mod.rs` chỉ nối module và export API nội bộ cần thiết.
