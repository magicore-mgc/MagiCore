//! Replaces script host/port values with the endpoint selected by MagiCore.
//! Thay host/port trong script bằng endpoint do MagiCore chọn.

use std::ffi::OsString;

pub(in crate::commands::core::web::implementation) fn replace_dev_endpoint_arg(
    args: &mut Vec<OsString>,
    flag: &str,
    value: OsString,
) {
    let equals_prefix = format!("{flag}=");
    let mut normalized = Vec::with_capacity(args.len() + 2);
    let mut index = 0;

    while index < args.len() {
        let current = args[index].to_string_lossy();
        if current == flag {
            index += 1;
            if index < args.len() && !args[index].to_string_lossy().starts_with('-') {
                index += 1;
            }
        } else if current.starts_with(&equals_prefix) {
            index += 1;
        } else {
            normalized.push(args[index].clone());
            index += 1;
        }
    }

    normalized.push(OsString::from(flag));
    normalized.push(value);
    *args = normalized;
}

pub(in crate::commands::core::web::implementation) fn append_dev_endpoint_args(
    args: &mut Vec<OsString>,
    host_flag: &str,
    port_flag: &str,
    host: Option<String>,
    port: Option<u16>,
) {
    // MGC owns the selected endpoint, so replace script values in both `--flag value` and `--flag=value` forms.
    // MGC sở hữu endpoint đã chọn nên thay giá trị trong cả hai dạng cờ của script.
    if let Some(host) = host {
        replace_dev_endpoint_arg(args, host_flag, OsString::from(host));
    }
    if let Some(port) = port {
        replace_dev_endpoint_arg(args, port_flag, OsString::from(port.to_string()));
    }
}
