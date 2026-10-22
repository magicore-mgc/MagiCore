# Web package launch owner

Maps a project dev script to a process description. It keeps external package-manager rejection before runtime spawn gating, optimizer environment selection, and final launch dispatch. `runtime_commands/` owns Bun/Deno/Node/TSX; `framework_launch/` dispatches to separate Vite, Next, Nuxt, Astro, Remix and Angular owners; `types/` owns the shared immutable launch context.

Ánh xạ dev script của project thành mô tả process. Giữ thứ tự từ chối package manager ngoài, runtime spawn gate, chọn optimizer env rồi dispatch launch. `runtime_commands/` sở hữu Bun/Deno/Node/TSX; `framework_launch/` dispatch tới owner riêng Vite, Next, Nuxt, Astro, Remix và Angular; `types/` sở hữu launch context bất biến dùng chung.
