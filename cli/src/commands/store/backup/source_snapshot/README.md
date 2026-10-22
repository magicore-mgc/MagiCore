# Project source snapshot / Snapshot nguồn project

Owns the pre-lock and post-lock project root checks, database no-follow probe, source-root stability checks and the maintenance lock guard. The guard is returned with the snapshot so the coordinator holds it through backup completion.

Quản lý kiểm tra root trước/sau lock, probe database no-follow, xác nhận source root ổn định và maintenance lock. Guard được trả cùng snapshot để coordinator giữ đến khi backup kết thúc.
