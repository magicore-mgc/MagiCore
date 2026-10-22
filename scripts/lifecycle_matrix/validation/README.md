# Validation / Kiểm tra claim

`__init__.py` is the lazy export surface. Validators are split by lane registry, native-PM claims, scaffold evidence, owners, adapter routes, binary dep-gate routes, framework catalog and platform evidence. They report explicit violations; they do not silently downgrade a broken claim.

`release_gate/` composes these contracts for the cross-platform CI release gate. Its context, per-lane scope/dimensions/claims, global scope and reports are separate owners; `main.py` is the workflow entry point. Keep the generic validator behavior here and the release-specific policy in that subpackage.

`__init__.py` là export lazy. Validator chia theo registry, native-PM, scaffold, owner, adapter route, dep-gate, framework catalog và platform evidence. Lỗi claim phải thành violation rõ ràng, không âm thầm hạ mức.

`release_gate/` ghép các contract này để chạy gate CI đa nền tảng. Context, scope/dimension/claim từng lane, scope toàn cục và báo cáo là các owner riêng; `main.py` là entry point workflow. Giữ hành vi validator dùng chung ở đây, policy dành riêng cho release ở subpackage đó.
