# Dist-tag owner

EN: Resolves a requested npm dist-tag and checks the concrete version against the active release-age policy.

VI: Resolve npm dist-tag được yêu cầu và kiểm tra version cụ thể theo release-age policy hiện hành.

The feature-disabled branch keeps its existing registry selector route. The Web branch never returns a tag that fails the active age check.

Nhánh tắt Web giữ route selector registry hiện có. Nhánh Web không trả tag nếu version bị age check chặn.
