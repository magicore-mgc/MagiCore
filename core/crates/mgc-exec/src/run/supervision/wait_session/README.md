# Wait session

Owns one child-process execution session from stream setup through completion, cancellation, inspection, and timeout handling.

## Purpose

This folder replaces the previous monolithic `supervision.rs` implementation while keeping its call contract. `wait_session.rs` owns session state; the loop delegates to focused owners for output capture, cancellation, process scans, root exit, termination, and timeout.

The session preserves bounded output capture, process-tree enforcement, Unix and Windows inspection, fail-closed inspection errors, and the existing `SupervisionError` source chain. Platform-specific behavior stays in the relevant OS owner.

The session passes the root `ExecutionScope` to each Unix scan phase and Windows Job inspection so descendant policy does not silently fall back to a default scope. Only the currently catalogued device tools `adb` and `xcrun` use this descendant check.

## Extension points and tests

Add behavior to the owner that owns its invariant. Keep `mod.rs` limited to module wiring and exports. The existing behavior test remains at `../test/supervision.rs` during the source-only pass; nearest-owner test placement is handled in the later test pass.

# Phiên chờ process

> Folder này quản lý một phiên thực thi process con, từ khởi tạo luồng output đến kết thúc, hủy, kiểm tra cây process và timeout.

`wait_session.rs` giữ trạng thái phiên. Các owner con phụ trách output, cancellation, process scan, root exit, termination và timeout. Giữ nguyên API nội bộ cùng các bảo đảm Unix/Windows và chuỗi lỗi typed hiện có.

Phiên truyền `ExecutionScope` của root tới từng pha scan Unix và kiểm tra Windows Job, tránh để policy descendant âm thầm rơi về scope mặc định. Chỉ hai công cụ thiết bị đang có trong catalog là `adb` và `xcrun` được kiểm tra theo cách này.

Thêm behavior vào đúng owner sở hữu invariant; `mod.rs` chỉ wiring/export. Test gần nhất đang giữ nguyên trong `../test/supervision.rs` cho lượt chỉ refactor source này; lượt test sau sẽ đặt assertion sát owner.
