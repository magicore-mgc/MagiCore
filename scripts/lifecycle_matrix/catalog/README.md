# Lane catalog / Danh mục lane

Each core file owns only its literal lifecycle lane profiles. `catalog.py` concatenates all nine core lists in the captured order into the single mutable `LANES` list. No profile is inferred from an adapter or copied from another core.

Mỗi file core chỉ giữ lane literal của core đó. `catalog.py` ghép đủ chín core theo đúng thứ tự baseline vào một list `LANES` dùng chung. Không suy luận profile từ adapter hoặc copy lane của core khác.
