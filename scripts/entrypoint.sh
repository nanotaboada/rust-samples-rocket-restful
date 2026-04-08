#!/bin/sh
set -e

# Helper function for formatted logging
log() {
    echo "[ENTRYPOINT] $(date '+%Y/%m/%d - %H:%M:%S') | $1"
    return 0
}

VOLUME_STORAGE_PATH="${STORAGE_PATH:-/storage/players-sqlite3.db}"

log "✔ Starting container..."

mkdir -p "$(dirname "$VOLUME_STORAGE_PATH")"

if [ ! -f "$VOLUME_STORAGE_PATH" ]; then
    log "⚠️ No existing database file found in volume."
    log "🗄️ Diesel migrations will initialize the database on first start."
else
    log "✔ Existing database file found at $VOLUME_STORAGE_PATH."
fi

log "✔ Ready!"
log "🚀 Launching app..."
log "🔌 API endpoints | http://localhost:9000"
log "📚 Swagger UI    | http://localhost:9000/swagger-ui/"
exec "$@"
