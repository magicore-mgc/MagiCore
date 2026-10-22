# Container profile output facade

Wires two real owners while preserving the existing `container_common::*` import path. Docker text and file generation belongs to `docker_assets/`; Web's DNS-safe name used by its image and Kubernetes resources belongs to `web_resource_name/`. Web-specific `.dockerignore` remains owned by the Web profile.

Nối hai owner có trách nhiệm thật và giữ nguyên đường import `container_common::*`. Sinh text/file Docker thuộc `docker_assets/`; tên DNS-safe của Web dùng cho image và resource Kubernetes thuộc `web_resource_name/`. `.dockerignore` riêng Web tiếp tục do profile Web sở hữu.
