# Archive path owner / Owner path archive

Owns archive path sanitization, signature path formatting, and detection of a shared first-component root. Parent/root/prefix traversal is rejected with the existing diagnostic; `CurDir` is ignored as before.

Sở hữu làm sạch path archive, định dạng path chữ ký và phát hiện root component đầu chung. Parent/root/prefix bị từ chối với diagnostic cũ; `CurDir` tiếp tục bị bỏ qua.

Tests cover safe relative paths, parent traversal, and common-root/non-common-root inputs. This is a pure path policy and has no local type/error contract.

Test bao phủ relative path hợp lệ, parent traversal, và input có/không có root chung. Đây là policy path thuần, không có contract type/error riêng.
