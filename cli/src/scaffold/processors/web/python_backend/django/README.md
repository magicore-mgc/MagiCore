# Django starter

## Purpose

Owns the native Django backend starter: PEP 621 `pyproject.toml` (django>=5,<6), `manage.py`, a settings module with `DJANGO_SECRET_KEY` from the environment (dev fallback is marked insecure in-comment), urls with `/api/health`, plus WSGI and ASGI entry points. Migrations are intentionally not generated.

## Vietnamese

Sở hữu starter backend Django native: `pyproject.toml` PEP 621 (django>=5,<6), `manage.py`, module settings lấy `DJANGO_SECRET_KEY` từ môi trường (giá trị dev có comment cảnh báo không an toàn), urls có `/api/health`, cùng entry WSGI và ASGI. Cố ý không sinh migrations.
