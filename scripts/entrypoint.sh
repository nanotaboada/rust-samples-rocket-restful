#!/bin/sh
set -e

# Helper function for formatted logging
log() {
    echo "[ENTRYPOINT] $(date '+%Y/%m/%d - %H:%M:%S') | $1"
    return 0
}

VOLUME_STORAGE_PATH="${STORAGE_PATH:-/storage/players-sqlite3.db}"

log "Starting container..."

mkdir -p "$(dirname "$VOLUME_STORAGE_PATH")"

log "Ready!"
log "Launching app..."
log "API endpoints | http://localhost:9000"
exec "$@"
