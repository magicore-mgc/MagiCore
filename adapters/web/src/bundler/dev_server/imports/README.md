# Import rewriting / Viết lại import

Owns the cached bare-import recognizer, native dependency URL rewrite, and safe JavaScript string-literal encoding shared by dependency and compiler responses.

Sở hữu bộ nhận diện bare-import có cache, viết lại URL dependency native và mã hóa JavaScript string literal an toàn dùng chung cho response dependency/compiler.

The matcher is retained byte-for-byte; relative, absolute and already rewritten URLs keep their existing behavior. Unit assertions remain in the unchanged CLI test file.

Regex được giữ nguyên; URL tương đối, tuyệt đối và đã rewrite giữ hành vi hiện tại. Assertion unit vẫn ở file test CLI hiện hữu.
