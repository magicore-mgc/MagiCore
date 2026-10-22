# Open Regular File Tests

**Purpose:** Verify the shared opener's observable contract without invoking package managers or network services.

**Mục đích:** Xác minh hợp đồng quan sát được của helper mở file, không gọi package manager hay mạng.

The suite checks regular bytes, non-regular rejection, Unix final-symlink rejection, and an already-open handle after its pathname is replaced. Windows reparse handling remains assigned to the Windows CI lane; tests are authored but NOT_RUN.

Suite kiểm tra bytes file thường, từ chối file không thường, từ chối symlink cuối Unix và handle đã mở sau khi path bị thay. Nhánh reparse Windows thuộc lane CI Windows; test đã viết nhưng NOT_RUN.
