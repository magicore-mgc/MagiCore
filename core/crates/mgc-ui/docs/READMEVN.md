# mgc-ui

<!-- English module guide: purpose, source map, and review boundaries. -->
<!-- Hướng dẫn tiếng Việt về chức năng, source và ranh giới review của module. -->

## Chức năng

Cung cấp primitive giao diện terminal: help, progress bar/spinner, prompt, table, output trạng thái và quiet mode. Crate sở hữu presentation dùng lại, không sở hữu policy command hay nghiệp vụ.

## Sơ đồ source

- [facade source](../src/README.md)
- [crate exports](../src/lib.rs)
- [banner owner](../src/banner/README.md)
- [command-style owner](../src/command_style/README.md)
- [help owner](../src/help/README.md)
- [install-summary owner](../src/install_summary/README.md)
- [next-steps owner](../src/next_steps/README.md)
- [progress owner](../src/progress/README.md)
- [quiet-state owner](../src/quiet/README.md)
- [section owner](../src/section/README.md)
- [status-message owner](../src/status/README.md)
- [prompt owner](../src/prompt/README.md)
- [table owner](../src/table/README.md)

## Bàn giao và review

Giữ output terminal tiếng Anh theo RULE.md và bảo toàn quiet/non-TTY. Presentation không xác nhận capability của command, cũng không thay thế diagnostic của engine.

## Xác minh

Batch source-first đã viết test nhưng trạng thái vẫn NOT_RUN. Sau khi đóng toàn bộ source và dung lượng máy dùng dưới 80 GB, chạy test gần owner cùng cargo test -p mgc-ui.
