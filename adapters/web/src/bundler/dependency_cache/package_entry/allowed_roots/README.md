# Installed dependency roots

This owner builds the canonical root allowlist used by the dependency-bundle provenance gate. It includes direct package directories (including scoped packages and workspace symlink targets) and known package-manager stores (`.pnpm`, `.bun`, `.store`); it deliberately excludes the parent `node_modules` directory and unrelated hidden folders. It reads directory metadata only; the bundle validator checks esbuild's reported inputs against these roots before returning or caching output.

Owner này lập allowlist canonical cho gate nguồn input dependency bundle. Nó gồm thư mục package trực tiếp (kể cả package scoped, đích symlink workspace) và store package manager đã biết (`.pnpm`, `.bun`, `.store`); chủ đích loại thư mục cha `node_modules` và hidden folder không liên quan. Owner chỉ đọc metadata thư mục; validator bundle đối chiếu input esbuild với các root này trước khi trả hoặc cache output.
