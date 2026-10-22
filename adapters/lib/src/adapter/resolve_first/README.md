# Resolve-first add owner

`resolve_first.rs` resolves one requested package through the same native registry factory as project resolution, then returns a prepared package without writing project files. `pinning.rs` owns the ecosystem-specific representation of an unpinned request. The CLI mutation gateway remains responsible for committing the manifest and lock transaction.

The shared protocol factory preserves Rust's current operation-specific registry identity and applies the existing NuGet target framework. No external package manager is spawned.

## Phạm vi

`resolve_first.rs` resolve một package được yêu cầu qua cùng protocol factory native với resolve dự án, sau đó trả package đã chuẩn bị mà chưa ghi file project. `pinning.rs` sở hữu cách biểu diễn request chưa pin theo ecosystem. CLI mutation gateway vẫn chịu trách nhiệm commit manifest và lock trong transaction.

Protocol factory chung giữ registry identity hiện tại riêng theo thao tác của Rust và áp dụng target framework NuGet như trước. Không spawn package manager bên ngoài.
