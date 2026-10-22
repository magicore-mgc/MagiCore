# Member-capture tests

Windows-gated tests verify that a live process outside the newly created private Job cannot be accepted as a member. PID-exit races and access-denied cases still need Windows stress/runtime coverage.

Test Windows xác minh process đang chạy nhưng nằm ngoài Job riêng mới tạo sẽ không được chấp nhận là member. Race process thoát và trường hợp access denied vẫn cần stress/runtime Windows.
