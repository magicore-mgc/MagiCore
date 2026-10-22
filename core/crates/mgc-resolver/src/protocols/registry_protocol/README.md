# RegistryProtocol

Defines the cross-language registry interface and default behavior for version matching, artifact integrity, CAS references and dependency graph resolution. Every registry implementation and the library adapter core share this contract.

Định nghĩa interface registry đa ngôn ngữ và mặc định cho version, toàn vẹn artifact, CAS reference và graph dependency. Mọi implementation registry và adapter library dùng chung hợp đồng.

The defaults fail closed on missing/mismatched SHA-256 and incompatible version choices. Protocols with different integrity/range rules override the relevant method.

Mặc định fail-closed khi thiếu/lệch SHA-256 hoặc chọn version xung đột. Protocol có quy tắc integrity/range khác override method tương ứng.
