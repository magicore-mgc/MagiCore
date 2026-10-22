# OCI manifest model module

This module defines OCI wire models and Distribution media-type constants. Its `mod.rs` is the compatibility facade: existing `mgc_oci::manifest::*` type paths and `mgc_oci::manifest::media_types::*` remain available while each type family is implemented and tested by a named owner. This layer has no transport, validation policy, or custom fallible operation.

Module này định nghĩa wire model OCI và hằng media type Distribution. `mod.rs` là facade tương thích: giữ nguyên các đường dẫn `mgc_oci::manifest::*` và `mgc_oci::manifest::media_types::*`, còn mỗi họ type được triển khai/kiểm thử tại owner riêng. Tầng này không xử lý transport, policy validation hay thao tác có thể lỗi tùy biến.
