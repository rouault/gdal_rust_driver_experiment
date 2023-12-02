#![crate_name = "gdal_rust_driver"]
#![crate_type = "lib"]

/// Base traits and utilities to define a GDAL Rust driver
pub mod bridge;

mod dummy_driver;
mod macros;
