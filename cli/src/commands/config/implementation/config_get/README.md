# Config get / Đọc config

## Responsibility / Trách nhiệm

Implements key lookup and environment normalization. Source precedence remains env, nearest mgc.toml, project npmrc, then user npmrc. Baseline has no direct source-priority test.

## Tiếng Việt

Đọc key và chuẩn hóa biến môi trường. Thứ tự nguồn giữ nguyên: env, mgc.toml gần nhất, npmrc project, rồi npmrc user. Baseline chưa có test trực tiếp cho thứ tự này.
