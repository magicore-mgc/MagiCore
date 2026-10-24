# Dependency delegation audit tests / Test audit

The CI entrypoint is `scripts/test/test_audit_dependency_delegation.py`. `test.py` aggregates the same eleven unittest classes. Shared fixture imports are in `support.py`; scanner tests stay in nearby feature modules. Large test classes use mixins split at method boundaries.

Entrypoint CI là `scripts/test/test_audit_dependency_delegation.py`. `test.py` gom đúng 11 lớp unittest; `support.py` chứa helper/import chung, test scanner nằm trong module theo chức năng. Lớp lớn được chia mixin tại ranh giới method, không thay body test.
