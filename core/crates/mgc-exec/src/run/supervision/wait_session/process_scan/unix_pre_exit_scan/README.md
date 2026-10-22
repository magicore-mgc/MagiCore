# Unix pre-exit scan

Runs the existing tracked descendant scan before `try_wait`, so a forbidden child is terminated even when the root command is about to exit.

# Quét Unix trước khi root thoát

> Chạy descendant scan có tracking trước `try_wait`; child bị cấm vẫn bị dừng kể cả khi root command sắp thoát.
