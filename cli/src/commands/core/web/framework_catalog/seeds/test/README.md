# test

Framework seed catalog contracts — kiểm tra contract catalog seed framework.

Tests here protect the visible framework order and seed constraints that affect manifest generation. Keep assertions next to the catalog data while retaining command-level tests for end-to-end creation behavior.

Test tại đây bảo vệ thứ tự framework và các constraint seed ảnh hưởng tới manifest. Đặt assertion gần dữ liệu catalog, đồng thời giữ test ở tầng command cho hành vi tạo project xuyên luồng.

The test module is registered only under `cfg(test)` from `seeds.rs`. Test sources were authored or moved for the source refactor and remain unexecuted until the repository verification phase.

Module test chỉ được đăng ký dưới `cfg(test)` từ `seeds.rs`. Test được thêm hoặc chuyển trong đợt refactor source và chưa chạy cho tới phase kiểm chứng toàn repo.
