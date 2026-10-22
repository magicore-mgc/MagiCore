# Publish options owner

Owns the serializable/deserializable request configuration and its existing defaults (`latest`, three retries). This type does not parse CLI arguments or perform publishing.

Sở hữu cấu hình request có thể serialize/deserialize và các giá trị mặc định hiện có (`latest`, ba lần retry). Type này không parse CLI argument hoặc tự publish.

The default helpers stay private to this owner; nearest tests cover default values and omitted serde-defaulted fields.

Các helper default giữ private trong owner; test gần nhất kiểm tra giá trị default và field được serde tự điền khi bị lược khỏi dữ liệu.
