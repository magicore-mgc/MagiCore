# Link worker pool

Owns the environment override, bounded default thread count and lazily initialized Rayon pool used by callers outside an existing Rayon worker.

Sở hữu override môi trường, số worker mặc định có giới hạn và Rayon pool khởi tạo lười cho caller chưa ở worker Rayon.
