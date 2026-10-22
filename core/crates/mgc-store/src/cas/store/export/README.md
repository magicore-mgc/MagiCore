# CAS Export

Creates independent project files from CAS blobs and handles platform COW staging. The private owner router separates single export, batch coordination, COW/copy selection, platform clone syscalls, staged hashing, executable permissions and directory sync. S298 carries one no-follow CAS source handle through verification and descriptor COW/copy staging; owner-local source and stage errors map to the existing shared StoreError boundary.

Tạo file project độc lập từ blob CAS và xử lý COW staging theo platform. Router owner nội bộ tách export đơn, điều phối batch, chọn COW/copy, syscall clone từng OS, hash file staging, quyền executable và sync thư mục. S298 truyền một handle CAS mở không theo symlink qua bước xác minh và COW/copy bằng descriptor; lỗi cục bộ của owner nguồn/stage được map về ranh giới StoreError chung hiện có.

Seven nearest-owner source test modules cover each owner. Existing adversarial and compiled-cache integration suites remain unchanged. All tests are authored but NOT_RUN in the source-first phase.

Có bảy module source test gần owner, mỗi module bao phủ một owner. Hai integration suite adversarial và compiled-cache hiện có giữ nguyên. Test đã được viết nhưng NOT_RUN trong phase source-first.
