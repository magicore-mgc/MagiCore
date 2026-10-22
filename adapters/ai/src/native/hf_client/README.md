# Hugging Face client owners / Các owner client Hugging Face

This facade preserves `native::hf_client::{HuggingFaceClient,download_file,get_model_card}` while separating client transport, `ApiClient` operations, pure metadata parsing, artifact endpoints and typed errors. Debug and HTTP diagnostics omit the token and caller-provided API URL.

Facade này giữ `native::hf_client::{HuggingFaceClient,download_file,get_model_card}` và tách transport, operation `ApiClient`, parse metadata thuần, endpoint artifact và lỗi có kiểu. Debug cùng HTTP diagnostic không in token hoặc API URL do caller cung cấp.
