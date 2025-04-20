// This is free and unencumbered software released into the public domain.

use crate::var;

/// See: https://docs.brew.sh/Manpage#environment
pub fn homebrew_cache() -> Option<String> {
    var("HOMEBREW_CACHE")
}

/// See: https://docs.brew.sh/Manpage#environment
pub fn homebrew_cellar() -> Option<String> {
    var("HOMEBREW_CELLAR")
}

/// See: https://docs.brew.sh/Manpage#environment
pub fn homebrew_prefix() -> Option<String> {
    var("HOMEBREW_PREFIX")
}

/// See: https://docs.brew.sh/Manpage#environment
pub fn homebrew_repository() -> Option<String> {
    var("HOMEBREW_REPOSITORY")
}
