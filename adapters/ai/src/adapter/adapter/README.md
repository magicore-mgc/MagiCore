# AI adapter implementation

The coordinator owns the public adapter value, native-Python lane selection, constructor, and source-test compatibility imports. Capability implementations live under their named owners. Keep the public fields and capability arrays stable; Python operations remain delegated to the existing native lane, while framework detection remains a separate module.

## Chủ sở hữu

Coordinator sở hữu giá trị adapter công khai, chọn lane Python native, constructor và import tương thích cho source test. Implementation capability nằm trong owner mang tên riêng. Giữ ổn định field công khai và capability array; thao tác Python tiếp tục ủy quyền lane native hiện hữu, nhận diện framework vẫn là module riêng.
