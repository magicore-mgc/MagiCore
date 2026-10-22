# Cache lock shards / Shard lock cache

Keeps a fixed set of randomized lock shards. Cache memory stays bounded regardless of how many unique package IDs the process encounters; legacy path aliases normalize to the same shard.
Duy trì số shard lock cố định với bộ băm ngẫu nhiên. Bộ nhớ lock không tăng theo số package ID khác nhau; các alias path legacy được chuẩn hóa vào cùng shard.

Hash collisions can reduce parallelism by serializing unrelated IDs; read locks still allow concurrent reads within a shard. Prune acquires all write locks in index order before deleting data.
Va chạm hash có thể giảm song song do tuần tự hóa ID không liên quan; read lock vẫn cho phép nhiều lượt đọc cùng shard. Prune lấy toàn bộ write lock theo thứ tự index trước khi xóa dữ liệu.
