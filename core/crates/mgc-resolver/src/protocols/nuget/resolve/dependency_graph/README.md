# NuGet dependency-graph owner

Fetches the required flat-container nuspec, records unresolved/group markers and selects dependency sets by consumer TFM. Divergent TFM sets fail closed without a compatible consumer; identical sets keep the existing merge behavior. Shared `MgError` categories and protocol diagnostics remain the outward contract.

## Tiếng Việt

Tải nuspec flat-container bắt buộc, ghi marker group/dependency chưa resolve và chọn tập dependency theo TFM consumer. Tập TFM phân kỳ fail-closed nếu thiếu consumer tương thích; tập giống nhau giữ hành vi merge hiện tại. Category `MgError` và diagnostic protocol dùng chung vẫn là contract outward.
