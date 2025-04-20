// This is free and unencumbered software released into the public domain.

use crate::var;

/// See: https://docs.near.org/smart-contracts/testing/kurtosis-localnet
pub fn near_env() -> Option<String> {
    var("NEAR_ENV")
}
