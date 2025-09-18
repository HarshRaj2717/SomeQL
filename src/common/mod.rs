//! Common module for shared constants, error types, and utilities.

pub(crate) mod constants;

mod datatypes;
pub(crate) use datatypes::DataTypes;

mod errors;
pub(crate) use errors::Error;
