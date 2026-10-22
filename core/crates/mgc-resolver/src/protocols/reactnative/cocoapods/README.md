# CocoaPods protocol / Protocol CocoaPods

This owner exposes `CocoaPodsProtocol`, `VerifiedPodSpec`, the URL helper, and dependency extraction behind the existing `protocols::reactnative` path. Construction/configuration, network verification, URL construction, and JSON edge extraction have separate implementation owners.

Owner này cung cấp `CocoaPodsProtocol`, `VerifiedPodSpec`, helper URL và trích cạnh dependency qua path `protocols::reactnative` hiện hành. Khởi tạo/cấu hình, xác minh mạng, tạo URL và trích cạnh JSON có implementation owner riêng.

The client maps its local typed failures to the same `MgError` categories and English diagnostics. It still reads the response body before checking HTTP status, preserving baseline ordering.

Client ánh xạ lỗi typed cục bộ về đúng nhóm `MgError` và thông báo tiếng Anh hiện hành. Client vẫn đọc response body trước khi kiểm tra HTTP status để giữ thứ tự baseline.
