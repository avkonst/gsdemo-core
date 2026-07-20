#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

STORAGE_BUCKET="/tmp/gitscale-demo-bucket"
ARTEFACT_PATH="$STORAGE_BUCKET/github.com/avkonst/gsdemo-core/master.tar.gz"

# Build images
echo "==> Building core-server image..."
"$SCRIPT_DIR/build-core.sh"

echo "==> Building migrations image..."
"$SCRIPT_DIR/build-migrations.sh"

echo "==> All images built successfully."

# Compute tags (same logic as individual build scripts)
CORE_TAG=$(find "$SCRIPT_DIR/apps/core-server" -type f -print0 | sort -z | xargs -0 md5sum | md5sum | awk '{print $1}')
MIGRATIONS_TAG=$(find "$SCRIPT_DIR/apps/migrations" -type f -print0 | sort -z | xargs -0 md5sum | md5sum | awk '{print $1}')

# Generate docker-compose.yml with resolved versions
echo "==> Publishing artefact..."
TMPDIR=$(mktemp -d)
sed \
  -e "s/\${gsdemo-core-server-version}/$CORE_TAG/g" \
  -e "s/\${gsdemo-core-migrations-version}/$MIGRATIONS_TAG/g" \
  "$SCRIPT_DIR/docker-compose.yml" > "$TMPDIR/docker-compose.yml"

# Package and store
mkdir -p "$(dirname "$ARTEFACT_PATH")"
tar -czf "$ARTEFACT_PATH" -C "$TMPDIR" docker-compose.yml
rm -rf "$TMPDIR"

echo "==> Artefact published to $ARTEFACT_PATH"
