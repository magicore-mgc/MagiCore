# CI generation / Sinh cấu hình CI

## Responsibility / Trách nhiệm

Selects the detected provider and writes the matching workflow to its existing destination.

## Tests / Kiểm thử

Provider destination/template selection is covered by the local pure test; filesystem writes remain exercised only by the later CLI integration lane. No provider executable is started.

## Tiếng Việt

Owner này chọn provider được nhận diện và ghi workflow tương ứng vào đúng đường dẫn hiện có. Test thuần kiểm tra kế hoạch đường dẫn/template; chỉ integration CLI mới kiểm tra thao tác ghi file. Không chạy executable của provider.
