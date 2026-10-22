# Quarantine Owner

**Purpose:** Preserve forensic evidence for a CAS digest mismatch by copying from the exact already-opened file handle; never move or unlink an entry by its mutable source pathname.

**Mục đích:** Giữ bằng chứng điều tra cho blob CAS sai digest bằng cách sao chép từ đúng handle đã mở; không move hoặc unlink entry theo pathname có thể bị thay.

The helper receives the CAS root and verified `File` explicitly. It copies with a bounded buffer, checks that the evidence digest equals the verified mismatch digest, and publishes unique evidence and report files without clobbering. Evidence failure is best-effort and never masks the primary `HashMismatch`; the corrupt CAS entry remains until a valid import atomically repairs it. Its `test/` directory checks source retention, report fields and path replacement after open.

Helper nhận CAS root và `File` đã xác minh tường minh. Helper sao chép bằng buffer giới hạn, xác nhận digest bằng digest mismatch đã kiểm, rồi publish file bằng chứng/report duy nhất không ghi đè. Lỗi evidence là best-effort và không che `HashMismatch` gốc; entry CAS hỏng được giữ cho tới khi import hợp lệ sửa atomic. `test/` kiểm tra source retention, report và path replacement sau open.
