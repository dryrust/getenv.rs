// This is free and unencumbered software released into the public domain.

use crate::var;

/// See: https://nodejs.org/en/learn/getting-started/nodejs-the-difference-between-development-and-production
pub fn node_env() -> Option<String> {
    var("NODE_ENV")
}
