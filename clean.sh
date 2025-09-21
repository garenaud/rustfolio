#!/usr/bin/env bash
set -euo pipefail

# ── OPTIONS ────────────────────────────────────────────────────────────────────
# --deep : supprime aussi les images construites par ce compose
DEEP=false
while [[ "${1:-}" =~ ^- ]]; do
  case "$1" in
    --deep) DEEP=true ;;
    -h|--help)
      echo "Usage: $0 [--deep]"
      echo "  --deep  Supprime aussi les images du projet"
      exit 0;;
    *) echo "Option inconnue: $1" ; exit 1;;
  esac
  shift || true
done

# ── SÉCURITÉ : vérifier qu’on est bien dans le bon projet ─────────────────────
if [[ ! -f "docker-compose.yml" && ! -f "compose.yml" && ! -f "docker-compose.yaml" && ! -f "compose.yaml" ]]; then
  echo "❌ Aucun fichier docker compose trouvé dans ce dossier."
  echo "   Exécute ce script depuis la racine de ton projet."
  exit 1
fi

# Nom de projet que Docker Compose utilise (par défaut: nom du dossier)
PROJECT_NAME="${COMPOSE_PROJECT_NAME:-$(basename "$PWD")}"
echo "🧹 Nettoyage du projet: ${PROJECT_NAME}"

# ── 1) Arrêt & suppression via compose (conteneurs + réseaux + volumes orphelins)
echo "⛔ docker compose down -v --remove-orphans"
docker compose down -v --remove-orphans || true

# ── 2) Tout retirer par label (au cas où il reste des trucs)
echo "🗑️  Suppression ciblée des ressources labellisées par compose"
CID=$(docker ps -aq --filter "label=com.docker.compose.project=${PROJECT_NAME}") || true
if [[ -n "${CID}" ]]; then
  docker rm -f ${CID} || true
fi

NID=$(docker network ls -q --filter "label=com.docker.compose.project=${PROJECT_NAME}") || true
if [[ -n "${NID}" ]]; then
  docker network rm ${NID} || true
fi

VID=$(docker volume ls -q --filter "label=com.docker.compose.project=${PROJECT_NAME}") || true
if [[ -n "${VID}" ]]; then
  docker volume rm ${VID} || true
fi

if $DEEP; then
  echo "🧯 Suppression des images du projet (--deep)"
  IID=$(docker images -q --filter "label=com.docker.compose.project=${PROJECT_NAME}") || true
  if [[ -n "${IID}" ]]; then
    docker rmi -f ${IID} || true
  fi
fi

# ── 3) Nettoyage des artefacts locaux du repo ─────────────────────────────────
echo "🧽 Nettoyage des artefacts locaux"

# dossiers courants (adapte la liste à ton arborescence)
paths=(
  "target"                         # workspace target
  "rustfolio/target"
  "rustfolio/dashboard-spa/target"
  "rustfolio/dashboard-spa/dist"   # trunk/yew build
  "rustfolio/assets/.trunk"        # si jamais tu as ce cache
)

# fichiers lock éventuels (si tu veux repartir clean)
files=(
  "Cargo.lock"
  "rustfolio/Cargo.lock"
  "rustfolio/dashboard-spa/Cargo.lock"
)

for p in "${paths[@]}"; do
  [[ -e "$p" ]] && { echo "  rm -rf $p"; rm -rf "$p"; }
done

for f in "${files[@]}"; do
  [[ -f "$f" ]] && { echo "  rm -f $f"; rm -f "$f"; }
done

# ── 4) Bonus : supprimer les conteneurs/images “dangling” DU PROJET seulement ─
# (Par sécurité on ne touche pas au global.)
# Rien ici : déjà géré par labels + compose.

echo "✅ Clean terminé pour '${PROJECT_NAME}'."
echo "   Astuce: relance un build propre avec:  docker compose build --no-cache && docker compose up -d"
