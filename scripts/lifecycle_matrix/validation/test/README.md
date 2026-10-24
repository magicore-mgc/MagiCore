# Lifecycle validation tests / Test kiểm định lifecycle

`test_*.py` owns one lifecycle-matrix behavior: environment/toolchain setup, native package-manager verdicts, framework inventory, provenance, platform-green evidence, working-tree cleanliness, or Windows runtime contracts. `support.py` contains only shared imports, repository path roots, and synthetic fixtures.

Each file keeps its test method names and assertions. The stable GitHub entrypoint remains `python scripts/test/test_native_pm_verdict.py`; the same modules can be discovered with `python -m unittest discover -s scripts/lifecycle_matrix/validation/test -t scripts -p 'test_*.py'`. Runtime verification of this move is pending.

Mỗi file `test_*.py` sở hữu một hành vi của lifecycle matrix: chuẩn bị environment/toolchain, verdict package manager native, inventory framework, provenance, platform-green, trạng thái working tree hoặc hợp đồng runtime Windows. `support.py` chỉ giữ import/helper/path root và fixture tổng hợp dùng chung.

Tên test method và assertion được giữ nguyên. Entrypoint GitHub ổn định vẫn là `python scripts/test/test_native_pm_verdict.py`; có thể discover các module bằng lệnh trong phần tiếng Anh. Chưa chạy xác minh runtime cho lần di chuyển này.

## Source review status / Trạng thái rà source

The source-only review found 20 focused modules containing the original 143 test methods. Every method name and normalized source fingerprint matches the frozen pre-move baseline; the compatibility runner binds all 20 owner classes once, keeps `unittest.main(verbosity=2)`, and the workflow command remains unchanged. The three reviewed process-call fingerprints and the shared `bootstrap_bash_command` fingerprint still match their source policy records. These are static checks only. Runtime tests, Python imports/compilation, process-audit execution, workflow, and CI remain NOT_RUN.

Rà source tĩnh xác nhận 20 module tập trung chứa đủ 143 test method gốc. Tên và fingerprint source đã chuẩn hóa của từng method khớp baseline trước khi chuyển; runner tương thích gắn đủ 20 class đúng một lần, giữ `unittest.main(verbosity=2)`, và lệnh workflow không đổi. Ba fingerprint process-call cùng fingerprint helper `bootstrap_bash_command` vẫn khớp policy source. Đây chỉ là kiểm tra tĩnh; test runtime, import/compile Python, chạy process audit, workflow và CI vẫn NOT_RUN.
