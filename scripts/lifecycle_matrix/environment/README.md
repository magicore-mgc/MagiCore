# Runtime environments / Môi trường runtime

`__init__.py` is the lazy export surface. Shared sandbox setup is separated from Python provisioning, Swift, Flutter, Pulumi and recovery variables. Runtime-specific cache/home paths must stay isolated below the lane sandbox.

`__init__.py` là export lazy. Tách sandbox chung khỏi Python, Swift, Flutter, Pulumi và biến recovery. Cache/home riêng từng toolchain phải nằm dưới sandbox của lane.
