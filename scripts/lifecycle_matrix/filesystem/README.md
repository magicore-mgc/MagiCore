# Filesystem evidence / Evidence filesystem

`__init__.py` is the lazy export surface. `filesystem.py`, `lockfiles.py` and `markers.py` own directory, lockfile and installed-tree evidence. Missing or empty artifacts remain unverified; read/parse failures are not converted to passes.

`__init__.py` là export lazy. `filesystem.py`, `lockfiles.py`, `markers.py` xử lý evidence thư mục, lockfile và cây cài. Thiếu/empty artifact là unverified; lỗi đọc/parse không biến thành pass.
