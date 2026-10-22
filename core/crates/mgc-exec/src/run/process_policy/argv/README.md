# argv

Owns the argv matcher used by process monitoring. Its ordered decision is scope-sensitive `adb`/`xcrun` descendant classification, then direct forbidden executable image, argv[0] manager script, runtime entrypoint, and dependency-resolution policy using only the observed process environment. DeviceControl suppresses only the device-tool denial; it does not suppress package-manager or dependency policy.

Sở hữu matcher argv cho process monitor. Thứ tự quyết định là phân loại descendant `adb`/`xcrun` theo scope, executable bị cấm trực tiếp, script manager ở argv[0], runtime entrypoint, rồi dependency-resolution policy dùng đúng environment của process được quan sát. DeviceControl chỉ gỡ deny cho công cụ thiết bị; không gỡ policy package manager/dependency.

It returns the established optional four-field match; no-match is expected. Runtime entrypoint details belong to runtime_invocation, and no process is started here. Focused tests live under test/.
