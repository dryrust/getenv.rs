// This is free and unencumbered software released into the public domain.

use crate::var;

/// See: https://docs.brew.sh/Manpage#environment
pub fn all_proxy() -> Option<String> {
    var("all_proxy")
}

/// See: https://docs.brew.sh/Manpage#environment
pub fn ftp_proxy() -> Option<String> {
    var("ftp_proxy")
}

/// See: https://docs.brew.sh/Manpage#environment
pub fn http_proxy() -> Option<String> {
    var("http_proxy")
}

/// See: https://docs.brew.sh/Manpage#environment
pub fn https_proxy() -> Option<String> {
    var("https_proxy")
}

/// See: https://docs.brew.sh/Manpage#environment
pub fn no_proxy() -> Option<String> {
    var("no_proxy")
}
