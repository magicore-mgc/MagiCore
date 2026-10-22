# Mutation policy / Policy mutation

This owner rejects unsafe legacy writes when signature evidence exists, when the target is not a regular file, or when the current document is v4. It also protects the legacy signing entry point from flattening a v4 document.

Owner này chặn ghi legacy không an toàn khi có bằng chứng chữ ký, target không phải file thường hoặc document hiện tại là v4. Nó cũng ngăn luồng ký legacy làm phẳng document v4.
