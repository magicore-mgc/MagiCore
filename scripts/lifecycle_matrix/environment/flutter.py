"""Flutter SDK and pub cache paths."""

from __future__ import annotations

import os

def flutter_sdk_pub_cache_directory(
    host_environment: dict[str, str],
) -> str:
    """Resolve the job-local cache warmed for Flutter's SDK tool.
    (Tìm cache riêng của job đã dùng để làm nóng công cụ Flutter SDK.)"""
    configured = host_environment.get("PUB_CACHE")
    if configured:
        return os.path.abspath(configured)
    job_home = host_environment.get("FLUTTER_SDK_HOME")
    if job_home:
        return os.path.join(os.path.abspath(job_home), ".pub-cache")
    # Never substitute the developer/runner account cache for an isolated lane.
    # Không bao giờ thay cache riêng của lane bằng cache tài khoản developer/runner.
    return ""

def flutter_sdk_home_directory(host_environment: dict[str, str]) -> str:
    """Resolve the job-local HOME shared by Flutter warm and runtime steps.
    (Tìm HOME riêng của job dùng chung cho bước warm và Flutter runtime.)"""
    configured = host_environment.get("FLUTTER_SDK_HOME")
    if configured:
        return os.path.abspath(configured)
    # Local matrix runs without the CI warm step keep their existing lane HOME.
    # Chạy matrix local không có bước warm CI thì giữ HOME riêng hiện tại của lane.
    return ""

__all__ = ['flutter_sdk_home_directory', 'flutter_sdk_pub_cache_directory']
