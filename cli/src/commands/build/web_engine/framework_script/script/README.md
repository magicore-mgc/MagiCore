# Framework launch mapping / Ánh xạ launch framework

Maps the supported Node/framework commands and direct Bun/Deno compatibility lanes to concrete executables/arguments. Syntax-check source paths must be regular files within the canonical project root. No package manager or shell fallback is allowed.

Map command Node/framework được hỗ trợ và lane compatibility Bun/Deno trực tiếp thành executable/arguments. Source syntax-check phải là file thường trong canonical project root. Không fallback sang package manager hoặc shell.

Invalid/outside paths return unsupported mapping; shared command errors propagate. Unit assertions remain in the CLI test module until tests are relocated later.

Path sai/ngoài project trả về mapping không hỗ trợ; lỗi command dùng chung được propagate. Assertion unit còn ở module test CLI tới lúc chuyển test sau.
