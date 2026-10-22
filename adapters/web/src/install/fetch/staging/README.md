# staging

Owns unique same-filesystem staging paths, best-effort cleanup for an owned temporary file, first-writer-wins hard-link publication, and SHA-512 verification of a concurrent cache winner. Filesystem failures are typed under `error/` and converted back to the established `MgError::Store` contract at existing call sites. It does not choose registry URLs or fetch package bytes.

# staging / file tạm

Quản lý đường dẫn staging duy nhất trên cùng filesystem, dọn file tạm thuộc quyền sở hữu theo kiểu best-effort, publish hard-link theo first-writer-wins và xác minh SHA-512 của cache được process khác publish trước. Lỗi filesystem được typed trong `error/` và chuyển về hợp đồng `MgError::Store` tại các callsite hiện tại. Owner này không chọn URL registry hoặc tải bytes package.
