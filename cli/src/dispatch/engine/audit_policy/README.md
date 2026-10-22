# Audit-strict policy

This owner decides whether a command can honor the global `--audit-strict` flag. It maps no command, starts no process, and changes no environment state; `engine::run` owns those orchestration steps. Rejected commands use the shared `EnginePolicyError` in the parent engine module because recursive policy uses the same error owner.

## Vietnamese

Owner này quyết định lệnh nào thực sự áp dụng được cờ toàn cục `--audit-strict`. Nó không điều phối command, không chạy process và không đổi environment; các bước đó thuộc `engine::run`. Lệnh bị từ chối dùng `EnginePolicyError` chung ở module engine cha vì recursive policy cũng dùng error owner này.
