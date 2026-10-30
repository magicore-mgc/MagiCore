# `cli/` — Giao diện dòng lệnh MagiCore

Đây là binary `mgc`; các lệnh dành cho người dùng được triển khai tại đây.

## Cấu trúc

```text
cli/src/
├── main.rs               # Điểm vào: phân tích tham số Clap rồi dispatch
├── commands/             # Module theo từng lệnh
│   ├── definitions/      # Clap struct — nguồn khai báo subcommand
│   ├── install/          # mgc install
│   ├── audit/            # mgc audit theo core
│   ├── mcp/              # mgc mcp — MCP server JSON-RPC 2.0 qua stdio
│   ├── doctor/           # mgc doctor check — chẩn đoán môi trường
│   ├── model/            # mgc model push/pull/list cho OCI model registry
│   ├── workspace/        # mgc workspace
│   └── ...               # Các lệnh còn lại
├── dispatch/
│   ├── engine/           # Dispatch cấp cao: --recursive, --filter, workspace loop
│   ├── common/           # Định tuyến CommonCommand tới handler
│   ├── per_core/         # Ánh xạ Commands thành lệnh chung hoặc theo core
│   ├── bare/             # Xử lý lệnh trần và tự nhận diện core
│   └── types/            # Các enum CommonCommand và CoreCommand
├── context/              # ProjectContext: đọc .mgc.core, mgc.toml và nhận diện hệ sinh thái
└── scaffold/             # Engine scaffold: nguồn theo registry, catalog initializer (không nhúng template FW)
```

## Thêm lệnh mới

1. Thêm variant vào clap struct trong `commands/definitions/commands/`.
2. Tạo `commands/<name>.rs` với hàm `pub async fn run(...)`.
3. Khai báo `pub mod <name>;` trong `commands/mod.rs`.
4. Ánh xạ variant trong `dispatch/per_core.rs` sang `CommonCommand::<Name>`.
5. Thêm nhánh dispatch trong `dispatch/common.rs`.
6. Thêm tên lệnh vào `command_name()` trong `dispatch/engine.rs`.
