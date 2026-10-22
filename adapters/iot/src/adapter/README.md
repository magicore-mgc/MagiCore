# IoT adapter contract

This owner tree contains the public `IotAdapter` state and factory plus independent capability implementations and the strict PlatformIO manifest reader. `mod.rs` preserves the existing adapter module exports; `adapter.rs` owns the adapter state and inherent API.

Cây owner này chứa state và factory public của `IotAdapter`, các implementation capability riêng và parser manifest PlatformIO strict. `mod.rs` giữ export module hiện có; `adapter.rs` sở hữu state và inherent API của adapter.

ESP32-Rust remains the only natively writable IoT manifest. PlatformIO/Zephyr dependency mutations and installation remain fail-closed. The parser does not call `pio` or invent an empty dependency graph. Existing framework, scaffold, install, flash, CLI and test owners remain outside this source slice.

ESP32-Rust vẫn là manifest IoT duy nhất được ghi native. Mutation dependency và install PlatformIO/Zephyr tiếp tục fail-closed. Parser không gọi `pio` và không tạo graph dependency rỗng giả. Framework, scaffold, install, flash, CLI và test owner hiện có nằm ngoài lát source này.
