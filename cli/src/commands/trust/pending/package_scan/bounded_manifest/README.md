# Bounded manifest reader owner

`bounded_manifest.rs` permits only a regular non-symlink file, enforces the existing 10 MiB limit before and during reading, and uses platform no-follow flags. These constraints prevent the trust report from following a swapped link or reading an unbounded manifest.

# Owner đọc manifest có giới hạn

`bounded_manifest.rs` chỉ chấp nhận file thường không phải symlink, giữ giới hạn 10 MiB trước và trong lúc đọc, đồng thời dùng flag no-follow theo nền tảng. Các rào này ngăn báo cáo trust follow link bị thay thế hoặc đọc manifest không giới hạn.
