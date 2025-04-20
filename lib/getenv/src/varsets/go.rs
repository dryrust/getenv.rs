// This is free and unencumbered software released into the public domain.

use crate::var;

/// See: https://pkg.go.dev/cmd/go#hdr-Environment_variables
pub fn goroot() -> Option<String> {
    var("GOROOT")
}
