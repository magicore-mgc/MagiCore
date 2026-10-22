# Go graph roots

Owns breadth-first expansion of multiple Go roots, visited `(module, requirement)` pairs, dependency-edge traversal, and highest-version retention per module. Keep deterministic map output and enqueue edges from every reachable requirement, including lower versions, to match Go MVS behavior.

## Tiếng Việt

Folder này sở hữu duyệt breadth-first nhiều Go root, cặp `(module, requirement)` đã thăm, cạnh dependency và giữ version cao nhất theo module. Phải giữ kết quả map có thứ tự và đưa cạnh từ mọi requirement truy cập được vào queue, kể cả version thấp hơn, để giữ hành vi Go MVS. Test graph đầy đủ vẫn ở integration target hermetic.
