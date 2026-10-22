# Launcher policy errors / Lỗi launcher policy

Owns typed failures for a rejected runtime flag and a rejected Deno permission. `Display` delegates to the existing shared error-message helpers so there is one canonical source for each diagnostic; the public launcher API continues to return `anyhow::Result<()>`. No new policy outcome is introduced.
