# Compiler options and cache identity / Compiler options và danh tính cache

Owns the esbuild identity/version pins and deterministic options digest, including project-root and logical-source-path cache namespace inputs.

Sở hữu pin danh tính/version esbuild và digest options tất định, gồm namespace cache theo project root và path logic của source.

Every transpile builder option remains represented in the canonical key. Tsconfig content state is delegated to the tsconfig owner.

Mọi option builder transpile tiếp tục được phản ánh trong key chuẩn hóa. Trạng thái nội dung tsconfig do owner tsconfig xử lý.
