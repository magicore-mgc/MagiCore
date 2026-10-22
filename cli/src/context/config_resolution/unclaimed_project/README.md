# Unclaimed project owner / Owner project chưa claim

`unclaimed_project.rs` builds the same synthetic `ProjectConfig` values used when an explicit `--core` is supplied for an unclaimed root or when no project root is found. It does not detect a core or write markers.

`unclaimed_project.rs` tạo `ProjectConfig` tạm như trước khi có `--core` cho root chưa claim hoặc chưa tìm thấy project root. Owner này không nhận diện core và không ghi marker.

Nearest tests pin the existing project-name fallback behavior. Test source is not yet run.

Test gần owner khóa quy tắc tên project hiện hành. Test chưa chạy.
