# runtime_verification / xác minh runtime

Coordinates the wheel metadata identity, RECORD rows and exact installed-site file set. It keeps the existing `PypiProtocol` method path used by the materializer and does not own ZIP parsing, CSV parsing or path traversal policy.

Điều phối identity metadata wheel, các dòng RECORD và tập file site đã cài chính xác. Giữ nguyên method path `PypiProtocol` mà materializer gọi; không sở hữu parser ZIP/CSV hay policy path.
