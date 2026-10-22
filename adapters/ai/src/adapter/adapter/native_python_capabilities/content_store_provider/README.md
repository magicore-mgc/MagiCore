# AI content store provider

## Responsibility

Forwards content-store installation and probe operations to the selected native Python lane. The implementation is private and preserves the current trait boundary and diagnostics. The nearest existing lifecycle coverage is in `adapters/ai/tests/integration/python_lane/`; it exercises `resolve`/`install` and capability claims.

## Trách nhiệm

Chuyển tiếp thao tác cài vào content store và probe tới lane Python native được chọn. Implementation private, giữ ranh giới trait và diagnostic hiện tại. Test lifecycle gần nhất nằm ở `adapters/ai/tests/integration/python_lane/`; hiện kiểm tra `resolve`/`install` và capability claim.
