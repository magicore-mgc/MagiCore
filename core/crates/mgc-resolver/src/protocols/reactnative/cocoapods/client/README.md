# CocoaPods HTTP client / Client HTTP CocoaPods

The client owns request ordering, status/body handling, SHA-1 verification, JSON parsing, and the local typed error boundary. It verifies the exact downloaded bytes before parsing the response and maps failures back to the established `MgError` categories and text.

Client sở hữu thứ tự request, xử lý status/body, xác minh SHA-1, parse JSON và boundary lỗi typed cục bộ. Client xác minh đúng byte tải xuống trước khi parse response và ánh xạ lỗi về nhóm `MgError` cùng nội dung hiện hành.

`adapters/app/tests/native_rn_tests.rs` owns the HTTP mock cases: a valid CocoaPods spec resolves, a mismatched checksum fails with `Integrity`, and a missing `Podfile.lock` fails closed. Local tests here pin error mapping. Those integration cases are source evidence only in this batch; they were not run. The existing `rn_layers.rs` consumer remains unchanged.

`adapters/app/tests/native_rn_tests.rs` sở hữu ca HTTP mock: spec CocoaPods hợp lệ được resolve, checksum lệch trả `Integrity`, và thiếu `Podfile.lock` fail-closed. Test cục bộ ở đây khóa ánh xạ lỗi. Các integration case chỉ là bằng chứng source trong batch này, chưa chạy. Consumer `rn_layers.rs` được giữ nguyên.
