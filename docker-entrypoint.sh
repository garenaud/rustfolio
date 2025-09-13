#!/bin/sh
set -eu

# --- Réglages/Defaults ---
# Dossier de l'app (peut être override via APP_DIR dans docker-compose)
APP_DIR="${APP_DIR:-/app/rustfolio}"
MIGRATIONS="${MIGRATIONS:-$APP_DIR/migrations}"
DATABASE_URL="${DATABASE_URL:-sqlite://$APP_DIR/app.db}"

echo "APP_DIR=$APP_DIR"
echo "DATABASE_URL=$DATABASE_URL"
echo "MIGRATIONS=$MIGRATIONS"

mkdir -p /app/data || true

# --- Création DB (ignore si existe déjà) ---
# Certains sqlx-cli n'ont pas --skip-if-exists, on ignore l'erreur proprement.
sqlx database create || true

# --- Fonction utilitaire: résout le chemin physique de la DB si SQLite ---
resolve_sqlite_path() {
  dburl="$1"
  case "$dburl" in
    sqlite://*)
      raw="${dburl#sqlite://}"       
      raw="${raw%%\?*}"              
      case "$raw" in
        ""|":memory:") echo "" ;;    
        /*)            echo "$raw" ;;
        *)             echo "$APP_DIR/$raw" ;;
      esac
      ;;
    *)
      echo ""
      ;;
  esac
}

DB_PATH="$(resolve_sqlite_path "$DATABASE_URL")"

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
