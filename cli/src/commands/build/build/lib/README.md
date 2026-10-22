# Library build owner / Owner build thư viện

## Responsibility / Trách nhiệm

Routes supported library projects to Rust, Python, Go, C#, Java or TypeScript handlers while preserving current manifest precedence and MGC ownership checks. `manifest/` owns only C# project and Java descriptor detection.

Định tuyến thư viện được hỗ trợ tới handler Rust, Python, Go, C#, Java hoặc TypeScript, giữ nguyên precedence manifest và kiểm tra ownership của MGC. `manifest/` chỉ nhận diện project C# và descriptor Java.

## Errors and tests / Lỗi và test

The owner-local `error.rs` defines toolchain, Python/Go/.NET compiler and Java-backend refusal errors. Compiler causes remain available through `source()`; the shared missing-local-executable helper stays shared. Language precedence and external-PM refusal do not change. Tests remain deferred until the repository source pass closes.

`error.rs` cục bộ định nghĩa lỗi toolchain, compiler Python/Go/.NET và từ chối Java backend. Lỗi compiler còn nguyên nguyên nhân qua `source()`; helper thiếu executable local dùng chung vẫn ở shared owner. Precedence ngôn ngữ và chính sách không gọi PM ngoài không đổi. Test chờ đến khi đóng source pass.