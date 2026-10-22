# Source validation

Owns no-follow root/entry checks, complete preflight before destination creation, strict lexical confinement for relative links, canonical resolution/containment for absolute and relative links, and source-file open checks. It rejects external, dangling, unsupported, path-escaping, and target-resolution-cycle links; recursive directory links to an ancestor are also refused.

# Xác thực source

Sở hữu kiểm tra root/entry no-follow, preflight toàn cây trước khi tạo đích, giới hạn lexical nghiêm cho link tương đối, resolve/containment canonical cho link tuyệt đối lẫn tương đối, và mở file source an toàn. Owner từ chối link ngoài, dangling, không hỗ trợ, thoát cây theo path và chu trình khi resolve; directory link quay về ancestor cũng bị từ chối.
