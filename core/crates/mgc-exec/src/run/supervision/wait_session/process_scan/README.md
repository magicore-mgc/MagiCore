# Process scan

Owns process-scan result handling and platform-specific active/pre-exit inspection for a wait session.

Unix scans reuse the existing process monitor, identity tracking, process-group tracking, and retry policy. Windows scans reuse the existing Job Object inspector and retry controller. Inspection uncertainty remains fail-closed, and forbidden-child diagnostics continue through the shared supervision diagnostic owner.

# Quét cây process

> Owner này xử lý kết quả process scan và việc kiểm tra trước-exit/đang-chạy theo từng nền tảng.

> Unix tiếp tục dùng process monitor, identity tracking, process group và retry hiện tại. Windows tiếp tục dùng Job Object inspector và retry controller. Không chắc chắn khi inspection vẫn fail-closed; diagnostic child bị cấm vẫn đi qua owner diagnostics hiện có.
