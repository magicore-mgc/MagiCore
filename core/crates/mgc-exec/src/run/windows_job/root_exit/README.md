# Windows Job root-exit owner

Owns final descendant inspection, termination and output-drain decisions after the monitored root exits. `mod.rs` exports the stable sibling entry; `root_exit.rs` implements cleanup; `error.rs` classifies stage-specific failures and preserves causes; `test/` contains native lifecycle and error contracts; `docs/` records progress.

# Owner root-exit Windows Job

Sở hữu quyết định inspection descendant cuối, terminate và drain output sau khi root process được giám sát thoát. `mod.rs` export entry ổn định cho sibling; `root_exit.rs` triển khai cleanup; `error.rs` phân loại lỗi theo phase và giữ nguyên nhân; `test/` chứa contract lifecycle native và lỗi; `docs/` ghi tiến độ.
