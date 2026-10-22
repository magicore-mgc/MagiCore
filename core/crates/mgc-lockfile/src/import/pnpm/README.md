# pnpm lockfile parser

Parses every YAML document, selects the final project graph, reports packages in earlier environment documents, and reconstructs edges only to known exact versions.

Parse toàn bộ YAML document, chọn graph dự án ở document cuối, báo package ở document môi trường trước đó và chỉ dựng cạnh đến version exact đã biết.

The format warning is advisory and peer suffix handling stays in the shared package-reference owner.

Cảnh báo format chỉ mang tính thông tin; xử lý hậu tố peer nằm trong owner package-reference dùng chung.
