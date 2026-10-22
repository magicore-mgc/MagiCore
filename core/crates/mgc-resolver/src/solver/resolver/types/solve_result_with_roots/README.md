# SolveResultWithRoots

## Responsibility / Trách nhiệm
Returns the ordinary resolver output together with exact direct-root selections. Existing `Resolver::solve` keeps returning `SolveResult`; Web policy callers that need to compare project roots use the additive root-aware method.

Trả kết quả resolver thông thường cùng các lựa chọn root trực tiếp chính xác. `Resolver::solve` hiện có vẫn trả `SolveResult`; caller Web cần so sánh root project dùng method mới có root metadata.

## Tests / Kiểm thử
Nearest tests verify the legacy result projection preserves its resolution list. NOT_RUN.
