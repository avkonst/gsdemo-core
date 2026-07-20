#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

TAG=$(find "$SCRIPT_DIR/apps/core-server" -type f -print0 | sort -z | xargs -0 md5sum | md5sum | awk '{print $1}')

docker build \
  -f "$SCRIPT_DIR/Dockerfile.core-server" \
  -t "gsdemo-core-server:$TAG" \
  "$SCRIPT_DIR"

echo "Tagged: gsdemo-core-server:$TAG"
