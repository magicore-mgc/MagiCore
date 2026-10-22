# AI Python container lane

`build_python_container.sh` creates the AI project through MGC, installs Python packages through MGC, checks the lock/core/Dockerfile/Compose contract, builds and runs the image, and reuses that image in Compose.

`build_python_container.sh` tạo dự án AI qua MGC, cài Python package bằng MGC, kiểm tra hợp đồng lock/core/Dockerfile/Compose, build/chạy image rồi tái sử dụng image đó trong Compose.

Keep `pytest` as an MGC-owned project dependency. The image name consumed by `MGC_IMAGE_NAME` must match the preceding `docker build --tag`; do not introduce a package-manager bypass.
