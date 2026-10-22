# CLI / Dòng lệnh

`__init__.py` exports the command entrypoint. `main.py` composes `preflight.py`, `collect.py`, `verdicts.py`, `document.py`, `output.py` and `report.py`. These modules preserve the existing flags, gate order, stdout/stderr and process exit semantics.

`__init__.py` export entrypoint. `main.py` ghép `preflight.py`, `collect.py`, `verdicts.py`, `document.py`, `output.py` và `report.py`. Các module giữ nguyên flag, thứ tự gate, stdout/stderr và exit.
