# Template context / Context template

The facade exposes stable Web/Core context types and target-path helpers. `web_context/` owns Web context values; `core_context/` owns shared non-Web values and IoT board validation; `rendering/` owns shared token/path substitution; `error/` owns typed local diagnostics. Module files only connect these owners.

## Tiếng Việt

Facade export các type context Web/Core và helper path ổn định. `web_context/` quản lý giá trị Web; `core_context/` quản lý giá trị core ngoài Web và validate board IoT; `rendering/` quản lý thay token/path dùng chung; `error/` quản lý diagnostic typed cục bộ. Các file module chỉ nối owner.
