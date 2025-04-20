// This is free and unencumbered software released into the public domain.

use crate::var;

/// See: https://en.wikipedia.org/wiki/Environment_variable#Windows
pub fn appdata() -> Option<String> {
    var("APPDATA")
}

/// See: https://www.nextofwindows.com/what-is-pathext-environment-variable-in-windows
/// See: https://en.wikipedia.org/wiki/Environment_variable#Windows
pub fn pathext() -> Option<String> {
    var("PATHEXT")
}

/// See: https://en.wikipedia.org/wiki/Environment_variable#Windows
pub fn username() -> Option<String> {
    var("USERNAME")
}

/// See: https://learn.microsoft.com/en-us/windows/deployment/usmt/usmt-recognized-environment-variables
pub fn windir() -> Option<String> {
    var("WINDIR")
}
