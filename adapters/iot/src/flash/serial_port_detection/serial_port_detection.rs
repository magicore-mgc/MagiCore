//! Enumerates serial-device candidates without flashing firmware.
//! Liệt kê candidate cổng serial mà không flash firmware.

use std::path::Path;

const SERIAL_DEVICE_PREFIXES: &[&str] = &["ttyUSB", "ttyACM", "tty.usb", "cu.usb"];

/// Scans one directory for known serial-device filename prefixes and sorts the paths.
/// Quét prefix tên file serial đã biết trong một thư mục và sắp xếp path.
pub fn detect_serial_ports_in(base: &Path) -> Vec<String> {
    let mut ports = Vec::new();
    let Ok(entries) = std::fs::read_dir(base) else {
        return ports;
    };
    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let Some(name) = file_name.to_str() else {
            continue;
        };
        if SERIAL_DEVICE_PREFIXES
            .iter()
            .any(|prefix| name.starts_with(prefix))
        {
            ports.push(base.join(&file_name).to_string_lossy().into_owned());
        }
    }
    ports.sort();
    ports
}

/// Enumerates serial ports using the current operating-system implementation.
/// Liệt kê cổng serial theo triển khai hệ điều hành hiện tại.
pub fn detect_serial_ports() -> Vec<String> {
    #[cfg(target_os = "windows")]
    {
        #[cfg(feature = "windows-serial")]
        {
            match serialport::available_ports() {
                Ok(ports) => ports.into_iter().map(|port| port.port_name).collect(),
                Err(_) => Vec::new(),
            }
        }
        #[cfg(not(feature = "windows-serial"))]
        {
            Vec::new()
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        detect_serial_ports_in(Path::new("/dev"))
    }
}
