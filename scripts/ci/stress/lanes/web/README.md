# Web frontend lifecycle lane / Lane lifecycle frontend Web

`run_frontend_lifecycle.sh` runs the seven dedicated Angular, Astro, Next.js, Vite React, Remix, SvelteKit and Vue/Vite E2Es in sequence. It continues after a failed lane, reports each failure, and exits nonzero if any lane failed. The workflow supplies the frozen CLI, evidence path and ninety-minute timeout.

`run_frontend_lifecycle.sh` chạy lần lượt bảy E2E Angular, Astro, Next.js, React Vite, Remix, SvelteKit và Vue/Vite. Script tiếp tục sau lane lỗi, báo từng lỗi và trả exit khác 0 nếu có lane thất bại. Workflow cung cấp CLI snapshot, evidence path và timeout 90 phút.
