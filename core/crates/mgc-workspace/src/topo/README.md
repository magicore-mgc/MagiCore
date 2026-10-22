# Topological ordering / Sắp thứ tự topo

## English

The public facade preserves the crate's existing `TopoError` and `topo_levels` routes. `dependency_first_levels/` adapts those levels for workspace execution because graph edges point from a dependent package to its dependency; it reverses levels without changing the graph contract. The coordinator computes Kahn levels; `cycle_path/` builds the current cycle diagnostic; `error/` owns the public cycle failure and its display contract. The implementation currently indexes public edge indices without checking bounds, so malformed externally constructed graphs may panic. This slice preserves that gap; a separate behavior/API fix is required.

## Tiếng Việt

Facade public giữ nguyên đường dẫn `TopoError` và `topo_levels` của crate. `dependency_first_levels/` chuyển level sang thứ tự thực thi workspace vì cạnh graph đi từ package phụ thuộc tới dependency; module đảo level nhưng không đổi contract graph. Coordinator tính level Kahn; `cycle_path/` tạo chẩn đoán cycle hiện có; `error/` quản lý lỗi public và hợp đồng hiển thị. Thuật toán hiện giả định edge index trỏ tới node tồn tại. Lát này giữ nguyên gap đó; cần một behavior/API fix riêng.
