# Command definitions

This owner defines the CLI command schema and preserves its type/module paths. CLI command words remain flat. The Rust `Commands` enum now groups variants by owner, so code that previously pattern-matched `Commands::Init` must use `Commands::Common(CommonCommands::Init)`; this is a source-level library API change. Parser/schema tests live in [`test/`](test/README.md) and are grouped by command area; they validate CLI parsing only.

## Vietnamese

Folder này sở hữu schema lệnh CLI và giữ đường dẫn module/kiểu. Tên lệnh trên CLI vẫn phẳng. Enum Rust `Commands` được nhóm theo owner nên code từng match `Commands::Init` cần chuyển sang `Commands::Common(CommonCommands::Init)`; đây là thay đổi source API của library. Test parser/schema nằm tại [`test/`](test/README.md), chia theo nhóm lệnh và chỉ kiểm tra parse CLI.
