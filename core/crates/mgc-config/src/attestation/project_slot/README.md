# Project identity slot / Identity slot project

Owns the `ATTESTATION_FILE` name, deterministic per-project external slot mapping and secure `mgc.toml` byte read through `ProjectConfig::read_regular_project_text`.

Quản lý tên `ATTESTATION_FILE`, ánh xạ slot ngoài project xác định theo project và đọc bytes `mgc.toml` an toàn qua `ProjectConfig::read_regular_project_text`.

Read failures retain the existing `anyhow` chain; missing config still returns the existing diagnostic. Tests cover canonical-path mapping and exact config bytes.

Lỗi đọc giữ nguyên source chain `anyhow`; thiếu config trả diagnostic hiện hành. Test kiểm tra ánh xạ canonical path và bytes config chính xác.
