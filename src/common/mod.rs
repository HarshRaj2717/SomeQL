//! Common module for shared constants, error types, and utilities.

pub(crate) mod constants;

mod datatypes;
pub(crate) use datatypes::DataTypeHolders;
pub(crate) use datatypes::DataTypeDefiners;

mod errors;
pub(crate) use errors::Error;
