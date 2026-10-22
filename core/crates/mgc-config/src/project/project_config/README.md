# ProjectConfig owners / Các owner ProjectConfig

EN: `project_config.rs` owns only the serialized `ProjectConfig` schema and the `default_version` serde function. The `constructor/` owner implements `ProjectConfig::new`; `script_grammar/` implements `ProjectConfig::script_grammar`. `mod.rs` wires those implementations and preserves the existing type export.

VI: `project_config.rs` chỉ sở hữu schema tuần tự hóa `ProjectConfig` và hàm serde `default_version`. Owner `constructor/` triển khai `ProjectConfig::new`; `script_grammar/` triển khai `ProjectConfig::script_grammar`. `mod.rs` nối các implementation và giữ export type hiện tại.

Field names, serde defaults/omission rules, public method signatures and all `mgc_config::project::*` paths are compatibility contracts. The serialized struct remains one type because splitting fields into nested configs would change the public and TOML shape.

Tên field, serde default/quy tắc bỏ field, chữ ký method công khai và mọi path `mgc_config::project::*` là contract tương thích. Struct tuần tự hóa vẫn là một type; tách field thành config lồng nhau sẽ đổi API và cấu trúc TOML.

Nearest owner tests are in each implementation's `test/` folder and in the schema `test/` folder. They remain `NOT_RUN` until the repository-wide source-first phase closes.

Test gần owner nằm trong `test/` riêng của từng implementation và trong `test/` của schema. Test vẫn `NOT_RUN` đến khi đóng phase source-first toàn repo.
