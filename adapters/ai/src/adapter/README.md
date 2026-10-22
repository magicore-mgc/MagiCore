# AI adapter facade

This owner is the public adapter boundary for the AI crate. It re-exports `AiAdapter`, `adapter_for`, and `uses_native_python_lane`; the nested `adapter/` tree owns implementation contracts. Framework detection remains in `../framework.rs`.

## Chủ sở hữu

Đây là ranh giới adapter công khai của crate AI. Folder export `AiAdapter`, `adapter_for` và `uses_native_python_lane`; cây `adapter/` con sở hữu các contract implementation. Nhận diện framework vẫn nằm ở `../framework.rs`.
