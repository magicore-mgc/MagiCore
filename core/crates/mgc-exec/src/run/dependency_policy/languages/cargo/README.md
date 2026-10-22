# Cargo policy

Owns Cargo subcommand classification. Dependency/registry mutation is denied; build/test-like commands require both `--locked` and `--offline`; unknown subcommands fail closed. The shared denial error and invocation normalization live at the parent policy owner.

# Policy Cargo

Sở hữu phân loại subcommand Cargo. Mutation dependency/registry bị chặn; build/test cần cả `--locked` và `--offline`; subcommand lạ bị chặn fail-closed. Error chung và chuẩn hóa invocation nằm ở policy owner cha.
