#!/bin/sh
set -eu

# --- Réglages/Defaults ---
APP_DIR="${APP_DIR:-/app/rustfolio}"
MIGRATIONS="${MIGRATIONS:-$APP_DIR/migrations}"
DATABASE_URL="${DATABASE_URL:-sqlite://$APP_DIR/app.db}"

echo "APP_DIR=$APP_DIR"
echo "DATABASE_URL=$DATABASE_URL"
echo "MIGRATIONS=$MIGRATIONS"

# --- Résout le chemin physique de la DB si SQLite ---
resolve_sqlite_path() {
  dburl="$1"
  case "$dburl" in
    sqlite://*)
      raw="${dburl#sqlite://}"      # enlève sqlite://
      raw="${raw%%\?*}"             # enlève une éventuelle query (?mode=…)
      case "$raw" in
        ""|":memory:") echo "" ;;   # mémoire: pas de fichier
        /*)            echo "$raw" ;;
        *)             echo "$APP_DIR/$raw" ;;  # chemin relatif -> sous APP_DIR
      esac
      ;;
    *)
      echo ""                        # pas SQLite
      ;;
  esac
}

DB_PATH="$(resolve_sqlite_path "$DATABASE_URL")"

# --- Crée le dossier parent du fichier SQLite si applicable ---
if [ -n "$DB_PATH" ]; then
  mkdir -p "$(dirname "$DB_PATH")"
fi

# --- Création DB (ignore si existe déjà) ---
sqlx database create || true

# --- Migrations ---
set +e
out="$(sqlx migrate run --source "$MIGRATIONS" 2>&1)"
code=$?
set -e

if [ $code -ne 0 ]; then
  echo "$out"
  if printf "%s" "$out" | grep -q "previously applied but has been modified"; then
    if [ "${DEV_RESET_DB_ON_MIGRATION_MISMATCH:-0}" = "1" ]; then
      echo "⚠️  Migration modifiée détectée (DEV). Réinitialisation de la DB..."
      if [ -n "$DB_PATH" ]; then
        echo "Reset DEV: suppression DB -> $DB_PATH"
        rm -f "$DB_PATH" || true
      fi
      sqlx database create || true
      sqlx migrate run --source "$MIGRATIONS"
    else
      echo "❌ Migration modifiée. En prod, NE JAMAIS réécrire une migration : crée-en une nouvelle."
      exit 1
    fi
  else
    echo "❌ Échec des migrations (autre erreur)."
    exit 1
  fi
fi

exec "$@"
