# JSY-MK-194 Rust Driver

[![Mega Linter](https://github.com/ScottGibb/jsy-mk-194-rs/actions/workflows/mega-linter.yaml/badge.svg)](https://github.com/ScottGibb/jsy-mk-194-rs/actions/workflows/mega-linter.yaml)
[![Continuous Build](https://github.com/ScottGibb/jsy-mk-194-rs/actions/workflows/continuous-build.yaml/badge.svg)](https://github.com/ScottGibb/jsy-mk-194-rs/actions/workflows/continuous-build.yaml)
[![Dependabot](https://github.com/ScottGibb/jsy-mk-194-rs/actions/workflows/dependabot.yaml/badge.svg)](https://github.com/ScottGibb/jsy-mk-194-rs/actions/workflows/dependabot.yaml)
[![Release](https://github.com/ScottGibb/jsy-mk-194-rs/actions/workflows/release-plz.yaml/badge.svg)](https://github.com/ScottGibb/jsy-mk-194-rs/actions/workflows/release-plz.yaml)
[![Crates.io Version](https://img.shields.io/crates/v/jsy-mk-194-rs?color=green)](https://crates.io/crates/jsy-mk-194-rs)
[![Crates.io Downloads](https://img.shields.io/crates/dv/jsy-mk-194-rs)](https://crates.io/crates/jsy-mk-194-rs)

Rust driver for the JSY MK-194 power monitor IC, designed for both desktop and embedded environments. Using `embedded-io` and `embedded-io-async` traits, allowing for true cross platform enablement.

## Add to Cargo.toml

Choose one runtime mode feature to work with in your application.

`sync` mode example:

```toml
[dependencies]
jsy-mk-194-rs = { version = "x.x", default-features = false, features = ["sync"] }
```

`std-sync` (default) mode example:

```toml
[dependencies]
jsy-mk-194-rs = { version = "x.x", features = ["std-sync"] }
```

`async` mode example:

```toml
[dependencies]
jsy-mk-194-rs = { version = "x.x", default-features = false, features = ["async"] }
```

## Why this crate

- Multi-platform support via feature flags.
- `sync` and `async` operation modes.
- Strongly typed measurements using uom (units of measurement), so APIs return quantities like voltage, current, frequency, and energy as typed values instead of raw floats.

## Feature flags

Only one runtime mode should be enabled at a time:

- `std-sync` (default): desktop/std usage.
- `sync`: no_std synchronous embedded usage.
- `async`: no_std asynchronous embedded usage.
- `advanced`: exposes low-level register modules and direct register read/write access.

## Run examples

The repository currently includes examples in the [examples](./examples/) directory.

You can run them like so:

```bash
cargo run --example getters --features std-sync
```

## Run tests

Some integration tests require real JSY MK-194 hardware connected over serial. Tests should be run single-threaded to avoid hardware access conflicts.

Run the full test suite with the `std-sync` flag, this is the easiest way and doesnt require any extra hardware other than a serial converter:

Run tests in std-sync mode explicitly:

```bash
cargo test --features std-sync -- --test-threads=1
```

## Useful Links

- [JSY-MK-194 Module](https://www.aliexpress.com/item/1005007369940517.html#nav-review)
- [USB to TTL Converter](https://www.amazon.co.uk/DSD-TECH-SH-U09C2-Debugging-Programming/dp/B07TXVRQ7V/ref=sxin_16_sbv_search_btf?aref=Vv9YmCPRzk&content-id=amzn1.sym.0cf85714-a5b0-4d96-9050-d0dce9037023%3Aamzn1.sym.0cf85714-a5b0-4d96-9050-d0dce9037023&cv_ct_cx=usb%2Bttl&keywords=usb%2Bttl&pd_rd_i=B07TXVRQ7V&pd_rd_r=04212b29-7e5e-4633-bacc-94676cbfd216&pd_rd_w=6nYOC&pd_rd_wg=K7M1W&pf_rd_p=0cf85714-a5b0-4d96-9050-d0dce9037023&pf_rd_r=961QBR06A76W3GQFKM4H&qid=1778159565&sbo=RZvfv%2F%2FHxDF%2BO5021pAnSA%3D%3D&sr=1-1-9131241a-a358-4619-a7b8-0f5a65d91d81&th=1)
- [JSY-MK-194 Datasheet](./docs/JSY-MK-194G-User-Manual.pdf)
