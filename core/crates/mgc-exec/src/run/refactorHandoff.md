# `mgc-exec::run` refactor handoff / Bàn giao refactor `mgc-exec::run`

This note records the current structural state and gives the next implementer concrete continuation rules. / Tài liệu này ghi trạng thái cấu trúc hiện tại và quy định cách người tiếp theo tiếp tục.

## Status / Trạng thái

**Not complete.** The worktree contains a broad `run/` decomposition, but the whole repository refactor is still in progress. `run/run.rs` is still 737 lines and owns several unrelated policy and platform helpers. No tests, builds, formatting, metadata checks, CI, or final audit loops have been run for this structural pass; record them as **NOT_RUN**, not PASS. This is deliberate: the user instructed that structural refactoring comes first and verification follows after the full refactor.

**Chưa xong.** Worktree hiện có nhiều owner folder trong `run/`, nhưng refactor toàn repository vẫn đang tiếp diễn. `run/run.rs` còn 737 dòng và vẫn giữ nhiều policy/helper nền tảng khác trách nhiệm. Chưa chạy test, build, format, metadata, CI hay các vòng audit cuối cho đợt refactor cấu trúc này; phải ghi **NOT_RUN**, không được ghi PASS. Đây là thứ tự người dùng yêu cầu: hoàn tất refactor cấu trúc trước rồi mới kiểm tra.

## Work recorded so far / Những phần đã ghi nhận

The following files and owner folders exist in the current worktree. Treat these as implementation in progress until source review and deferred verification confirm them; folder presence alone does not prove behavior is correct.

Các folder/file dưới đây đang có trong worktree. Xem chúng là phần triển khai đang tiếp tục cho đến khi review source và lượt kiểm tra được hoãn xác nhận; chỉ thấy folder không chứng minh hành vi đúng.

- The runner facade and stable exports are in `mod.rs`; the main orchestration implementation is `run.rs`.
- Public execution request/report types moved to `types/`; capture-mode project binary entry points to `project_binary/`.
- Environment preparation, child spawn, process isolation, supervision, shutdown, timeout, termination, output, runtime argv parsing, Rust compiler child checks, and argument validation have dedicated owner folders.
- Process-tree scanning is separated by Linux, macOS, and Windows owners; Windows Job Object inspection, root-exit handling, and spawn command preparation have their own folders.
- Process policy, process-monitor data, compatibility audit writing, and supervision diagnostics have separate owners/types/errors.
- S48 started the dependency-resolution policy split: shared parsing/typed denial plus Cargo, Rustc, Go, Python, .NET, Flutter/Dart, Swift, Gradle/Maven, and Zig language folders. The latest work moved broad runner policy tests to nearest owners and preserved a compatibility facade for the Go toolchain probe. This batch is **IN_PROGRESS**, not accepted.
- Bilingual READMEs, owner-local errors/types, and focused test source files were added across many owners. Some owners still need structural completeness review; do not assume every folder already has all required docs, error/type ownership, or registered tests.

- Runner facade và export ổn định nằm trong `mod.rs`; orchestration chính nằm trong `run.rs`.
- Type request/report public được chuyển vào `types/`; entry chạy binary project ở capture mode được chuyển vào `project_binary/`.
- Chuẩn bị environment, spawn child, process isolation, supervision, shutdown, timeout, termination, output, parse argv runtime, kiểm tra Rust compiler child và validate argument có owner folder riêng.
- Quét process tree được tách theo Linux, macOS, Windows; Windows Job Object inspection, root-exit và chuẩn bị lệnh spawn cũng có folder riêng.
- Process policy, dữ liệu process monitor, ghi compatibility audit và supervision diagnostics có owner/type/error riêng.
- S48 bắt đầu tách dependency-resolution policy: parser dùng chung và typed denial cùng các folder Cargo, Rustc, Go, Python, .NET, Flutter/Dart, Swift, Gradle/Maven, Zig. Phần mới nhất chuyển test policy khỏi runner test lớn về owner gần nhất và giữ facade tương thích cho Go toolchain probe. Batch này đang **IN_PROGRESS**, chưa được chấp nhận.
- Nhiều owner đã có README song ngữ, error/type cục bộ và source test tập trung. Vẫn phải rà tính đầy đủ cấu trúc ở từng owner; không mặc định folder nào cũng đã có đủ docs, ownership error/type hay test được đăng ký.

Relevant progress records: `docs/specs/wholeprojectrefactor/tracking/workboard.md` and batches `S38` through `S48`. The latest S48 record is `docs/specs/wholeprojectrefactor/tracking/batches/S48-dependency-policy-language-owners.md`. Project-wide design rules are in `docs/specs/wholeprojectrefactor/refactorVN.md`, `fileDecompositionVN.md`, `engineStructureVN.md`, `reviewProtocolVN.md`, and `verificationVN.md`. Read `RULE.md` and the approved requirements before changing source.

Hồ sơ tiến độ liên quan nằm tại `docs/specs/wholeprojectrefactor/tracking/workboard.md` và batch `S38` đến `S48`. Batch S48 mới nhất là `docs/specs/wholeprojectrefactor/tracking/batches/S48-dependency-policy-language-owners.md`. Quy tắc thiết kế toàn dự án nằm trong `docs/specs/wholeprojectrefactor/refactorVN.md`, `fileDecompositionVN.md`, `engineStructureVN.md`, `reviewProtocolVN.md`, và `verificationVN.md`. Đọc `RULE.md` cùng yêu cầu đã được duyệt trước khi sửa source.

## Required structure for each owner / Cấu trúc bắt buộc cho từng owner

Apply this pattern to `run/` and other folders in the approved refactor scope. It is a responsibility-based shape, not a mandate to create empty folders:

Áp dụng quy chuẩn này cho `run/` và các folder khác trong phạm vi refactor đã duyệt. Đây là cấu trúc theo trách nhiệm, không phải yêu cầu tạo folder rỗng:

```text
<owner>/
├── mod.rs                 # declarations and stable re-exports only
├── <owner>.rs             # implementation for this responsibility
├── error.rs               # only when this owner has distinct errors
├── types/                 # only when this owner owns reusable/local types
│   ├── mod.rs
│   └── types.rs
├── test/                  # nearest focused tests, registered explicitly
│   ├── README.md
│   └── <subject>.rs
├── docs/                  # local checklist/progress when the task requires it
└── README.md              # purpose, boundaries, call path, platform rules
```

- Keep `mod.rs` as the module declaration/export surface; put implementation in `<owner>.rs` or a smaller responsibility-named child file. Do not let `mod.rs` become a second implementation file.
- Create a child owner only when it owns real behavior/configuration. Never leave a placeholder folder, empty source, or test-only production folder.
- Put each meaningful test next to the behavior owner, keep bodies out of production files, and register every test module/target explicitly. Tests should cover behavior and edge cases, not just mirror a function.
- Give a folder its own `error.rs` when its failures have distinct meaning/recovery. Use typed variants, keep source chains, redact sensitive data, and never turn an error into success. Put non-shared types with their owner; promote only proven shared types.
- Each README states purpose, public/internal entry points, callers/dependencies, ownership boundaries, OS/language cfg behavior, and what must remain compatible. Keep progress/checklist logs in the owner `docs/` when required by `RULE.md`.
- Keep stable external paths/re-exports while moving internals. Preserve behavior, security checks, cfg conditions, error wording/contracts, and caller visibility. Do not use a large “utils” bucket or one universal core for unrelated language/OS behavior.
- Maintain parity across `web`, `ai`, `app`, and `lib` where the change introduces a shared CLI/scaffold/test/quality gate; explain a genuinely local scope in its batch record.

- Giữ `mod.rs` làm bề mặt khai báo/export; implementation nằm trong `<owner>.rs` hoặc file con có tên đúng trách nhiệm. Không biến `mod.rs` thành file implementation thứ hai.
- Chỉ tạo owner con khi nó sở hữu behavior/configuration thật. Không để folder placeholder, source rỗng hay folder production chỉ chứa test.
- Mỗi test có ý nghĩa nằm gần owner behavior, không đặt body test trong file production và phải đăng ký module/target rõ ràng. Test phải kiểm tra hành vi/edge case, không chỉ lặp lại implementation.
- Tạo `error.rs` riêng nếu lỗi của owner có ý nghĩa/khả năng recovery riêng. Dùng variant typed, giữ source chain, che dữ liệu nhạy cảm và không biến lỗi thành success. Type chưa dùng chung ở lại owner; chỉ đưa lên shared sau khi có bằng chứng dùng chung.
- README ghi mục đích, entry point public/internal, caller/dependency, ranh giới ownership, khác biệt OS/ngôn ngữ và điều kiện tương thích cần giữ. Khi `RULE.md` yêu cầu, checklist/progress nằm trong `docs/` của owner.
- Giữ nguyên đường dẫn/export ổn định khi chuyển nội bộ. Bảo toàn behavior, kiểm tra bảo mật, cfg, hợp đồng lỗi và visibility của caller. Không gom trách nhiệm khác nhau vào “utils” hay một core phổ quát.
- Giữ parity cho `web`, `ai`, `app`, `lib` khi thay đổi thêm CLI/scaffold/test/quality gate dùng chung; nếu scope chỉ cục bộ thì ghi lý do trong batch.

## Continue `run/` first / Tiếp tục `run/` trước

1. Read `RULE.md`, this handoff, the approved whole-project refactor specifications, `workboard.md`, and the S48 batch. Confirm the current branch/worktree and preserve inherited dirty files; do not reset, clean, or overwrite user changes.
2. Continue the structural pass in `run/`. Review `run/run.rs` function-by-function and assign each remaining behavior to a real owner before moving it. Current candidates still visible in the runner include manifest exceptions, self-answered reports, compatibility policy, forbidden script-file checks, Unix npm shadow-probe recognition, and audit-entry construction. Also review the remaining broad Unix-session and Windows executable-hash tests for nearest-owner placement. Do not move orchestration merely to reduce line count.
3. Finish S48 migration and source review: confirm all previous language decisions/reasons and caller signatures are preserved; check visibility/cfg and explicit test registration; reconcile the S48 workboard/batch status. The stale GitNexus findings for this area were UNKNOWN, so direct caller mapping and risk remain open until a fresh graph is available.
4. Inspect every owner folder for real implementation, `mod.rs` facade, nearest tests, README, and local errors/types where warranted. Fix structural omissions before expanding to the next crate/core. Do not mark S38–S48 accepted just because files exist.
5. Keep all checks deferred until the user-requested structural pass is complete. Then run the prescribed verification and two review loops from `RULE.md`/`AGENTS.md`, including tests for affected OS lanes, and report each result as PASS/FAIL/NOT_RUN with commands and evidence. Never claim product support from an unexecuted test file.
6. Append a changelog entry after each completed edit as `AGENTS.md` requires. Internal docs and changelog stay local-only: do not stage, commit, or push docs without explicit approval. Do not commit/push anything without user instruction.

1. Đọc `RULE.md`, handoff này, spec refactor toàn dự án đã duyệt, `workboard.md` và batch S48. Xác nhận branch/worktree hiện tại và giữ nguyên file dirty có sẵn; không reset, clean hoặc ghi đè thay đổi của user.
2. Tiếp tục lượt cấu trúc trong `run/`. Rà từng function còn lại trong `run/run.rs` và xác định owner có trách nhiệm thật trước khi di chuyển. Các ứng viên còn nhìn thấy gồm manifest exception, self-answered report, compatibility policy, kiểm tra forbidden script file, nhận diện Unix npm shadow-probe và tạo audit entry. Rà vị trí owner gần nhất cho test Unix session và Windows executable hash còn trong test runner rộng. Không chuyển orchestration chỉ để giảm số dòng.
3. Hoàn tất migration và source review S48: xác nhận mọi quyết định/reason theo ngôn ngữ cùng chữ ký caller được bảo toàn; kiểm tra visibility/cfg và đăng ký test tường minh; cập nhật đúng trạng thái workboard/batch S48. Kết quả GitNexus cũ của khu vực này là UNKNOWN nên cần giữ trạng thái rủi ro cho tới khi có graph mới.
4. Rà từng owner xem có implementation thật, facade `mod.rs`, test gần nhất, README và error/type riêng khi phù hợp. Sửa thiếu sót cấu trúc trước khi mở rộng sang crate/core khác. Không đánh dấu S38–S48 là accepted chỉ vì file đã tồn tại.
5. Tiếp tục hoãn toàn bộ check đến khi xong lượt cấu trúc theo yêu cầu user. Sau đó chạy verification và hai vòng review theo `RULE.md`/`AGENTS.md`, gồm test các OS lane bị ảnh hưởng; báo từng kết quả PASS/FAIL/NOT_RUN kèm lệnh và bằng chứng. Không tuyên bố hỗ trợ dựa trên file test chưa chạy.
6. Sau mỗi lần sửa hoàn tất, append changelog theo `AGENTS.md`. Docs/changelog nội bộ local-only: không stage, commit hoặc push docs nếu chưa được duyệt rõ. Không commit/push nếu user chưa yêu cầu.

## Risks and open items / Rủi ro và việc còn mở

- The structural diff is not verified by a compiler or tests; API visibility, module paths, platform-only imports, and test registrations may still be wrong.
- GitNexus was stale and returned CRITICAL/UNKNOWN/partial results for several high-fanout runner symbols in earlier batches. A zero/empty result was not treated as safety evidence; source callers were searched, but graph risk stays unresolved.
- The runner is not yet “small” or fully owner-oriented. `run.rs` remains the main extraction queue; the entire repository is much larger and is not complete.
- Local SSD ceiling previously blocked Rust verification. Do not fill storage with duplicate target/cache directories; follow the user's storage constraint and use the agreed CI runner after structural work is complete.

- Diff cấu trúc chưa được compiler/test xác nhận; visibility API, đường dẫn module, import chỉ dành cho OS và đăng ký test có thể còn sai.
- GitNexus đang stale và ở các batch trước trả CRITICAL/UNKNOWN/partial cho nhiều symbol có caller rộng. Không xem kết quả rỗng là bằng chứng an toàn; đã rà source caller nhưng rủi ro graph vẫn chưa giải quyết.
- Runner chưa nhỏ và chưa hoàn toàn theo owner; `run.rs` còn là hàng đợi tách tiếp. Toàn repository lớn hơn nhiều và chưa xong.
- Giới hạn SSD trước đó đã chặn xác minh Rust. Không tạo nhiều target/cache trùng lặp; giữ giới hạn dung lượng của user và dùng CI đã thống nhất sau khi xong cấu trúc.

## Handoff summary / Tóm tắt bàn giao

The user has now directly assigned the current assistant to lead and implement the full refactor. Continue from the current dirty worktree and S48/S49, finish `run/` structurally using this exact ownership pattern, then apply the same pattern systematically across the user-approved repository scope. Verification is explicitly a later phase. This document is a progress/handoff record, not a completion claim or release approval.

User đã giao trực tiếp assistant hiện tại làm leader và thực thi toàn bộ refactor. Tiếp tục trên worktree dirty hiện tại từ S48/S49, hoàn tất cấu trúc `run/` theo đúng ownership pattern này, rồi áp dụng có hệ thống cho phạm vi repository user đã duyệt. Verification là phase sau. Đây là hồ sơ tiến độ/bàn giao, không xác nhận hoàn tất hay duyệt phát hành.
