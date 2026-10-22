# Deprecation deserialization tests / Test deserialize deprecation

Checks acceptance of boolean/string/null values through `PackageVersion`. The source review has identified that the non-empty string is discarded; these source tests cover input acceptance only and do not claim reason-text preservation. All cases remain `NOT_RUN`.

Kiểm tra `PackageVersion` nhận được giá trị boolean/chuỗi/null. Source review đã xác định chuỗi không rỗng bị loại bỏ; test source này chỉ bao phủ khả năng nhận đầu vào, không khẳng định giữ được lý do. Tất cả case vẫn `NOT_RUN`.
