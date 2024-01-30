# Changelog

All notable changes to docker-autoheal are documented in this file.
The sections should follow the order `Security`, `Added`, `Changed`, `Fixed`, and `Removed`.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

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
