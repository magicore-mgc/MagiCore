# Output types

Owns the small contracts shared by output capture and process supervision: `OutputMode` selects captured versus inherited streams, while `ExecOutcome` carries the child's final status and captured bytes. No output buffering or error policy lives here; those remain in the adjacent `output/` owner files.

# Type output

Sở hữu các hợp đồng nhỏ dùng chung giữa capture output và supervision process: `OutputMode` chọn stream capture hay inherit, còn `ExecOutcome` chứa status cuối và byte đã capture. Logic buffer và chính sách lỗi không nằm ở đây mà thuộc các file owner `output/` kế bên.
