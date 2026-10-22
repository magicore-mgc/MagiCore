# Import artifact reader / Reader artifact import

Owns safe optional reads for the prior `mgc.lock` and signature snapshots. Missing files remain `None`; all other lockfile-reader failures retain their path and source.

Sở hữu việc đọc an toàn snapshot `mgc.lock` và signature trước đó. File không có vẫn là `None`; lỗi reader khác giữ path và source.
