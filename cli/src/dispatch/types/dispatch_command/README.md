# DispatchCommand owner

Owns wraps a common command or a core-specific command without merging route identity. Its public path is re-exported through `crate::dispatch::types`. Tests here protect the command type contract only; route behavior is covered by the existing dispatcher mapping and engine tests.

## Tiếng Việt

Owner này chịu trách nhiệm wraps a common command or a core-specific command without merging route identity. API được tái xuất qua `crate::dispatch::types`. Test gần owner khóa hợp đồng kiểu lệnh; behavior route tiếp tục được kiểm tra tại test mapping dispatcher và engine hiện có.
