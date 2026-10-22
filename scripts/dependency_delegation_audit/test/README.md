# Dependency delegation audit tests / Test audit

The root `scripts/test_audit_dependency_delegation.py` remains the CI entrypoint. `test.py` aggregates the same eleven unittest classes. Shared fixture imports are in `support.py`; scanner tests stay in nearby feature modules. Large test classes use mixins split at method boundaries.

Entrypoint CI cũ vẫn giữ nguyên. `test.py` gom đúng 11 lớp unittest; `support.py` chứa helper/import chung, test scanner nằm trong module theo chức năng. Lớp lớn được chia mixin tại ranh giới method, không thay body test.
