# Reachable closure

Walks dependency and peer references from the selected roots using owner/ecosystem-scoped indexes. Missing or ambiguous edges fail closed, then the reachable count is compared with all packages belonging to the active owner and selected ecosystems so orphan rows cannot enter a frozen install.

Duyệt dependency và peer từ các root đã chọn bằng index theo owner/ecosystem. Cạnh thiếu hoặc mơ hồ bị từ chối; sau đó so số package reachable với toàn bộ package của owner và ecosystem đang chọn để chặn row mồ côi lọt vào frozen install.
