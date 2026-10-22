# Virtual-store cleanup tests

Nearest fixtures cover lock retention, ordinary orphan removal, missing/invalid roots, symlinked roots/candidates, nested symlink exclusion, malformed measurement roots, and logical-byte accounting. Symlink fixtures are compiled for Unix and Windows; their creation must succeed, so an unsupported runner fails visibly rather than passing by skipping the security case.

Fixture gần owner bao phủ giữ path còn trong lock, xóa orphan thường, root thiếu/sai loại, root/package symlink, bỏ qua symlink lồng, root đo sai loại và cách tính byte logic. Fixture symlink được biên dịch cho Unix và Windows; thao tác tạo link phải thành công, runner không hỗ trợ sẽ báo lỗi rõ thay vì pass giả do skip test bảo mật.
