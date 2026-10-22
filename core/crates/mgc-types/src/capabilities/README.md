# Shared adapter capability owners / Owner capability adapter dùng chung

This namespace exposes the stable Global Gate 1 capability API while each contract lives in a same-name owner.

Namespace này expose API capability Global Gate 1 ổn định; mỗi hợp đồng nằm trong owner cùng tên.

**Contract / Hợp đồng:** Direct `capabilities::*` and crate-root exports remain stable. Trait defaults stay fail-closed, and the six-case real-adapter gate remains in `tests/capabilities.rs`.

Các export `capabilities::*` và crate-root giữ nguyên. Default trait tiếp tục fail-closed; gate sáu case trên adapter thật vẫn ở `tests/capabilities.rs`.
