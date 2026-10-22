# Ownership tests

`native_project.rs` checks the native-lane boundary for simple projects, foreign locks/sources, and allowed versus unsupported dependency groups. These tests are registered under the ownership module so failures identify the policy owner.

## Phạm vi test

`native_project.rs` kiểm tra ranh giới lane native với project đơn giản, lockfile/nguồn bên ngoài, và dependency group được hỗ trợ hoặc bị từ chối. Test được đăng ký dưới module ownership để lỗi chỉ đúng owner policy.
