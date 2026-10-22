# Go integrity owner

Computes the sorted-file SHA-256 directory digest encoded as Go sumdb `h1:`. It rejects newline-bearing names to avoid ambiguous records.

## Tiếng Việt

Tính directory digest SHA-256 trên danh sách file đã sắp xếp rồi encode `h1:` theo Go sumdb. Tên file có newline bị từ chối để tránh record nhập nhằng. Unit test hiện có được đặt trong `test/`.
