// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
pub fn var(key: impl AsRef<std::ffi::OsStr>) -> Option<String> {
    use std::env::VarError::*;
    match std::env::var(key) {
        Err(NotPresent | NotUnicode(_)) => None,
        Ok(value) if value.trim().is_empty() => None,
        Ok(value) => Some(String::from(value.trim())), // TODO: remove the extra allocation
    }
}

#[cfg(not(feature = "std"))]
pub fn var(_key: impl AsRef<std::ffi::OsStr>) -> Option<String> {
    None // environment variables not supported
}
