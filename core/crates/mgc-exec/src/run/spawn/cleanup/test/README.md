# Spawn cleanup tests

The owner-local tests cover clean success, execution-only failure, cleanup-only failure, and simultaneous execution plus cleanup failure for both Windows handles and the guarded environment. Combined failures must keep execution as the primary error source; cleanup-only environment errors keep their typed variant and I/O source. Tests have not been run.

Test gần owner kiểm tra success sạch, execution-only failure, cleanup-only failure và hai lỗi cùng lúc cho Windows handle/guarded environment. Lỗi ghép giữ execution làm source chính; lỗi environment đơn lẻ giữ đúng variant và I/O source. Chưa chạy.
