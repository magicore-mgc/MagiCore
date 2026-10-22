# cicd package dispatch / Điều phối package cicd

This owner handles only `cicd` `add`, `remove`, `list`, and `update` `CoreCommand` variants. It preserves the existing downstream `commands::core` calls, argument forwarding, and feature fallback. Current feature route: feature `cicd`; disabled builds keep `core_not_in_build("cicd")`.

Owner này chỉ xử lý bốn variant `add`, `remove`, `list`, `update` của `CoreCommand` thuộc `cicd`. Giữ nguyên lời gọi `commands::core` hiện có, tham số chuyển tiếp và fallback feature. Route feature hiện tại: feature `cicd`; disabled builds keep `core_not_in_build("cicd")`.

`mod.rs` exports `dispatch` and the command classifier to the package coordinator. `cicd.rs` contains the implementation. The nearest test checks all four owned variants and rejects a neighboring core variant. Tests are authored but NOT_RUN until the whole-project source pass closes.

`mod.rs` export `dispatch` và bộ phân loại lệnh cho coordinator package. `cicd.rs` chứa implementation. Test gần nhất kiểm tra đủ bốn variant và từ chối variant core kế bên. Test đã được viết nhưng NOT_RUN đến khi hoàn tất source pass toàn dự án.
