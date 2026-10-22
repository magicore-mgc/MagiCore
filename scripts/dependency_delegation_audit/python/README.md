# Python scanners / Scanner Python

`python.py` scans source text; `process_calls.py` matches exact reviewed process-call AST fingerprints; `helper_fingerprint.py` owns stable AST fingerprints; `helper_namespace.py` traces reviewed module aliases; `helper_review.py` verifies helper-boundary reviews. Parsing and stale-review failures remain explicit findings.

`python.py` quét source; `process_calls.py` đối chiếu fingerprint AST process call; `helper_fingerprint.py` tạo fingerprint ổn định; `helper_namespace.py` theo dấu alias module; `helper_review.py` kiểm tra review helper boundary. Lỗi parse và review cũ vẫn là finding rõ ràng.
