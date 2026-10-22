# Hardware scaffold owners

## Responsibility
This folder owns the built-in Hardware fallback. The stable HardwareProcessor API selects optimizer or bench marker output; command validation and later runtime operations belong elsewhere.

## Phạm vi
Folder này quản lý fallback Hardware tích hợp. API HardwareProcessor ổn định chọn output marker optimizer hoặc bench; validate command và thao tác runtime sau đó thuộc nơi khác.

## Preserved behavior
Every key other than optimizer still selects the bench writer when called directly. The normal create-hardware command validates optimizer/bench first; this structural split leaves the lower-level wildcard unchanged.

## Hành vi được giữ
Mọi key ngoài optimizer vẫn chọn writer bench khi gọi trực tiếp. Lệnh create-hardware thông thường validate optimizer/bench trước; refactor cấu trúc giữ nguyên wildcard ở tầng writer.
