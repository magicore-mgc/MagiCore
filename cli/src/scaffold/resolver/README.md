# Scaffold resolver / Bộ phân giải scaffold

This owner preserves the public `scaffold::resolver` surface while separating status, typed errors, missing-layer aggregation and CoreKind path mapping. Its `mod.rs` only wires/re-exports; implementation lives beside its own test owner.

Owner này giữ nguyên API công khai `scaffold::resolver`, đồng thời tách status, lỗi có kiểu, báo cáo layer thiếu và ánh xạ path theo CoreKind. `mod.rs` chỉ nối/export; implementation có test riêng gần nhất.
