# Lib audit / Audit của Lib

This module keeps the public scanner facade and owns the adapter-specific audit route. `scanner` remains a re-export of the shared `mgc-audit` scanners; the internal dispatcher selects a `LibLanguage` lane and delegates mixed Rust projects to language-specific plan owners.

Module này giữ facade scanner public và sở hữu route audit riêng của adapter. `scanner` tiếp tục re-export scanner dùng chung từ `mgc-audit`; dispatcher nội bộ chọn lane `LibLanguage` và giao project Rust đa ngôn ngữ cho owner plan theo ngôn ngữ.

Audit findings and unavailable states are returned by the shared scanner/report types. This source-only slice does not change coverage claims; current tests call Rust/Python scanners directly, while dispatcher and polyglot behavior still need direct tests after source decomposition closes.

Finding và trạng thái unavailable do scanner/report type dùng chung trả về. Slice source-only này không đổi claim coverage; test hiện gọi scanner Rust/Python trực tiếp, còn dispatcher và polyglot cần test riêng sau khi source decomposition đóng.
