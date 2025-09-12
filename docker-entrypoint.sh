#!/usr/bin/env sh
set -euxo pipefail

: "${DATABASE_URL:=sqlite:///app/data/app.db}"
export DATABASE_URL

mkdir -p /app/data

sqlx database create || true
sqlx migrate run --source /app/web-hello/migrations

exec "$@"
