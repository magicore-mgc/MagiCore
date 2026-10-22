# Node local command / Lệnh Node cục bộ

Resolves `node_modules/.bin` entries and Windows `.cmd` script targets, selects Node as the launcher, and constructs a project-local PATH prefix. It never installs or downloads executables.

Resolve entry `node_modules/.bin` và target script `.cmd` trên Windows, chọn Node làm launcher và dựng prefix PATH cục bộ project. Không cài hoặc tải executable.

`NodeCommandError` owns missing-local-binary and PATH join failures and retains a source for the latter. Symlink/shim behavior tests remain in `cli/src/test/build_test.rs` until test relocation.

`NodeCommandError` sở hữu lỗi thiếu binary cục bộ và ghép PATH, giữ source cho lỗi ghép path. Test symlink/shim còn ở `cli/src/test/build_test.rs` tới pha chuyển test.
