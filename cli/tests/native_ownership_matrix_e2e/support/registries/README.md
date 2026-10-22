# Hermetic registries / Registry hermetic

Each module owns a local registry fixture (Crates, Go, NuGet, Maven, Pub, or PyPI), its package/archive builder, and server lifecycle. E2E tests consume temporary local state only.

Mỗi module sở hữu fixture registry cục bộ (Crates, Go, NuGet, Maven, Pub hoặc PyPI), bộ dựng package/archive và vòng đời server. E2E chỉ dùng state tạm cục bộ.
