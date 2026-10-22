# Python install service

This module is the stable public surface for shared native-Python install helpers used by the Lib adapter and, through existing compatibility facades, AI. `install.rs` preserves those function names and translates owner-local errors back to the existing `MgError` variants. It does not own download/install orchestration or mutate the shared package store.

`distribution_name/` handles PEP 503 name comparison; `external_manifest/` blocks fallback when a foreign Python manager owns dependency metadata; `core_owner/` reads project identity and filters mixed-core lock entries; `artifact_cache/` opens and digest-checks cached artifacts; `runtime_compatibility/` validates the Python site-directory segment and importable wheel lane. Each owner documents its own boundary and nearest tests.

## Trách nhiệm

Đây là bề mặt công khai ổn định cho các helper install Python native dùng chung bởi adapter Lib và AI thông qua facade tương thích hiện tại. `install.rs` giữ nguyên tên hàm và chuyển lỗi local về các variant `MgError` hiện có. Module này không sở hữu orchestration tải/cài hoặc sửa package store dùng chung.

`distribution_name/` đối chiếu tên theo PEP 503; `external_manifest/` chặn fallback khi metadata dependency thuộc manager Python khác; `core_owner/` đọc identity project và lọc lock nhiều core; `artifact_cache/` mở cache và kiểm tra digest; `runtime_compatibility/` xác thực segment thư mục site và lane wheel import được. Mỗi owner có README và test gần behavior.
