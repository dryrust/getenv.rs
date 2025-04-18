// This is free and unencumbered software released into the public domain.

//! This crate provides environment variable wrappers.
//!
//! ```edition2021
//! # use getenv::*;
//! ```

#![deny(unsafe_code)]
#![allow(unused)]

mod features;
pub use features::*;

#[cfg(feature = "std")]
pub mod vars;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
