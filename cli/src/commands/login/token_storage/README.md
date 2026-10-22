# Registry token storage

EN: Selects project `.npmrc` for `--local` or the user's `~/.npmrc` otherwise, then delegates scope validation, symlink rejection and file permissions to `mgc-config::NpmRc`.

VI: Owner này chọn `.npmrc` của project khi dùng `--local`, hoặc `~/.npmrc` của user, rồi giao kiểm tra scope, chặn symlink và quyền file cho `mgc-config::NpmRc`.

Do not broaden token scope, print the token, or bypass the shared writer. Tests exercise path selection and missing-home behavior; filesystem side effects remain for the controlled runtime phase.

Không mở rộng scope token, in token hoặc bỏ qua writer dùng chung. Test kiểm tra chọn path và thiếu home; side effect filesystem để phase runtime có kiểm soát.
