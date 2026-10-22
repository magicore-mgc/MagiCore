# Serial-port detection

EN: Detects candidate device paths only. Unix-like systems scan `/dev`; Windows uses the optional `windows-serial` feature and the `serialport` crate. The directory helper treats unreadable/missing directories as an empty result. None of these functions flashes a device.

VI: Chỉ nhận diện path thiết bị candidate. Hệ Unix quét `/dev`; Windows dùng feature tùy chọn `windows-serial` và crate `serialport`. Helper quét thư mục trả danh sách rỗng khi thư mục thiếu/không đọc được. Các hàm này không flash thiết bị.
