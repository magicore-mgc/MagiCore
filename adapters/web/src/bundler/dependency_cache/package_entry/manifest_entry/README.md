# Manifest entry selection

This owner reads `package.json`, preserves browser → module → main precedence, normalizes relative entry strings and keeps selected entries inside the canonical package root. Missing or malformed manifests retain the `index.js` fallback.

Owner này đọc `package.json`, giữ thứ tự browser → module → main, chuẩn hóa chuỗi entry tương đối và giới hạn entry trong canonical package root. Manifest thiếu hoặc sai định dạng vẫn fallback `index.js`.
