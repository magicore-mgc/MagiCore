# Optimizer adapter selection

## Purpose

Owns `find_adapters`: matches detected runtimes against the registry and keeps the P0-4 rule that rival runtimes (Bun/Deno) stay dormant unless an explicit compat mode names them. Unknown or native mode never emits rival profiles.

## Phạm vi

Sở hữu `find_adapters`: khớp runtime đã phát hiện với registry và giữ quy tắc P0-4 — runtime đối thủ (Bun/Deno) ngủ yên trừ khi compat mode tường minh chỉ đích danh. Chế độ native/không rõ không bao giờ sinh profile đối thủ.
