# Evidence / Bằng chứng

`__init__.py` is the lazy export surface. `source.py` records checkout provenance; `records.py` owns evidence path/schema/release scope; `platform.py` atomically reads/writes consecutive green-run counters. IO/JSON failures preserve their existing fail-closed or explicit error result.

`__init__.py` là export lazy. `source.py` ghi provenance checkout; `records.py` giữ path/schema/scope; `platform.py` quản lý counter xanh liên tiếp theo cách atomic. Lỗi IO/JSON giữ nguyên fail-closed/result hiện tại.
