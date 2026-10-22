# Shared registry protocol contract

This folder owns native registry protocol engines and the shared contract used by all compatible cores. `mod.rs` only wires protocol modules and re-exports. `ResolvedEntry`, `RegistryRoot`, `RegistryProtocol`, bounded graph traversal, shared digests and the unsupported npm slot each have named owners.

Thư mục sở hữu engine registry native và contract dùng chung cho các core tương thích. `mod.rs` chỉ nối module và export. `ResolvedEntry`, `RegistryRoot`, `RegistryProtocol`, graph có giới hạn, digest dùng chung và slot npm unsupported có owner riêng.

The shared graph contract uses breadth-first layers and bounded concurrency. npm remains unsupported at this library slot; the Web adapter's own native path is separate. Changes here are high-impact because adapters and protocol implementations share the public types.

Contract graph dùng BFS và giới hạn concurrency. npm vẫn unsupported ở slot library này; đường native riêng của adapter Web tách biệt. Thay đổi tại đây ảnh hưởng rộng vì adapters và protocol implementations dùng chung type công khai.
