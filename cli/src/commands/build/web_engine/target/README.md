# Web build target / Target build Web

Selects compatibility-shell, native-ready or compiled-executable from the explicit CLI value or project execution lane. This mapping is infallible and does not guess compiler availability.

Chọn compatibility-shell, native-ready hoặc compiled-executable từ giá trị CLI tường minh hoặc execution lane của project. Mapping không phát sinh lỗi và không đoán compiler có sẵn.

No local error type is needed for a pure selection; target tests remain in the CLI build test owner until the later test phase.

Không cần error type cho lựa chọn thuần; test target giữ ở owner test build CLI tới pha test sau.
