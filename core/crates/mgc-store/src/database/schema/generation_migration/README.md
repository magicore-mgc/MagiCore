# CAS generation schema migration

Detects and migrates pre-token CAS reference schemas while the caller holds the schema transaction. It preserves legacy rows and fail-closed migration checks.

## Migration schema CAS generation

Phát hiện và migrate schema CAS reference cũ khi caller đang giữ transaction schema. Giữ lại row cũ và các kiểm tra migration fail-closed.

## Phase owners

- `generation_token_migration_needed/`: checks the generation key and composite foreign key before deciding to rebuild legacy claims.
- `legacy_generation_column_present/`: distinguishes generation-bearing v1 from pre-generation v0 data.
- `create_v2_token_tables/`: creates replacement tables before migration copy.
- `migrate_v1_token_rows/`: copies v1 generation markers and claim rows.
- `migrate_v0_token_rows/`: converts legacy claims into the promoted generation-zero baseline.

## Các owner theo phase

- `generation_token_migration_needed/`: kiểm tra khóa generation và foreign key ghép trước khi quyết định rebuild claim cũ.
- `legacy_generation_column_present/`: phân biệt dữ liệu v1 có generation với dữ liệu v0 trước generation.
- `create_v2_token_tables/`: tạo bảng thay thế trước khi chép dữ liệu.
- `migrate_v1_token_rows/`: chép marker generation và claim row v1.
- `migrate_v0_token_rows/`: chuyển claim cũ thành baseline promoted generation 0.
