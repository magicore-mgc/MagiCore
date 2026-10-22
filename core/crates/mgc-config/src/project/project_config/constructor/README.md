# ProjectConfig constructor / Constructor ProjectConfig

EN: Owns the inherent `ProjectConfig::new` method and its existing defaults, including the shared execution-policy selector. This owner does not validate ecosystem names or rewrite caller inputs. `constructor.rs` implements the method; `mod.rs` only wires the module; `test/` covers default invariants.

VI: Sở hữu method inherent `ProjectConfig::new` và các default hiện tại, gồm cả selector execution policy dùng chung. Owner này không validate tên ecosystem hoặc viết lại input caller. `constructor.rs` triển khai method; `mod.rs` chỉ nối module; `test/` kiểm tra invariant default.

Do not change field defaults, execution precedence or public signature in this structural split. Tests remain `NOT_RUN`.

Không đổi default field, thứ tự execution hoặc chữ ký công khai trong lần tách cấu trúc này. Test vẫn `NOT_RUN`.
