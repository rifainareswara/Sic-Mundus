#!/bin/bash
# Safe deploy script — backup first, then rebuild without touching volumes
set -e

echo "🔒 Step 1: Backup database..."
bash "$(dirname "$0")/backup-db.sh"

echo "🚀 Step 2: Rebuilding containers (data preserved)..."
docker compose up -d --build backend frontend

echo "✅ Deploy complete! Data is safe."
echo ""
echo "⚠️  NEVER run: docker compose down -v  (this deletes all data!)"
echo "    Safe stop:  docker compose down"
