# Changelog

## 2026-03-30 — Fork migration

Forked from [sw-vibe-coding/web-tc24r](https://github.com/sw-vibe-coding/web-tc24r) to [sw-embed/web-sw-cor24-tinyc](https://github.com/sw-embed/web-sw-cor24-tinyc) as part of the COR24 ecosystem consolidation.

### Changes

- Renamed package from `web-tc24r` to `web-sw-cor24-tinyc`
- Updated tc24r compiler dependency paths from `../tc24r/` to `../sw-cor24-tinyc/`
- Updated emulator dependency path from `../../sw-embed/cor24-rs` to `../sw-cor24-emulator`
- Updated bundled header `include_str!` paths to reference `sw-cor24-tinyc`
- Updated GitHub URLs and demo links to sw-embed organization
- Added `scripts/build-pages.sh` for GitHub Pages deployment
- Removed legacy `.agentrail/` metadata and pre-built `pages/` artifacts
- Updated README, CLAUDE.md with new repository references
