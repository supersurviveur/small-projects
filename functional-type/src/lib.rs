#![no_std]

pub mod application;
pub mod bool;
pub mod integer;
pub mod invalid;
pub mod macros;
pub mod operators;
pub mod type_traits;

pub use functional_type_macros::{generate_signed_integer, generate_unsigned_integer};
