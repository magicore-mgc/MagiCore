# Member capture owner

Owns ordered Job membership capture, PID deduplication and root exclusion, retained identity guards, snapshot retries, and the captured/exited/unavailable distinctions. Implementation is in member_capture.rs; mod.rs connects it to the coordinator. Existing capture tests live in test/.

# Owner chụp member

Sở hữu chụp membership Job theo thứ tự, loại PID trùng và root, giữ identity guard, retry snapshot, phân biệt member đã chụp/đã thoát/không đọc được. Code ở member_capture.rs; mod.rs nối lên coordinator. Test capture hiện có ở test/.
