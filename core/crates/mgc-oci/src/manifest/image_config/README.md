# image_config owner

Owns oci image configuration models and its public compatibility export from `mgc_oci::manifest`. This owner contains data definitions/serialization shape only; it introduces no fallible behavior, so no local `error.rs` is warranted. Errors from HTTP/serde operations remain at the `OciClient` boundary.

Owner này sở hữu model cấu hình oci image và export tương thích tại `mgc_oci::manifest`. Chỉ định nghĩa dữ liệu/hình dạng serialization, không tạo thao tác có thể lỗi nên không cần `error.rs` giả. Lỗi HTTP/serde thuộc boundary `OciClient`.
