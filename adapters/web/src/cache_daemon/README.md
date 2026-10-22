# Process-local memory cache / Cache RAM theo process

This owner provides the existing `MemoryCacheDaemon` singleton API. It is a process-local `OnceLock` plus mutex-protected map, not a persistent store or cross-process shared package store. The implementation and public route are preserved in this structural refactor.

Owner này cung cấp API singleton `MemoryCacheDaemon` hiện có. Nó là `OnceLock` trong một process cùng map có mutex, không phải store bền vững hoặc kho package chia sẻ xuyên process. Batch refactor cấu trúc này giữ nguyên implementation và public route.

Current source bounds entry count by clearing after the map is already over 10,000 entries; it does not bound total bytes or an individual value. The singleton retains its entries until process exit or a clear. Cache byte-budget/eviction policy is a separate follow-up and is not solved here.

Source hiện tại chỉ giới hạn số entry bằng cách clear khi map đã vượt 10.000 entry; không giới hạn tổng byte hoặc kích thước từng value. Singleton giữ entry đến khi process kết thúc hoặc map bị clear. Chính sách byte-budget/eviction cần batch riêng và chưa được giải quyết ở đây.
