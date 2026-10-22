# resolution / resolution

## Responsibility / Trách nhiệm

Keep the stable RegistryProtocol implementation and coordinate the PyPI resolution sequence. Release candidate policy, Python target selection, selected-version metadata, marker projection, dependency projection and binary download belong to explicit child owners.

Giữ implementation RegistryProtocol và điều phối thứ tự resolve PyPI ổn định. Policy chọn release, chọn Python target, tải metadata version đã chọn, tạo marker, chiếu dependency và tải artifact thuộc owner con riêng.

## Contract / Hợp đồng

The coordinator preserves request order, diagnostics, version and file selection order, dependency order, and the public PypiProtocol export. Shared error values come from MgError; selection-transfer structs live in the local types module.

Coordinator giữ thứ tự request, diagnostic, chọn version/file, thứ tự dependency và export PypiProtocol. Error dùng MgError chung; struct chuyển dữ liệu selection nằm trong types local.

## Tests and extension / Test và mở rộng

Behavior tests for the new owners live under each owner's test folder. Existing mock-server integration tests stay in the registered protocols_pypi integration suite because they verify the public trait and request boundary. Tests are authored but remain NOT_RUN during the source-first phase.

Test behavior của owner mới nằm trong test folder cạnh owner. Integration test mock server giữ ở suite protocols_pypi đã đăng ký vì chúng kiểm tra trait public và ranh giới request. Test được viết nhưng NOT_RUN trong pha source-first.
