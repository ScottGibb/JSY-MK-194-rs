//! # Physical Units and Measurements
//!
//! This module re-exports types and units from the [`uom`] (Units of Measurement) crate
//! that are used throughout the JSY-MK-194 driver. All measurements use strongly-typed
//! units to prevent unit conversion errors.
//!
//! ## Available Units
//!
//! ### Electrical Measurements
//! - [`ElectricCurrent`] - Current measurements (amperes, milliamperes)
//! - [`ElectricPotential`] - Voltage measurements (volts, millivolts)  
//! - [`Power`] - Power measurements (watts)
//! - [`Frequency`] - Line frequency measurements (hertz)
//! - [`Energy`] - Energy measurements (watt-hours, kilowatt-hours)
//!
//! ## Unit Constants
//!
//! Each measurement type has associated unit constants for creating values:
//!
//! - **Current**: [`ampere`], [`milliampere`]
//! - **Voltage**: [`volt`], [`millivolt`]
//! - **Power**: [`watt`]
//! - **Frequency**: [`hertz`]
//! - **Energy**: [`watt_hour`], [`kilowatt_hour`]
//!
//! ## Usage Examples
//!
//! ```rust
//! use jsy_mk_194_rs::units::*;
//!
//! // Create measurements using unit constants
//! let voltage = ElectricPotential::new::<volt>(5.0);
//! let current = ElectricCurrent::new::<milliampere>(1500.0);
//! let frequency = Frequency::new::<hertz>(50.0);
//! let energy = Energy::new::<kilowatt_hour>(1.25);
//!
//! // Convert between units
//! let voltage_mv = voltage.get::<millivolt>(); // 5000.0
//! let current_a = current.get::<ampere>();     // 1.5
//! let line_hz = frequency.get::<hertz>();       // 50.0
//! let energy_wh = energy.get::<watt_hour>();    // 1250.0
//! ```
pub use uom::si::electric_current::ampere;
pub use uom::si::electric_current::milliampere;
pub use uom::si::electric_potential::millivolt;
pub use uom::si::electric_potential::volt;
pub use uom::si::energy::kilowatt_hour;
pub use uom::si::energy::watt_hour;
pub use uom::si::f32::ElectricCurrent;
pub use uom::si::f32::ElectricPotential;
pub use uom::si::f32::Energy;
pub use uom::si::f32::Frequency;
pub use uom::si::f32::Power;
pub use uom::si::frequency::hertz;
pub use uom::si::power::watt;
