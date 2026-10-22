# Lifecycle App Linux prerequisites / Prerequisite Linux cho App lifecycle

`install_linux_build_dependencies.sh` installs the existing Ubuntu Flutter build packages with apt. It is called only by the `ubuntu-24.04` workflow step; Flutter SDK setup remains an action and its dependency warmup remains a separate MGC-guarded step.

`install_linux_build_dependencies.sh` cài các gói build Flutter Ubuntu hiện tại bằng apt. Script chỉ được gọi ở step `ubuntu-24.04`; setup Flutter SDK vẫn dùng action và bước warmup dependency vẫn tách riêng dưới MGC process guard.
