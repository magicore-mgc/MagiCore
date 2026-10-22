# AI dependency resolver

## Responsibility

Forwards native dependency graph resolution and capability probing to the Python lane. The implementation is private and preserves the current trait boundary and diagnostics. The nearest existing lifecycle coverage is in `adapters/ai/tests/integration/python_lane/`; it exercises `resolve`/`install` and capability claims.

## Trách nhiệm

Chuyển tiếp resolve graph dependency và capability probe sang lane Python. Implementation private, giữ ranh giới trait và diagnostic hiện tại. Test lifecycle gần nhất nằm ở `adapters/ai/tests/integration/python_lane/`; hiện kiểm tra `resolve`/`install` và capability claim.
