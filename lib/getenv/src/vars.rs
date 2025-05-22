// This is free and unencumbered software released into the public domain.

use secrecy::SecretString;
use std::{ffi::OsStr, string::String};

#[cfg(feature = "std")]
pub fn var(key: impl AsRef<OsStr>) -> Option<String> {
    use std::env::VarError::*;
    match std::env::var(key) {
        Err(NotPresent | NotUnicode(_)) => None,
        Ok(value) if value.trim().is_empty() => None,
        Ok(value) => Some(String::from(value.trim())), // TODO: remove the extra allocation
    }
}

#[cfg(not(feature = "std"))]
pub fn var(_key: impl AsRef<OsStr>) -> Option<String> {
    None // environment variables not supported
}

#[cfg(feature = "std")]
pub fn var_secret(key: impl AsRef<OsStr>) -> Option<SecretString> {
    use std::env::VarError::*;
    match std::env::var(key) {
        Err(NotPresent | NotUnicode(_)) => None,
        Ok(value) if value.trim().is_empty() => None,
        Ok(value) => Some(SecretString::from(value.trim())),
    }
}

#[cfg(not(feature = "std"))]
pub fn var_secret(_key: impl AsRef<OsStr>) -> Option<SecretString> {
    None // environment variables not supported
}
