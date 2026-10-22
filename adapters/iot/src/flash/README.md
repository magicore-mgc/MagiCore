# IoT flash boundary

EN: This module does not implement firmware flashing. `flash_firmware` always returns the existing Unsupported capability while no MagiCore-owned backend exists, and never invokes an external flasher. The only implemented device helper enumerates candidate serial ports. `mod.rs` wires the stable public API; `flash.rs`, `flash_result/` and `serial_port_detection/` own the boundary, result DTO and OS-specific enumeration.

VI: Module này chưa triển khai flash firmware. `flash_firmware` luôn trả capability Unsupported khi chưa có backend do MagiCore sở hữu và không gọi flasher bên ngoài. Helper thiết bị duy nhất hiện có là liệt kê cổng serial candidate. `mod.rs` nối API công khai ổn định; `flash.rs`, `flash_result/` và `serial_port_detection/` lần lượt sở hữu boundary, DTO kết quả và liệt kê theo OS.
