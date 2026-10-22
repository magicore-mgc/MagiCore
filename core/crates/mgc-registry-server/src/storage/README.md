# Registry storage

This directory splits the registry store by responsibility. `mod.rs` wires the owner modules and preserves the public `crate::storage` facade. Owner folders contain their named implementation file, focused types/errors where owned, nearest source tests where existing tests can be moved without changing integration boundaries, and this README explains responsibility. Existing registered integration tests stay in `tests/`. No database schema or operation behavior is changed by this move.

# Registry storage — Tiếng Việt

Thư mục này tách registry store theo trách nhiệm. `mod.rs` nối các owner và giữ facade công khai `crate::storage`. Mỗi folder owner có file implementation cùng tên, type/error thuộc owner nếu có, test source gần nhất khi có thể chuyển mà không làm đổi ranh giới integration; README nêu rõ trách nhiệm. Các integration test đã đăng ký vẫn ở `tests/`. Việc di chuyển này không đổi schema database hay hành vi thao tác.
