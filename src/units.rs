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
//!
//! ## Unit Constants
//!
//! Each measurement type has associated unit constants for creating values:
//!
//! - **Current**: [`ampere`], [`milliampere`]
//! - **Voltage**: [`volt`], [`millivolt`]
//! - **Power**: [`watt`]
//!
//! ## Usage Examples
//!
//! ```rust
//! use jsy_mk_194_rs::units::*;
//!
//! // Create measurements using unit constants
//! let voltage = ElectricPotential::new::<volt>(5.0);
//! let current = ElectricCurrent::new::<milliampere>(1500.0);
//! let temp = ThermodynamicTemperature::new::<degree_celsius>(25.0);
//!
//! // Convert between units
//! let voltage_mv = voltage.get::<millivolt>(); // 5000.0
//! let current_a = current.get::<ampere>();     // 1.5
//! ```
pub use uom::si::electric_current::ampere;
pub use uom::si::electric_current::milliampere;
pub use uom::si::electric_potential::millivolt;
pub use uom::si::electric_potential::volt;
pub use uom::si::f32::ElectricCurrent;
pub use uom::si::f32::ElectricPotential;
pub use uom::si::f32::Power;
pub use uom::si::power::watt;
