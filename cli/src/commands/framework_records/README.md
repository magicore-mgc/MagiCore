# Framework qualification catalog / Catalog qualification framework

## Responsibility / Trách nhiệm

Owns the stable public FrameworkRecord API and qualification JSON used by mgc capabilities. Data is divided into shared DTO/evidence, nine core owners and dependency projection; Web is split into three families.

Sở hữu API FrameworkRecord ổn định và JSON qualification của mgc capabilities. Dữ liệu tách thành DTO/evidence, chín owner core và projection dependency; Web chia thành ba nhóm.

## Flow and errors / Luồng và lỗi

The top-level facade preserves public exports and test registration. The coordinator filters ordered rows and delegates dependency ownership to dep_gate. Static catalog logic has no local error.rs.

Facade ngoài cùng giữ export public và đăng ký test. Coordinator lọc row có thứ tự rồi giao dependency ownership cho dep_gate. Logic catalog tĩnh không có error.rs riêng.

## Tests / Kiểm thử

The existing suite remains at cli/src/commands/test/framework_records_test.rs and is deferred until source decomposition closes.

Suite hiện tại vẫn ở cli/src/commands/test/framework_records_test.rs, hoãn tới khi đóng source pass.
