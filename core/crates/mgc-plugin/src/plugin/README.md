# plugin

The owner keeps the public `Plugin` API stable while separating legacy adapter bridges into `adapter_bridge/`. `mod.rs` only wires and re-exports. Constructor behavior, public fields, back-reference, context-scoped flag and unsupported capability errors remain as before.

# plugin — trách nhiệm

Owner giữ nguyên API công khai `Plugin` và tách bridge adapter cũ vào `adapter_bridge/`. `mod.rs` chỉ nối/export. Giữ behavior constructor, field public, back-reference, cờ context-scoped và lỗi capability chưa hỗ trợ.
