# Changlog

## [2.0.0] - 2026-03-22
### Added
- Custom writer support via `set_debug_writer`
- `auto` mode for `debug!` and `get_dbginfo!`

### Changed
- `get_dbginfo!` on `auto` and boolean literals now uses `$crate::has_debug_flag` instead of just `has_debug_flag`
- `has_debug_flag` now takes a `should_check_build` argument

## [1.0.1] - 2026-03-22
### Added
- A README file
- A LICENSE file

## [1.0.0] - 2026-03-22
Initial Release
