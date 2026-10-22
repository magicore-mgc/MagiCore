# Mutation journal / Journal mutation

This owner defines the closed mutation operation, snapshot and versioned journal document, then wires focused owners for safe filesystem access, project identity, canonical digest, parsing, staging, completion, recovery, restoration and rollback. The single mutation gateway remains in the shared CLI service and keeps its lock → legacy adoption → recovery → lock validation order.

Owner này định nghĩa operation mutation dạng enum đóng, snapshot và document journal có version, rồi nối các owner chuyên trách filesystem an toàn, identity project, digest canonical, parser, staging, completion, recovery, restore và rollback. Mutation gateway duy nhất vẫn nằm ở shared CLI service và giữ thứ tự lock → chuyển journal cũ → recovery → kiểm tra lock.
