# External Hash Owner

**Purpose:** Keep the public preflight for externally supplied BLAKE3 text close to its implementation.

**Mục đích:** Đặt preflight public cho chuỗi BLAKE3 từ bên ngoài cạnh phần triển khai.

`validate_external_hash` remains re-exported through `cas::store`; malformed input retains the existing `StoreError::InvalidHash` mapping. The focused source tests cover valid and invalid text.

`validate_external_hash` tiếp tục được export qua `cas::store`; input sai giữ mapping `StoreError::InvalidHash`. Test source tập trung cho chuỗi hợp lệ và không hợp lệ.
