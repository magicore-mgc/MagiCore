# Trust report owner

`report.rs` classifies scripted packages through the shared `mgc_config::project::decide_scripts` policy and renders the established denied/approved/pending sections. It owns deterministic sorting and output text; project access and filesystem traversal stay in the coordinator and scan owners.

The nearest tests cover deny precedence, exact-version database lookup, name fallback, and stable grouping. They are source-authored and NOT_RUN.

# Owner báo cáo trust

`report.rs` phân loại package có script bằng policy dùng chung `mgc_config::project::decide_scripts` và in các mục denied/approved/pending hiện hành. Owner này sở hữu sorting xác định cùng output text; truy cập project và duyệt filesystem thuộc coordinator/scan owners.

Test gần owner kiểm tra deny precedence, tra database theo version chính xác, fallback theo tên và nhóm kết quả ổn định. Test đã được viết, hiện NOT_RUN.
