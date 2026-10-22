# Metafile document reader

This owner streams the esbuild metafile object, requires exactly one non-empty `inputs` map, ignores unrelated metadata without materializing a JSON tree, and delegates each path to the input-path validator.

Owner này đọc streaming object metafile esbuild, yêu cầu đúng một `inputs` map không rỗng, bỏ qua metadata khác mà không tạo cây JSON trong bộ nhớ và giao từng path cho validator input-path.
