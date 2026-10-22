# Release artifact smoke scripts / Script smoke artifact phát hành

This folder owns the Web install smoke step for the unpacked release binary on Bash and PowerShell runners. It creates the same minimal project, invokes the distributed binary with core web, and verifies express materialization. The workflow owns artifact download, extraction, OS matrix, and version smoke.

Folder này sở hữu bước smoke Web install cho binary release đã giải nén trên Bash và PowerShell. Script tạo project tối thiểu, gọi binary phân phối với core web và xác minh express được materialize. Workflow sở hữu download/extract artifact, ma trận OS và kiểm tra phiên bản.

The executable path is passed through an environment variable; scripts do not resolve another package manager. Add focused contract tests later; this source-first batch does not create or run them.
