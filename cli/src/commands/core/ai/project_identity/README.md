# AI project identity

Owns AI project-root validation through the AI adapter and persisted core identity. It does not select runtime dependencies or claim support for a package lifecycle.

`project_identity.rs` contains the public cwd-based helper and a private path-based validation seam used by its nearest tests. The stable `ai_project_root` route is re-exported by the AI core and shared CLI facade.

# Nhận diện project AI

Sở hữu việc xác thực root project qua AI adapter và core identity đã lưu. Owner này không chọn runtime dependency hay tự nhận support package lifecycle.

`project_identity.rs` chứa helper public dựa trên cwd và seam kiểm tra path nội bộ dùng bởi test gần owner. Route `ai_project_root` ổn định được export qua AI core và shared CLI facade.
