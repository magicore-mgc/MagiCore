# Guarded PATH owner

EN: Owns host-specific PATH key matching and PATH composition. The last explicit PATH value wins over inherited PATH, the shadow directory is prepended, and native path splitting/joining errors remain visible to the shared execution boundary.

VI: Sở hữu so khớp key PATH theo hệ điều hành và ghép PATH. Giá trị PATH tường minh cuối cùng được ưu tiên hơn PATH kế thừa, shadow directory đứng đầu, lỗi split/join path được trả về boundary thực thi dùng chung.
