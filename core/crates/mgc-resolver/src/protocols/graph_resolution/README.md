# Registry graph resolution

Owns the bounded BFS traversal used by the `RegistryProtocol` defaults. Processing remains queue-ordered; each BFS layer is buffered at 16, root graphs are buffered at 4 and share one semaphore. Conflicts fail closed.

Sở hữu BFS có giới hạn được `RegistryProtocol` dùng làm mặc định. Kết quả vẫn xử lý theo thứ tự queue; mỗi tầng BFS buffer 16, root graph buffer 4 và dùng chung semaphore. Xung đột vẫn fail-closed.
