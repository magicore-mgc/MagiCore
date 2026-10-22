# Ignore walker configuration

This owner configures the existing `ignore::WalkBuilder` with the package's source precedence. An existing `.npmignore` disables git/global/exclude sources and becomes the custom ignore file; otherwise the previous `.ignore`, `.gitignore`, global and git-exclude behavior remains enabled.

The helper does not build or traverse the walker and does not own file inclusion lists. Its precedence cases live in this owner's test folder and run under the registered `test_ignore` integration target.

READMEVN: Owner này cấu hình walker theo đúng nguồn ignore hiện hành. Khi có `.npmignore`, các nguồn git bị tắt và file npm được dùng thay thế; nếu không, `.ignore`, `.gitignore`, global và git-exclude giữ nguyên. Test precedence nằm ở owner chọn file cha.
