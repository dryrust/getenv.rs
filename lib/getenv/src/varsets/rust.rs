// This is free and unencumbered software released into the public domain.

use crate::var;

/// See: https://doc.rust-lang.org/cargo/reference/environment-variables.html
pub fn rustc() -> Option<String> {
    var("RUSTC")
}

/// See: https://doc.rust-lang.org/cargo/reference/environment-variables.html
pub fn rustdoc() -> Option<String> {
    var("RUSTDOC")
}

/// See: https://doc.rust-lang.org/cargo/reference/environment-variables.html
pub fn rustfmt() -> Option<String> {
    var("RUSTFMT")
}
