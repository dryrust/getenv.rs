// This is free and unencumbered software released into the public domain.

//! This crate provides environment variable wrappers.
//!
//! ```edition2021
//! # use getenv::*;
//! ```

#![forbid(unsafe_code)]
#![allow(unused)]

mod features;
pub use features::*;

pub use secrecy;

mod vars;
pub use vars::*;

mod varsets;
pub use varsets::*;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
