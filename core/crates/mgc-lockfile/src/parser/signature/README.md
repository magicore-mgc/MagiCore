# Signature sidecar / Sidecar chữ ký

Owns `.lock.sig` path derivation, bounded sidecar presence checks, and the deprecated boolean compatibility wrapper. It reuses the bounded-reader error contract and distinguishes NotFound from read/path errors; presence alone does not parse or authenticate the sidecar.

Owner này tạo path `.lock.sig`, kiểm tra sidecar có giới hạn và giữ wrapper boolean đã deprecated để tương thích. Nó dùng lại error contract của bounded reader, phân biệt NotFound với lỗi đọc/path; chỉ kiểm tra sự tồn tại không parse hay xác thực sidecar.
