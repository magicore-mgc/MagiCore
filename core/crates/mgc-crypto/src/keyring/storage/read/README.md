# Keyring read owner

Owns the single keyring-content read contract and selects Unix, Windows or fallback implementations. Unix rejects symlinks/FIFOs and group/world or executable permissions; Windows rejects reparse points; other platforms reject non-regular entries.

Sở hữu contract đọc nội dung keyring và chọn implementation Unix, Windows hoặc fallback. Unix chặn symlink/FIFO và permission group/world hoặc executable; Windows chặn reparse point; OS khác chặn entry không regular.
