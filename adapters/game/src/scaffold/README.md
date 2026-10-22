# Game scaffolding / Scaffold Game

The facade owns template context and engine dispatch; each engine module owns its generated project files. Existing source templates are generated from these Rust constants.

Facade phụ trách context template và điều phối engine; module từng engine phụ trách file dự án được sinh. Template source hiện được tạo từ các hằng Rust tại đây.

Framework tests are in [`test/`](test/README.md). Errors use `MgError`/`MgResult` from shared types.
