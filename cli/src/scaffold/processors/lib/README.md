# CLI Lib scaffold owners

## Responsibility
`lib.rs` keeps the stable `LibProcessor::files` API and dispatches to TypeScript, Python, Go, .NET, Java, or Rust fallback output owners. The owners only create files through the shared `write_file` helper.

## Phạm vi
`lib.rs` giữ API `LibProcessor::files` ổn định và dispatch tới owner output TypeScript, Python, Go, .NET, Java hoặc Rust fallback. Các owner chỉ ghi file qua helper `write_file` dùng chung.

## Error boundary / Ranh giới lỗi
Filesystem failures propagate through the existing `anyhow::Result` and shared writer; there is no distinct language-specific error state to encode here. These scaffold owners do not install packages, call package managers, or invoke compilers.

Lỗi filesystem được truyền qua `anyhow::Result` và writer dùng chung; không có trạng thái lỗi riêng theo ngôn ngữ cần mã hóa ở đây. Các owner scaffold không cài package, gọi package manager hoặc chạy compiler.

## Known behavior gap / Gap behavior đã biết
Only `ts`, `typescript`, `python`, `go`, `dotnet`, and `java` have explicit branches. Any other direct key currently falls through to Rust. This is preserved by the structural split and is not an assertion that all callers reject unsupported keys.

Chỉ `ts`, `typescript`, `python`, `go`, `dotnet`, `java` có branch tường minh. Key trực tiếp khác hiện rơi vào Rust. Refactor cấu trúc giữ nguyên behavior này và không khẳng định mọi caller đều từ chối key không hỗ trợ.
