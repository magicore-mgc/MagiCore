//! Acquire visible usernames and bounded password input.
//! Nhận username hiển thị và password input có giới hạn.

use anyhow::Result;
use std::io::{BufRead, Read, Write};

use super::error::CredentialInputError;

const MAX_PASSWORD_STDIN_BYTES: usize = 8 * 1024;

pub(in crate::commands::login) fn prompt_text(label: &str) -> Result<String> {
    print!("{}: ", label);
    std::io::stdout().flush()?;
    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line)?;
    Ok(line.trim().to_string())
}

pub(crate) fn read_password(password_stdin: bool, label: &str) -> Result<String> {
    read_password_inner(password_stdin, label).map_err(super::credential_input_diagnostic)
}

pub(in crate::commands::login) fn read_password_inner(
    password_stdin: bool,
    label: &str,
) -> Result<String, CredentialInputError> {
    let password = if password_stdin {
        let mut value = String::new();
        std::io::stdin()
            .take((MAX_PASSWORD_STDIN_BYTES + 1) as u64)
            .read_to_string(&mut value)?;
        parse_password_stdin(value)?
    } else {
        let value = mgc_ui::prompt::password(label).map_err(CredentialInputError::Prompt)?;
        require_non_empty(value)?
    };
    Ok(password)
}

fn parse_password_stdin(value: String) -> Result<String, CredentialInputError> {
    if value.len() > MAX_PASSWORD_STDIN_BYTES {
        return Err(CredentialInputError::TooLarge);
    }
    require_non_empty(value.trim_end_matches(['\r', '\n']).to_owned())
}

fn require_non_empty(password: String) -> Result<String, CredentialInputError> {
    if password.is_empty() {
        return Err(CredentialInputError::Empty);
    }
    Ok(password)
}

#[cfg(test)]
#[path = "test/credentials.rs"]
mod tests;
