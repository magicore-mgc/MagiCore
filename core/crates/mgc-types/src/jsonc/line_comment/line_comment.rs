//! Remove line comments outside escaped JSON strings.
//! Bỏ comment dòng nằm ngoài JSON string có xử lý escape.

pub(in crate::jsonc) fn strip_line_comments(raw: &str) -> String {
    let mut output = String::with_capacity(raw.len());
    for line in raw.lines() {
        let mut in_string = false;
        let mut escaped = false;
        let mut comment_start = line.len();

        for (index, character) in line.char_indices() {
            if in_string {
                if escaped {
                    escaped = false;
                } else if character == '\\' {
                    escaped = true;
                } else if character == '"' {
                    in_string = false;
                }
            } else if character == '"' {
                in_string = true;
            } else if character == '/' && line[index..].starts_with("//") {
                comment_start = index;
                break;
            }
        }

        output.push_str(&line[..comment_start]);
        output.push('\n');
    }
    output
}

#[cfg(test)]
#[path = "test/line_comment.rs"]
mod tests;
