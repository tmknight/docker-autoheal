# Changelog

All notable changes to docker-autoheal are documented in this file.
The sections should follow the order `Security`, `Added`, `Changed`, `Fixed`, and `Removed`.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

## 0.10.0

There are several breaking changes; please read carefully

### Added

- Environment variables
  - AUTOHEAL_MONITOR_ALL (TRUE/FALSE)
- Container labels
  - autoheal.monitor.enable (TRUE/FALSE) to control monitoring of individual containers
    - Overrides AUTOHEAL_MONITOR_ALL

### Changed

- Binary options
  - --log-exclude replaced by -l, --log-all
  - --post-action replaced by -a, --post-action
- Environment variables
  - AUTOHEAL_LOG_EXCLUDED replaced by boolean AUTOHEAL_LOG_ALL
- Container labels
  - autoheal.restart.exclude replaced by boolean autoheal.restart.enable

### Removed

- Binary options
  - -l, --container-label
- Environment variables
  - AUTOHEAL_CONTAINER_LABEL

## 0.9.0

### Added

- `post-action` to execute a task post-restart attempt
- `autoheal.restart.exclude` container label as override when `AUTOHEAL_CONTAINER_LABEL` set to `all`
- `log-excluded` as a switch to allow logging of containers excluded from restart

## 0.8.3

### Changed

- Small change to `log_message` to discard Err and always return Ok
- Slight change to how startup delay is implemented and reported
- Minor code cleanup

### Removed

- h2 build dependency statement no longer required as now addressed upstream

## 0.8.2

### Changed

- Small change to how connection type assessed and reported for the sake of efficiency
- Minor code cleanup

## 0.8.1

### Changed

- Small change to how webhook and apprise are called for the sake of efficiency
- Updated license to GPL-3.0

## 0.8.0

### Added

- Binary option for `tcp-timeout`, now in alignment with environment `AUTOHEAL_TCP_TIMEOUT`
  - Breaking changes
    - `stop-timeout` is now `s`
    - `tcp-timeout` is now `t`
- Additional checks, balances & error handling

### Changed

- Refactored binary options into separate function for more efficient parsing
- Refactored environment variables into separate function for better organization

## 0.7.0

### Added

- Per container override (in seconds) of `AUTOHEAL_STOP_TIMEOUT` during restart via `autoheal.stop.timeout` label

## 0.6.1

### Fixed

- Binary options descriptions and hints for webhook entries

## 0.6.0

### Added

- Support for webhook and/or apprise

## 0.5.4

### Added

- Support for connecting to Docker hosts over TLS

## 0.5.3

### Removed

- SSL support is not ready; removed referrences to SSL

## 0.5.2

### Security

- Bump build-dependency [h2](https://github.com/hyperium/h2/releases/tag/v0.3.24) from 0.3.22 to 0.3.24 which addresses:
  - Limit error resets for misbehaving connections

## 0.5.1

### Changed

- Logging updated to better standardize on error level and improve output formatting
