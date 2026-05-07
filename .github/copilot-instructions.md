# JSY-MK-194-rs Copilot Instructions

## Project Overview

A Rust driver for the JSY MK-194 power monitor IC, supporting synchronous, asynchronous, and std-based operation modes via feature flags. The driver should work across multiple platforms, including embedded no_std environments and desktop applications. They will use the appropriate HAL traits for serial communication and timing based on the selected mode.

## Feature Flags

Three mutually exclusive modes:

| Feature    | Description                                                                                           |
|------------|-------------------------------------------------------------------------------------------------------|
| `std-sync` | Desktop/PC usage, wraps `std::io` and requires `embedded-hal` for timing                              |
| `sync`     | Embedded no_std sync via `embedded-io` + `embedded-hal`                                               |
| `async`    | Embedded no_std async via `embedded-io-async` + `embedded-hal-async`                                  |
| `advanced` | Optional features for advanced users (e.g., allow reading registers directly, exposing internal APIS) |

- **Never combine** `std-sync` with `sync` or `async`.
- **Never combine** `sync` with `async`.
- Use `#[cfg(feature = "...")]` for conditional compilation. Use `#[maybe_async::maybe_async]` on functions that must work in both sync and async modes.

## Architecture

- `src/lib.rs` — Feature gate validation and HAL re-exports.
- `src/jsy_mk_194g.rs` — Main driver struct `JsyMk194g<Serial, D>`.
- `src/getters.rs` / `src/setters.rs` — Public API impl blocks.
- `src/registers/` — Register definitions and traits.
- `src/modbus/` — Low-level Modbus RTU protocol (requests, responses, CRC, buffer comms).
- `src/error.rs` — Error types.
- `src/types.rs` / `src/units.rs` — Shared types and `uom`-based physical units.

## Code Conventions

- No unsafe code (`#![deny(unsafe_code)]`).
- Use `uom` for physical quantities (e.g., `Energy`, `Frequency`, `ElectricPotential`). Always use typed units, never raw floats, in the public API.
- Register structs are defined in `src/registers/` with traits from `src/registers/traits.rs`.
- Modbus requests/responses are built via `ReadRequest`/`WriteRequest` in `src/modbus/requests.rs`.
- Error messages in `JSYMk194Error` should be clear and actionable.
- Clone is used on `Id` and `RegisterAddress`; prefer moving when possible.

## Build and Test

```bash
# Build with std (default)
cargo build

# Build for no_std async (embedded)
cargo build --no-default-features --features async

# Run all tests (hardware must be connected; single-threaded required)
cargo test --no-fail-fast -- --test-threads=1

# Run with std feature
cargo test --features std -- --test-threads=1

# Run a specific test
cargo test --test <test_file> -- <test_name> --exact --nocapture
```

Some tests require physical hardware (JSY-MK-194 device connected via serial).

## Registers

- Each register maps to a Modbus register address.
- Registers implement `from_scaled_value` / `to_scaled_value` for unit conversion and `try_from_bytes` / `to_bytes` for Modbus communication.
- Scalars are defined in `src/registers/scalars.rs`.
- Two channels (Channel::One, Channel::Two) are supported.

## Examples

Examples live in `examples/` and require the `std` feature:

```bash
cargo run --example getters --features std
```

## Utils

A `utils` module exists for helper functions and implemention of a std delay provider for the `std` feature. It is not intended for public use and should not be exposed in the public API.
