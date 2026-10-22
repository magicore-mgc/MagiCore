# PyPI protocol / Protocol PyPI

Shared native PyPI registry protocol. `pypi/pypi.rs` retains stable `PypiProtocol`; sibling owners isolate request, resolution, metadata, artifact integrity/materialization, wheel/target compatibility, PEP 440/508 and markers. External callers keep importing `protocols::PypiProtocol`. Inline tests remain unchanged until the source-first pass ends, then move beside owners.

Protocol PyPI native dùng chung. `pypi/pypi.rs` giữ `PypiProtocol` ổn định; owner bên cạnh tách request, resolve, metadata, integrity/materialize artifact, tương thích wheel/target, PEP 440/508 và marker. Caller tiếp tục import `protocols::PypiProtocol`. Test inline được giữ nguyên tới khi xong refactor source rồi chuyển về cạnh owner.
