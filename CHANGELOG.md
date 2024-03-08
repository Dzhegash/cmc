# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased
### Added

### Changed

### Removed

## 0.4.3 - 2024-03-08
### Changed
- Visibility type `CmcResult`
- field type `price` for `cmc::api::cryptocurrency::quotes_latest_v2::Currency`
- fn `price_by_id()`
- fn `price_by_slug()`
- fn `price_by_symbol()`
- Unit tests

## 0.4.1 - 2024-01-18
### Added
- `base_url()`
- `metadata_map()`
- async `metadata_map()`
- Unit test

## 0.4.0 - 2023-09-01
### Added
- Async implementation (feature `async`)
- Minor fixes
- Documentation

## 0.3.0 - 2022-12-20
### Added
- Default features
- `fiat_id_map()`
- Documentation

### Changed
- Deprecated `id_map_fiat()`

## 0.2.4 - 2022-09-08
### Added
- `SortExchange`
- `ListingStatusExchange`
- `exchange_metadata()`
- `exchange_id_map()`
- `quotes_latest_by_id()`
- `quotes_latest_by_slug()`
- `quotes_latest_by_symbol()`
- `Display` implementation for `CmcExchangeIdMap`
- Unit tests
- Documentation

### Changed
- `api::cryptocurrency::metadata_v2::Metadata`

## 0.2.3 - 2022-08-20
### Added
- `global_metrics()`
- Unit tests
- Documentation

## 0.2.2 - 2022-08-10
### Added
- `metadata()`
- Unit tests

### Changed
- CmcErrors
- Pass
- Documentation

## 0.2.1 - 2022-07-31
### Added
- `categories()`
- `category()`
- Implementations of `Display` for `CmcCategories, Category`
- Unit tests

### Changed
- Documentation

## 0.2.0 - 2022-07-13
### Added
- Changelog
- `price_conversion()`
- `price_conversion_id()`
- Implement `Display` for `CmcIdMap` and `CmcIdMapFiat`
- Unit tests
- Documentation

### Changed
- Structures renamed
