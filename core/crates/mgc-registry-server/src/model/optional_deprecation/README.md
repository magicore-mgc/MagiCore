# Optional deprecation value / Giá trị deprecation tùy chọn

Owns the Serde adapter for npm's boolean/string/null `deprecated` representation. Its current behavior maps any present boolean or string to `Some("")` and null to `None`; non-empty deprecation text is lost. This extraction intentionally preserves that observed implementation so the issue can be handled as a separate behavior fix after the source decomposition phase.

Quản lý Serde adapter cho biểu diễn `deprecated` dạng boolean/chuỗi/null của npm. Hiện tại boolean hoặc chuỗi có mặt đều thành `Some("")`, còn null thành `None`; nội dung deprecation không rỗng bị mất. Lần tách này cố ý giữ nguyên source đã quan sát để xử lý lỗi ở phase sửa hành vi riêng sau khi bóc tách source.
