#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

TAG=$(find "$SCRIPT_DIR/apps/migrations" -type f -print0 | sort -z | xargs -0 md5sum | md5sum | awk '{print $1}')

docker build \
  -f "$SCRIPT_DIR/Dockerfile.migrations" \
  -t "gsdemo-core-migrations:$TAG" \
  "$SCRIPT_DIR"

echo "Tagged: gsdemo-core-migrations:$TAG"
