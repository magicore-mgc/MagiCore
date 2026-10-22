# Per-core common command adapter

This owner maps shared CLI command variants into the existing `CommonCommand` dispatch contract. It forwards each payload, including nested subcommands, and preserves the current `Option` fallback. Capabilities intentionally normalizes the CLI's explicit JSON flag because JSON is its only dispatch format.

The mapper is exhaustive and performs no fallible work, so it owns no local error type. Behavior tests live under `test/`; the coordinator and command handlers remain outside this adapter.

## Vietnamese

Owner này ánh xạ các variant lệnh CLI dùng chung sang hợp đồng dispatch `CommonCommand` hiện tại. Nó chuyển tiếp toàn bộ payload, gồm cả subcommand lồng nhau, và giữ fallback `Option`. Capabilities chuẩn hóa có chủ ý cờ JSON tường minh vì JSON là định dạng dispatch duy nhất.

Mapper exhaustive và không có thao tác phát sinh lỗi nên không cần error type cục bộ. Test hành vi nằm trong `test/`; coordinator và command handler nằm ngoài adapter này.
