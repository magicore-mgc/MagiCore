# Lifecycle contract / Hợp đồng lifecycle

`__init__.py` is the lazy export surface. `operations.py`, `dimensions.py`, `ownership.py` and `status.py` each own one contract area. Changes to status meaning or required dimensions must remain compatible with matrix consumers. Invalid status values stay fail-closed.

`__init__.py` là export lazy. `operations.py`, `dimensions.py`, `ownership.py`, `status.py` mỗi file giữ một phần contract. Đổi nghĩa phải tương thích consumer; status lạ tiếp tục fail-closed.
