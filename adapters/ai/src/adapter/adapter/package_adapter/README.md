# AI package adapter

Owns only the AI PackageAdapter forwarding implementation and module wiring. Responsibility owners cover capability selection, native-update policy, manifest identity/parsing, installed inventory, add preparation and age-gate delegation.

The native Python engine remains the implementation for native Python lifecycle operations. Foreign Python locks remain fail-closed. Shared capability errors and MgResult remain the error contract; this slice adds no local error layer.

## Chủ sở hữu

Chỉ sở hữu module wiring và implementation chuyển tiếp PackageAdapter của AI. Các owner riêng phụ trách chọn capability, chính sách update native, identity/parse manifest, danh sách package đã cài, chuẩn bị add và ủy quyền cổng tuổi.

Python engine native tiếp tục thực thi lifecycle Python native. Lock Python ngoài vẫn fail-closed. Capability error và MgResult dùng chung tiếp tục là hợp đồng lỗi; lát cắt này không thêm error layer cục bộ.
