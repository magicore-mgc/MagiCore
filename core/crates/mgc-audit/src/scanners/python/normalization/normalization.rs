pub(in crate::scanners::python) fn normalize_python_name(name: &str) -> String {
    let mut normalized = String::with_capacity(name.len());
    let mut separator = false;
    for ch in name.to_ascii_lowercase().chars() {
        if matches!(ch, '_' | '.' | '-') {
            separator = true;
        } else {
            if separator && !normalized.is_empty() {
                normalized.push('-');
            }
            separator = false;
            normalized.push(ch);
        }
    }
    normalized
}
