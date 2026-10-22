# Unix monitor implementation / Triển khai monitor Unix

EN: The stable outer `process_scan::monitor` module wires this implementation. `monitor.rs` contains only the ordered scan coordinator. The child owners separately implement process snapshot acquisition, PID identity and zombie traversal, observed process groups, root-exit recovery, descendant refresh, Go environment revalidation, forbidden-process selection and npm probe authentication/argv matching. The scan order and root-visibility downcast contract are retained.

VI: Module `process_scan::monitor` bên ngoài nối implementation này. `monitor.rs` chỉ điều phối scan theo đúng thứ tự. Các owner con tách riêng việc lấy snapshot, identity PID và duyệt zombie, process group đã quan sát, khôi phục sau khi root thoát, refresh descendant, xác minh lại environment Go, chọn process bị cấm và xác thực probe npm/so khớp argv. Thứ tự scan và contract downcast root-visibility được giữ nguyên.

Executable checks remain deferred until the source-first repository pass closes and the machine disk-use limit is met.

Các kiểm tra thực thi được hoãn đến khi hoàn tất lượt refactor source toàn repository và đáp ứng giới hạn dung lượng máy.
