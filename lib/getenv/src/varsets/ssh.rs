// This is free and unencumbered software released into the public domain.

use crate::var;

/// See: https://man7.org/linux/man-pages/man1/ssh.1.html#ENVIRONMENT
pub fn ssh_auth_sock() -> Option<String> {
    var("SSH_AUTH_SOCK")
}
