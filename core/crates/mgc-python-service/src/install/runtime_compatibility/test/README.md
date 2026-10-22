# Runtime-compatibility tests

These tests cover unsafe site path inputs, an incompatible compiled-wheel ABI, a source distribution, and a universal pure-Python wheel. They verify the public error category for incompatibility, but OS-specific ABI behavior still requires runtime CI. Authored tests are currently `NOT_RUN`.

## Test tương thích runtime

Test bao phủ input path site không an toàn, ABI compiled wheel không tương thích, source distribution và universal pure-Python wheel. Chúng kiểm tra error category public; hành vi ABI theo OS vẫn cần CI runtime. Test đã viết, hiện `NOT_RUN`.
