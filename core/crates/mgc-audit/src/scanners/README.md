# Shared audit scanners / Scanner audit dùng chung

- Purpose: Exports the stable scanner API and routes parser/input owners; ecosystem scanners remain sibling modules.
- Mục đích: Giữ API scanner ổn định và export các owner parser/input; scanner theo hệ sinh thái vẫn là module anh em.


## OSV scanner owners

`osv/` is a wiring facade around the same-name `osv/osv.rs` audit coordinator and small owners for OSV wire types, query materialization, severity, Swift Package.resolved, and CocoaPods inventory. Each behavior owner has a matching implementation file and README. `MgError` / `MgResult` remain the shared error boundary. See `osv/README.md` for the actual owner tree and invariants.

## Owner của scanner OSV

`osv/` là facade nối `osv/osv.rs` cùng tên (điều phối audit) với các owner nhỏ cho wire type OSV, truy vấn, severity, Package.resolved của Swift và kiểm kê CocoaPods. Mỗi owner hành vi có implementation và README cùng tên. `MgError` / `MgResult` vẫn là ranh giới lỗi dùng chung. Xem `osv/README.md` để biết cây owner và invariant thực tế.

## .NET NuGet scanner owners

`dotnet/` preserves the public scanner functions while assigning audit orchestration, lock parsing, solution parsing, canonical path checks, pin collection, and parse errors to separate owners. Project paths outside the selected root and ambiguous case-fold matches are refused; project inputs remain bounded, and the scanner never runs `dotnet restore`. Hermetic owner tests live beside each behavior. See `dotnet/README.md` for the concrete tree; runtime remains unverified until the deferred test gate opens.

## Owner scanner .NET NuGet

`dotnet/` giữ các public scanner function, đồng thời tách điều phối audit, parse lock, parse solution, kiểm tra canonical path, gom pin và lỗi parse thành owner riêng. Path project ra ngoài root đã chọn và kết quả case-fold mơ hồ bị từ chối; project input vẫn có giới hạn kích thước, scanner không chạy `dotnet restore`. Test hermetic nằm cạnh từng hành vi. Xem `dotnet/README.md` để biết cây thực tế; runtime chưa xác minh vì test đang hoãn theo gate.


## Bun/Deno lockfile owners

`bun_deno.rs` preserves the public scanner path and historical `NpmPin` declaration; Bun JSONC parsing, Deno JSON parsing, package-spec splitting, aggregate result types, local parse errors, and project lock aggregation live in documented same-name owners under `bun_deno/`. `read_js_lockfiles` reads all present Bun and Deno lockfiles in that order, does not read `mgc.lock`, and has no production caller found in the current source map. Ten nearest-owner tests are authored but NOT_RUN. See `bun_deno/README.md`.

## Owner lockfile Bun/Deno

`bun_deno.rs` giữ đường dẫn scanner công khai và khai báo `NpmPin` lịch sử; parse JSONC Bun, parse JSON Deno, tách package spec, kiểu kết quả, lỗi parse cục bộ và gộp lock dự án nằm trong các owner cùng tên có tài liệu tại `bun_deno/`. `read_js_lockfiles` đọc mọi lock Bun/Deno hiện có theo thứ tự đó, không đọc `mgc.lock`, và source map hiện không tìm thấy caller production. Mười test gần owner đã viết nhưng NOT_RUN. Xem `bun_deno/README.md`.
