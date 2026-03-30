#!/usr/bin/env bash
set -euo pipefail

# Build for GitHub Pages deployment
trunk build --release --public-url /web-sw-cor24-tinyc/ -d pages
