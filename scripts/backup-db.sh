#!/bin/bash
# Backup database before deploy
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="backup_${TIMESTAMP}.sql"

echo "📦 Backing up database to ${BACKUP_FILE}..."
docker exec sic-mundus-db pg_dump -U postgres sic_mundus > "${BACKUP_FILE}"

if [ $? -eq 0 ]; then
  echo "✅ Backup saved: ${BACKUP_FILE}"
else
  echo "❌ Backup failed!"
  exit 1
fi
