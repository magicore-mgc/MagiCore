# Project parse tests

`parse.rs` tests distinguish base dependencies, the selected PEP 735 `dev` group, inactive optional extras, and fail-closed unsupported groups at the project-file boundary.

## Phạm vi test

`parse.rs` phân biệt dependency nền, nhóm `dev` PEP 735 được chọn, optional extras không hoạt động, và group ngoài phạm vi bị từ chối ngay tại ranh giới project file.
