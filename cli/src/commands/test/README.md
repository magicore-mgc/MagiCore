# Test command owner

`mod.rs` exports `test` and the crate-local `test_at` entry. `test.rs` owns runner selection, policy checks, optimizer environment loading, and execution through `mgc-exec`. `test_runner.rs` in this folder is its focused unit-test module.

`mod.rs` export `test` và entry `test_at` trong crate. `test.rs` sở hữu chọn runner, kiểm tra policy, nạp môi trường optimizer và thực thi qua `mgc-exec`. `test_runner.rs` trong folder là test unit gần chức năng.
