# Package-manager script policy / Policy package-manager script

Rejects build scripts that delegate to an external package manager, except the declared compatibility runner whose downstream execution is separately audited. The owner error is typed and preserves the existing message.

Từ chối build script ủy quyền package manager ngoài, trừ compatibility runner được khai báo và sẽ được audit riêng phía sau. Error của owner có kiểu và giữ message hiện tại.

Tests for native refusal and qualified-runner boundaries remain in the CLI build test owner pending the later test pass.

Test từ chối native và ranh giới runner khai báo đang ở owner test build CLI, chờ pha test.
