# Prune candidate discovery

This owner lists direct regular-file and directory entries, excludes metadata sidecars and symbolic links, reads sidecar metadata only when it is a regular file, and measures content only for size-based pruning. For Hugging Face it accepts only `models--...` model repository directories, excluding root controls, datasets and spaces. Enumeration and requested size I/O failures propagate.

Owner này liệt kê file thường/thư mục trực tiếp, bỏ sidecar metadata và symlink, chỉ đọc sidecar khi đó là file thường, và chỉ đo nội dung với prune theo size. Với Hugging Face, chỉ nhận thư mục repository model `models--...`, bỏ control, dataset và space ở root. Lỗi enumeration và đo size khi được yêu cầu sẽ propagate.

Unreadable metadata remains unavailable to age/access policies, which retain that entry; size-based policy can still use measured content bytes. Tests are in `test/candidate_discovery.rs`.

Metadata không đọc được thì policy tuổi/truy cập không có dữ liệu và giữ entry; policy theo size vẫn có thể dùng content bytes đã đo. Test nằm tại `test/candidate_discovery.rs`.
