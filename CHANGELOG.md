# Changelog

## [v0.8.0](https://github.com/hoodie/dvb-rs/compare/v0.7.4...v0.8.0) (2026-02-20)

### Features

* add route changes endpoint
([3237b68](https://github.com/hoodie/dvb-rs/commit/3237b6822c793472e2666382b1e2added4d9e51a))

### Fixes

* add missing request params for point
([6ef17d2](https://github.com/hoodie/dvb-rs/commit/6ef17d2fbbd4863ec43327a4aae25d036a511305))
* add missing request params for route
([698c69f](https://github.com/hoodie/dvb-rs/commit/698c69f019e372f8da11ec86a23201fd2d2348d9))
* add/update missing route fields
([8061e91](https://github.com/hoodie/dvb-rs/commit/8061e91decd3a5ee725eff396e73becdb29e9409))
* add missing trip fields
([5291ec9](https://github.com/hoodie/dvb-rs/commit/5291ec90fcbb447434b21deb44bb9a42458963d4))
* add missing enum variants
([a93b4cb](https://github.com/hoodie/dvb-rs/commit/a93b4cb23cf64f86d7ef7f278394b4d56153318c))
* use POST requests where necessary
([fe312df](https://github.com/hoodie/dvb-rs/commit/fe312df5fe8fc4e326c9bd8d733361cbb9c3c616))
* add missing departure fields
([6678be5](https://github.com/hoodie/dvb-rs/commit/6678be5067dbd0c1415a36b82e2d15e1d2b9d744))

### [v0.7.4](https://github.com/hoodie/dvb-rs/compare/v0.7.3...v0.7.4) (2026-01-11)

#### Fixes

* add BusOnRequest to Mode of Transport enum
([793f8cb](https://github.com/hoodie/dvb-rs/commit/793f8cb7d8cf3bb45c7035f1385b711a0ba05382))

### [v0.7.3](https://github.com/hoodie/dvb-rs/compare/v0.7.2...v0.7.3) (2025-12-25)

#### Fixes

* add missing Clone and Deserialize implementations
([4d42b63](https://github.com/hoodie/dvb-rs/commit/4d42b63c9afc13b74b914bf180545dc02e90aeb8))

### [v0.7.2](https://github.com/hoodie/dvb-rs/compare/v0.7.1...v0.7.2) (2025-12-19)

#### Features

* add feature to serialize dvb-time stamps to iso
([ce6899c](https://github.com/hoodie/dvb-rs/commit/ce6899ce807332cc5d5f4ff4e7fec458fa34701a))

### [v0.7.1](https://github.com/hoodie/dvb-rs/compare/v0.7.0...v0.7.1) (2025-10-29)

#### Fixes

* add module-level doc comments to all source files
([bf882ba](https://github.com/hoodie/dvb-rs/commit/bf882ba8bc7d5573bb3a366fd9a1aad83adcb920))

## [v0.7.0](https://github.com/hoodie/dvb-rs/compare/v0.6.0...v0.7.0) (2025-10-29)

## [v0.6.0](https://github.com/hoodie/dvb-rs/compare/v0.5.0...v0.6.0) (2025-08-26)

### ⚠ BREAKING CHANGE

* extend Mot and DvbTime types


### Features

* add schemars JsonSchema derives to API structs
([dfec501](https://github.com/hoodie/dvb-rs/commit/dfec5010c09594ac5188919543caf336964b85a6))
* support routes api
([67575ad](https://github.com/hoodie/dvb-rs/commit/67575add4564020b9cacd24093e309da8ad92d07))

## [v0.5.0](https://github.com/hoodie/dvb-rs/compare/v0.4.1...v0.5.0) (2025-08-04)

### Features

* async
([62c2b7e](https://github.com/hoodie/dvb-rs/commit/62c2b7e157d921f584f126855dd9a9fc616857f5))
* add lines api
([cec405d](https://github.com/hoodie/dvb-rs/commit/cec405dd395b3a71abb76ebbeaef957b2e30f482))

### Fixes

* DepartureMonitor may have no Place or Name in case of an error
([d084551](https://github.com/hoodie/dvb-rs/commit/d08455102c5d3310ba31ae72a33cec989bc9eede))

### [v0.4.1](https://github.com/hoodie/dvb-rs/compare/v0.4.0...v0.4.1) (2022-12-22)

#### Fixes

* handle compile errors and warnings
([58b4eee](https://github.com/hoodie/dvb-rs/commit/58b4eee4365a450dc8adb7b34533dee452dd075a))

## v0.4.0 (2019-04-22)
