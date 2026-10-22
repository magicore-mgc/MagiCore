# Web build workspace

This owner finds the project root and creates the filtered mirror used when MagiCore-managed node_modules needs a resolvable workspace view.

Owner này tìm root dự án và tạo mirror đã lọc khi node_modules do MagiCore quản lý cần workspace view có thể resolve.

The mirror is Web-specific: it skips generated output, MagiCore state and package installations, links the original node_modules tree, and removes only its private temporary directory when its guard drops.

Mirror thuộc riêng Web: bỏ output sinh ra, state MagiCore và package install; link cây node_modules gốc; guard chỉ xóa thư mục temporary riêng khi bị drop.
