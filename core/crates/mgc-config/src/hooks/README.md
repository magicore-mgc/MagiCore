# User hooks / Hook người dùng

This owner decomposes the `mgc_config::hooks` API into configuration, path, merge, dependency-policy, parser, execution and listing modules. It does not create a shell; execution must remain routed through `mgc_exec`.

Owner này tách API `mgc_config::hooks` thành các module cấu hình, path, merge, policy dependency, parser, execution và listing. Không tạo shell; việc chạy lệnh phải tiếp tục qua `mgc_exec`.

Local test modules cover owner behavior; the existing integration tests remain the public/process-policy backstop. All tests are currently `NOT_RUN`.

Test cục bộ bao phủ hành vi theo owner; integration test hiện có tiếp tục kiểm tra API công khai và process policy. Hiện tất cả đều `NOT_RUN`.
