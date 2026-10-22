# Game optimizer error boundary

This folder owns the Game/Bevy optimizer's local typed error boundary. `mod.rs` keeps the existing `game::error::GameOptimizerError` route; `error/` owns variants and constructors, `display/` owns stable human diagnostics, and `source/` owns causal chaining. These errors remain local to the Game command module and do not replace shared CLI errors. Each implementation has its nearest tests.

# Boundary lỗi optimizer Game

Folder này sở hữu typed error boundary cục bộ cho optimizer Game/Bevy. `mod.rs` giữ route `game::error::GameOptimizerError`; `error/` quản lý variant/constructor, `display/` quản lý diagnostic ổn định, `source/` quản lý source chain. Lỗi vẫn nằm riêng trong module lệnh Game, không thay thế error dùng chung CLI. Mỗi implementation có test gần owner.
