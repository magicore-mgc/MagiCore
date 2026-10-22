//! Remove commas directly before closing JSON arrays or objects.
//! Bỏ dấu phẩy ngay trước token đóng array hoặc object JSON.

pub(in crate::jsonc) fn strip_trailing_commas(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut in_string = false;
    let mut escaped = false;

    for (index, character) in input.char_indices() {
        if in_string {
            output.push(character);
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
            continue;
        }

        if character == '"' {
            in_string = true;
            output.push(character);
        } else if character == ',' {
            let next = input[index + 1..]
                .chars()
                .find(|next| !is_json_whitespace(*next));
            if !matches!(next, Some(']' | '}')) {
                output.push(character);
            }
        } else {
            output.push(character);
        }
    }
    output
}

fn is_json_whitespace(character: char) -> bool {
    matches!(character, ' ' | '\t' | '\r' | '\n')
}

#[cfg(test)]
#[path = "test/trailing_comma.rs"]
mod tests;
