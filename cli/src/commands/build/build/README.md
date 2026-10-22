# Build command owners / Các owner lệnh build

## Responsibility / Trách nhiệm

This folder splits the CLI build implementation by orchestration, shared project/toolchain policy, and core-specific build routes. `build.rs` in the parent directory remains the compatibility facade. The Web engine was split in the follow-on S162 slice under the sibling `web_engine/` directory; see its owner README and batch record.

Folder này tách implementation build của CLI theo orchestration, policy project/toolchain dùng chung và route riêng từng core. `build.rs` ở thư mục cha tiếp tục là facade tương thích. Web engine được tách ở batch S162 tiếp theo trong folder cùng cấp `web_engine/`; xem README owner và batch record của nó.

## Flow / Luồng chạy

`dispatch` resolves compatibility flags and project/core context, then calls the selected owner. `shared` owns project discovery, tool lookup and Go environment policy. The `ai`, `game`, `iot`, `lib`, `hardware`, `app`, `cloud`, and `web_go_backend` owners preserve the existing core-specific commands.

`dispatch` resolve cờ compatibility cùng project/core context rồi gọi owner được chọn. `shared` quản lý tìm project, dò tool và policy môi trường Go. Các owner theo core giữ nguyên lệnh build hiện có.

## Errors and tests / Lỗi và test

Owner-specific failures now live beside their AI, Game, IoT, Lib, Hardware, Cloud, Web-Go, App and Web-engine implementations. Shared cross-core missing-executable/freshness services remain in their shared owners. Local typed errors preserve current English CLI text; execution failures retain their source where the owner can represent one. Existing build unit tests stay at `cli/src/test/build_test.rs` and E2E remains under `cli/tests/`; moving/adding tests is deferred until the production-source pass closes.

Lỗi riêng của owner hiện nằm cạnh implementation AI, Game, IoT, Lib, Hardware, Cloud, Web-Go, App và Web engine. Service thiếu executable/freshness dùng chung vẫn thuộc shared owner. Error typed giữ nguyên text CLI tiếng Anh; lỗi thực thi giữ source khi owner có thể biểu diễn. Unit test hiện vẫn ở `cli/src/test/build_test.rs`, E2E ở `cli/tests/`; chỉ chuyển/thêm test sau khi đóng source production.
