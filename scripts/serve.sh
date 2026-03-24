#!/usr/bin/env bash
set -euo pipefail

PORT=9101

exec trunk serve --port "$PORT" "$@"
