# Lib language owner

This owner preserves public language APIs and separates project config, markers, manifest formats, file policy, csproj selection and manifest classification.
Owner này giữ API ngôn ngữ công khai và tách config project, marker, định dạng manifest, chính sách file, chọn csproj và phân loại manifest.

The public Option and bool contracts remain best-effort. There is no local error type because these functions expose no error channel.
Contract Option và bool công khai tiếp tục best-effort. Không tạo error type cục bộ vì các hàm này không trả kênh lỗi.
