# Changelog

All notable changes to docker-autoheal are documented in this file.
The sections should follow the order `Security`, `Added`, `Changed`, `Fixed`, and `Removed`.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

## 0.13.8

### Changed

- Simplify 'connection type' option check
- Introduce 'nightly' tag for regular updates to base OS and binary dependencies to ensure vulnerabilities are addressed in between releases
  - Reserve 'latest' tag for customary latest release

## 0.13.7

### Changed

- Bump bollard from 0.17.0 to 0.17.1 (which includes bumping various build dependencies)
- Remove static Cargo.lock to allow for more seamless automated updates (let Cargo.toml guide updates)

## 0.13.6

### Changed

- Bump bollard from 0.16.0 to 0.17.0 (which includes bumping various build dependencies)

## 0.13.5

### Security

- Bump openssl from 0.10.63 to 0.10.66
  - Fixed invariant violation in MemBio::get_buf with empty results

## 0.13.4

### Fixed

- Ensure log length assessment does not underflow and compensate for empty log
- Other code cleanup

### Security

- Bumps [rustls](https://github.com/rustls/rustls) from 0.22.2 to 0.22.4.
  - Infinite loop in rustls::conn::ConnectionCommon::complete_io() with proper client input

## 0.13.2

### Security

- Bump [h2](https://github.com/hyperium/h2) from 0.3.24 to 0.3.26
  - h2 servers vulnerable to degradation of service with CONTINUATION Flood

## 0.13.1

### Changed

- Streamline persistent log write which should resolve occasional conjoined entries

## 0.13.0

### Changed

- Breaking: Add state health exit code to persistent log
  - This will invalidate any existing log.json and cause an error; removal of existing log.json is requisite
- Breaking: Rename option and variable:
  - Binary option: --history replaced by -L, --log-persist
  - Environment variAble: AUTOHEAL_HISTORY replaced by AUTOHEAL_LOG_PERSIST
- Adjustment to log output formatting for consistency
- Additional error handling on read_record

## 0.12.0

### Added

- External persistent logging: Capture Date, container name, container id, last health message and last action to /opt/docker-autoheal/log.json
  - Requires the path to exist and that it is writable by the docker-autoheal user
  - Controlled via (default: disabled):
    - Binary option: -H, --history (switch)
    - Environment variable: AUTOHEAL_HISTORY (TRUE/FALSE)

### Changed

- Bump bollard to 0.16.0

## 0.11.2

### Security

- Bump mio from 0.8.10 to 0.8.11
  - Fix receiving IOCP events after deregistering a Windows named pipe (tokio-rs/mio#1760, backport pr: tokio-rs/mio#1761).

## 0.11.1

### Changed

- Minor change to default notification message

## 0.11.0

### Added

- Providing docker system hostname in webhook/apprise payload

### Changed

- Rework action and notify logic for efficiency
- Minor code cleanup

## 0.10.1

### Changed

- Binary options
  - -v, --version replaced by -V, --version to be in alignment with standard practices
  - Alphabetized help output for easier reading (lowercase then uppercase)

### Fixed

- Binary options
  - -a, --post-action replaced by -P, --post-action to resolve conflict with -a, --apprise-url

## 0.10.0

There are several breaking changes; please read carefully and refer to [README](https://github.com/tmknight/docker-autoheal/blob/main/README.md) for more details

### Added

- Binary options
  - -m, --monitor-all switch to control monitoring of all containers
- Environment variables
  - AUTOHEAL_MONITOR_ALL (TRUE/FALSE) to control monitoring of all containers
- Container labels
  - autoheal.monitor.enable (TRUE/FALSE) to control monitoring of individual containers
    - Overrides AUTOHEAL_MONITOR_ALL
- Returning last health 'ExitCode` and 'Output' to log (and webhook/apprise if configured) for unhealthy containers

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
  - -l, --container-label (see --monitor-all and autoheal.monitor.enable)
- Environment variables
  - AUTOHEAL_CONTAINER_LABEL (see --monitor-all and autoheal.monitor.enable)

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
