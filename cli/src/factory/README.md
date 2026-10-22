# Shared CLI adapter factory

This owner preserves the stable `crate::factory` facade while separating build-shape listing, current-project entry, root-bound adapter construction, scoped registry construction, and npm credential eligibility. Core selection remains exhaustive across Web, Lib, AI, App, Game, IoT, Cloud, CI/CD, and Hardware.

The root `mod.rs` and child `mod.rs` files only wire/re-export implementations. Feature gates, constructor behavior, plugin precedence, error propagation, and credentials remain owned by the matching function module. Existing integration contracts live in [`cli/tests/factory`](../../tests/factory/README.md).

# Factory adapter dùng chung trong CLI

Owner này giữ facade ổn định `crate::factory`, đồng thời tách danh sách core theo build, điểm vào project hiện tại, dựng adapter theo root, registry có scope và điều kiện dùng npm credentials. Bộ chọn vẫn bao phủ Web, Lib, AI, App, Game, IoT, Cloud, CI/CD và Hardware.

Các file `mod.rs` chỉ wiring/re-export. Feature gate, constructor, ưu tiên plugin, truyền lỗi và credentials nằm ở module cùng tên. Contract integration hiện có ở [`cli/tests/factory`](../../tests/factory/README.md).
