# CLI definition tests / Test lược đồ CLI

This owner holds the command-definition parser tests beside the definitions module. `cli_surface.rs` covers global flags, aliases, help, core shape and cache CLI; `create_commands.rs` covers create payloads and container flags; `package_lifecycle.rs` covers package and publish lifecycle arguments; `trusted_operations.rs` covers trusted publish, signature audit and OCI binding inputs; `runtime_workspace.rs` covers dev, deploy and workspace options. The test `mod.rs` only wires those groups.

These tests parse CLI values and inspect the command schema. They do not execute package handlers or prove runtime installation behavior.

## Tiếng Việt

Owner này đặt test parser của command definitions cạnh module definitions. `cli_surface.rs` phủ cờ toàn cục, alias, help, core shape và cache CLI; `create_commands.rs` phủ payload create và cờ container; `package_lifecycle.rs` phủ tham số package/publish lifecycle; `trusted_operations.rs` phủ trusted publish, signature audit và OCI binding; `runtime_workspace.rs` phủ tùy chọn dev, deploy và workspace. `test/mod.rs` chỉ nối các nhóm.

Các test parse CLI value và đọc schema lệnh; chúng không chạy package handler và không chứng minh cài package khi runtime.
