# Changelog

All notable changes to docker-autoheal are documented in this file.
The sections should follow the order `Packaging`, `Added`, `Changed`, `Fixed` and `Removed`.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

- Support for connecting to Docker hosts over TLS
- Support for webhook and/or apprise

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
