# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.1](https://github.com/ScottGibb/JSY-MK-194-rs/compare/v0.0.0...v0.0.1) - 2026-05-07

### Added

- add simple power loop example
- add feature gating for advanced mode
- add setter example
- remove hard dep
- add register size check to response
- add check to new_default
- *(ai)* add copilot instructions
- better gating around std and sync
- *(ci)* add std build
- add gettters example
- add get all channels
- *(tests)* add missing config attribute
- *(tests)* add generic test to find and set id baudrate
- remove magic numbers
- add working Channel Read function
- *(tests)* passing energuy register tests!
- swap to using requests and response types
- *(errors)* add modbus error handling to the responses
- add requests structure for WriteRequest
- add extra structures for the modbus protocol
- add try_from for the register enum
- add size check to macro for register
- *(tests)* add more test output
- add message check
- make function code public
- add non exhaustive to error
- add device response error
- add exception response check
- *(tests)* swap tp kilowatt_hour
- *(tests)* add remaining setter tests
- add single channel read
- add initial version for full read
- add extra error type
- *(tests)* add more getter tests
- add defauly for SystemParamaters
- *(tests)* add id and baudrate
- add get_id function
- add tests
- add register tests
- add watt hour to units
- expanded error enum to contain more data
- expose response delay
- add Read and Write to Energy Registers
- add Passing Tests
- add Function codes for errors
- *(ci)* add more tests regarding the misc
- Add missing register implementation for PowerDirectionRegister
- add try_from for configuration types
- add default values to system configuration
- add delay struct to driver
- add register tests
- add baudrate conversion
- add display and debug macros to types
- add msrv
- add std support
- add remaining setters
- add first setter method
- add ID Type
- add some getters
- add Modbus Error Message
- update deps to use minor versions rather than patches
- add getter templates
- adding initial implementation of registers
- add generic read and write for all registers
- add initial register definitions

### Fixed

- power register le vs be from bytes
- copilot review warnings
- changed default feature
- async build issues
- getter example compile warning
- increase response delays
- change set_baudrate to consume the driver
- clippy issues
- clippy warnings
- naming of parameter
- channel extraction
- feature gates
- add feature gates for std sync
- remove illegal comment
- dead imports
- wrong register used for the power registers
- async mode and converion error type
- incorrect scalar for power
- remove print statements from codebase
- write response error
- remove dead offsets
- better error message
- make it compile, will be removed in future
- modbus error code
- formatting
- error message
- error code position
- add todo
- clippy fixes and formatting
- fix id setter
- *(tests)* add a bit more timeout
- remove redudant tests
- add missing buff elements
- remove default implementation from macro
- clippy warnings
- scalars
- better error handling
- missing scalar
- clippy errors
- remove old example
- clippy issues

### Other

- remove build docs
- :art: Applied MegaLinter Changes
- :art: Applied MegaLinter Changes
- :art: Applied MegaLinter Changes
- const names
- add setter comment
- add better formatting
- add cargo.toml example
- add datasheet
- add badges
- clippy fixes
- fix typos
- better error handling
- improve error type for unexpected length
- improve error handling
- remove panics and bubble errors
- bytes and offsets for protocol
- :art: Applied MegaLinter Changes
- add todos
- Read Tests now work ish
- noting failing test
- communications
- wip
- *(tests)* increase timings and standardise
- *(tests)* naming and doc string
- add address offsets
- getters layout
- *(tests)* Introduce common package
- adding more type safety to communications
- add derive instead of impl block
- add enum comments
- add test documentation
- type restrictions
- add more test cases for the registers
- id typing enforcement
- rename of tests
- add missing doc strings for the error type
- remove unecessary comment
- handle read and write errors
- protocol and read write methods
- might abandon this way of doing the reading and writing, it doesnt make sense considering everything is either 16 bit or 32 bit values were usiing bitbybit but like only two registers are like that
- *(deps)* Bump marocchino/sticky-pull-request-comment from 2 to 3
