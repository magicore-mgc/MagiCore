# Wait loop

Owns the ordering of cancellation, Unix pre-exit inspection, root-exit handling, active platform inspection, timeout enforcement, and poll sleep.

The coordinator must preserve the existing order: cancellation first; Unix pre-exit scan; root status; active process scan; timeout; poll interval. Platform behavior is delegated to its owner.

# Vòng chờ

> Owner này quản lý thứ tự cancellation, Unix pre-exit scan, xử lý root exit, active scan theo OS, timeout và khoảng poll.

> Coordinator giữ thứ tự hiện tại: cancellation trước; Unix pre-exit scan; root status; active process scan; timeout; poll interval. Hành vi riêng theo nền tảng được giao đúng owner.
